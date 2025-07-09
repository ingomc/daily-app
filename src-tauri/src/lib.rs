use chrono::Local;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WebviewWindow, Emitter,
};
use tauri_plugin_positioner::{WindowExt, Position};

// Shared application state
#[derive(Default, Clone, serde::Serialize)]
struct AppState {
    current_note: String,
    current_date: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn get_notes_dir(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Could not get app data directory: {}", e))?;

    let notes_dir = app_data_dir.join("daily-notes");
    if !notes_dir.exists() {
        fs::create_dir_all(&notes_dir).map_err(|e| e.to_string())?;
    }

    Ok(notes_dir)
}

#[tauri::command]
fn get_today_note(app_handle: AppHandle) -> Result<String, String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let notes_dir = get_notes_dir(&app_handle)?;
    let note_file = notes_dir.join(format!("{}.txt", today));

    let content = if note_file.exists() {
        fs::read_to_string(note_file).map_err(|e| e.to_string())?
    } else {
        String::new()
    };

    // Update shared state
    if let Some(state) = app_handle.try_state::<Mutex<AppState>>() {
        if let Ok(mut state_guard) = state.lock() {
            state_guard.current_note = content.clone();
            state_guard.current_date = today;
        }
    }

    Ok(content)
}

#[tauri::command]
fn save_today_note(content: String, app_handle: AppHandle) -> Result<(), String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let notes_dir = get_notes_dir(&app_handle)?;
    let note_file = notes_dir.join(format!("{}.txt", today));
    fs::write(note_file, &content).map_err(|e| e.to_string())?;

    // Update shared state and notify all windows
    if let Some(state) = app_handle.try_state::<Mutex<AppState>>() {
        if let Ok(mut state_guard) = state.lock() {
            state_guard.current_note = content.clone();
            state_guard.current_date = today;
        }
    }

    // Emit event to all windows about the note update with the full content
    let _ = app_handle.emit("note-updated", &content);
    
    // Also emit to specific windows to ensure delivery
    let _ = app_handle.emit_to("main", "note-updated", &content);
    let _ = app_handle.emit_to("quick-capture", "note-updated", &content);

    Ok(())
}

#[tauri::command]
fn position_window_to_tray(window: WebviewWindow) -> Result<(), String> {
    // Just center the window - no tray positioning on initial load
    window.center().map_err(|e| e.to_string())
}

#[tauri::command]
fn show_and_position_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        // Just center and show - no tray positioning on global shortcut
        let _ = window.center();
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn show_settings_window(app_handle: AppHandle) -> Result<(), String> {
    // Check if settings window already exists
    if let Some(window) = app_handle.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Create new settings window
    use tauri::{WebviewWindowBuilder, WebviewUrl};
    
    WebviewWindowBuilder::new(
        &app_handle,
        "settings",
        WebviewUrl::App("settings.html".into())
    )
    .title("Settings")
    .inner_size(480.0, 600.0)
    .min_inner_size(480.0, 400.0)
    .max_inner_size(600.0, 800.0)
    .resizable(true)
    .fullscreen(false)
    .decorations(false)
    .shadow(true)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn toggle_window_visibility(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            window.hide().map_err(|e| e.to_string())?;
        } else {
            // Just center and show - no tray positioning on global shortcut  
            let _ = window.center();
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
async fn open_url(url: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err("URL opening not implemented for this platform".to_string())
    }
}

#[tauri::command]
fn append_to_today_note(content: String, app_handle: AppHandle) -> Result<(), String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let notes_dir = get_notes_dir(&app_handle)?;
    let note_file = notes_dir.join(format!("{}.txt", today));
    
    let existing_content = if note_file.exists() {
        fs::read_to_string(&note_file).map_err(|e| e.to_string())?
    } else {
        String::new()
    };
    
    let timestamp = Local::now().format("%H:%M").to_string();
    let new_line = format!("[{}] {}", timestamp, content);
    
    let updated_content = if existing_content.is_empty() {
        new_line
    } else {
        format!("{}\n{}", existing_content, new_line)
    };
    
    fs::write(note_file, &updated_content).map_err(|e| e.to_string())?;

    // Update shared state and notify all windows
    if let Some(state) = app_handle.try_state::<Mutex<AppState>>() {
        if let Ok(mut state_guard) = state.lock() {
            state_guard.current_note = updated_content.clone();
            state_guard.current_date = today;
        }
    }

    // Emit event to all windows about the note update with the full content
    let _ = app_handle.emit("note-updated", &updated_content);
    
    // Also emit to specific windows to ensure delivery
    let _ = app_handle.emit_to("main", "note-updated", &updated_content);
    let _ = app_handle.emit_to("quick-capture", "note-updated", &updated_content);

    Ok(())
}

#[tauri::command]
fn get_current_note_from_state(app_handle: AppHandle) -> Result<String, String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    
    if let Some(state) = app_handle.try_state::<Mutex<AppState>>() {
        if let Ok(mut state_guard) = state.lock() {
            // If state is empty or for a different date, load from file
            if state_guard.current_note.is_empty() || state_guard.current_date != today {
                match get_today_note(app_handle.clone()) {
                    Ok(note_content) => {
                        state_guard.current_note = note_content.clone();
                        state_guard.current_date = today;
                        return Ok(note_content);
                    },
                    Err(_) => {
                        // No note file exists yet, initialize with empty content
                        state_guard.current_note = String::new();
                        state_guard.current_date = today;
                        return Ok(String::new());
                    }
                }
            } else {
                return Ok(state_guard.current_note.clone());
            }
        }
    }
    
    // Fallback to loading from file if state is not available
    get_today_note(app_handle)
}

#[derive(serde::Serialize)]
struct RecentNote {
    date: String,
    content: String,
    lines: Vec<String>,
}

#[tauri::command]
fn get_recent_notes(app_handle: AppHandle) -> Result<Vec<RecentNote>, String> {
    let notes_dir = get_notes_dir(&app_handle)?;
    let mut recent_notes = Vec::new();
    
    // Get only yesterday's note (skip today)
    for i in 1..3 {  // Start from 1 to skip today, get 2 previous days
        let date = Local::now() - chrono::Duration::days(i);
        let date_str = date.format("%Y-%m-%d").to_string();
        let note_file = notes_dir.join(format!("{}.txt", date_str));
        
        if note_file.exists() {
            let content = fs::read_to_string(note_file).map_err(|e| e.to_string())?;
            let lines: Vec<String> = content
                .lines()
                .map(|line| line.to_string())
                .filter(|line| !line.trim().is_empty())
                .collect();
            
            recent_notes.push(RecentNote {
                date: date_str,
                content,
                lines,
            });
        }
    }
    
    Ok(recent_notes)
}

#[tauri::command]
fn show_quick_capture_window(app_handle: AppHandle) -> Result<(), String> {
    // Check if quick capture window already exists
    if let Some(window) = app_handle.get_webview_window("quick-capture") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // If the window doesn't exist, it means it wasn't created in the config
    // In Tauri 2.x, we need to create it manually if not in config
    use tauri::{WebviewWindowBuilder, WebviewUrl};
    
    WebviewWindowBuilder::new(
        &app_handle,
        "quick-capture",
        WebviewUrl::App("quick-capture.html".into())
    )
    .title("Quick Capture")
    .inner_size(680.0, 440.0)
    .min_inner_size(680.0, 340.0)
    .max_inner_size(800.0, 600.0)
    .resizable(false)
    .fullscreen(false)
    .decorations(false)
    .shadow(false)
    .always_on_top(true)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcuts(["cmd+shift+n", "cmd+shift+space"])
                .map_err(|e| {
                    eprintln!("Failed to register shortcuts: {}", e);
                    e
                })
                .unwrap()
                .with_handler(|app, shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        // Parse shortcut manually since ID matching is unreliable
                        let shortcut_str = format!("{:?}", shortcut).to_lowercase();
                        println!("Global shortcut pressed: {}", shortcut_str);
                        
                        if shortcut_str.contains("cmd+shift+n") || shortcut_str.contains("cmd") && shortcut_str.contains("shift") && shortcut_str.contains("n") {
                            println!("Handling Cmd+Shift+N");
                            // Toggle main window visibility
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    // Try tray positioning first, fallback to center
                                    match window.move_window(Position::TrayBottomRight) {
                                        Ok(_) => {},
                                        Err(_) => {
                                            let _ = window.center();
                                        }
                                    }
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        } else if shortcut_str.contains("cmd+shift+space") || shortcut_str.contains("space") {
                            println!("Handling Cmd+Shift+Space");
                            // Show quick capture window
                            let _ = show_quick_capture_window(app.clone());
                        }
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            println!("App setup complete with shortcuts initialized!");
            
            // Initialize shared state
            app.manage(Mutex::new(AppState::default()));
            
            // Create tray menu
            let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let show_item = MenuItemBuilder::with_id("show", "Show Notes").build(app)?;
            let settings_item = MenuItemBuilder::with_id("settings", "Settings").build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[&show_item, &settings_item, &quit_item])
                .build()?;

            // Create tray icon
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            // Try tray positioning first, fallback to center
                            match window.move_window(Position::TrayBottomRight) {
                                Ok(_) => {},
                                Err(_) => {
                                    let _ = window.center();
                                }
                            }
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "settings" => {
                        let _ = show_settings_window(app.clone());
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    // Update tray state for positioner plugin
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                    
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                // Try different tray positions for macOS
                                let positioned = match window.move_window(Position::TrayBottomRight) {
                                    Ok(_) => true,
                                    Err(_) => {
                                        match window.move_window(Position::TrayRight) {
                                            Ok(_) => true,
                                            Err(_) => {
                                                match window.move_window(Position::TrayCenter) {
                                                    Ok(_) => true,
                                                    Err(_) => false
                                                }
                                            }
                                        }
                                    }
                                };
                                
                                if !positioned {
                                    let _ = window.center();
                                }
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_today_note,
            get_current_note_from_state,
            save_today_note,
            position_window_to_tray,
            show_and_position_window,
            toggle_window_visibility,
            show_settings_window,
            open_url,
            append_to_today_note,
            get_recent_notes,
            show_quick_capture_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
