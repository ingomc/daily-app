use chrono::Local;
use std::fs;
use std::path::PathBuf;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WebviewWindow,
};
use tauri_plugin_positioner::{WindowExt, Position};

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

    if note_file.exists() {
        fs::read_to_string(note_file).map_err(|e| e.to_string())
    } else {
        Ok(String::new())
    }
}

#[tauri::command]
fn save_today_note(content: String, app_handle: AppHandle) -> Result<(), String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let notes_dir = get_notes_dir(&app_handle)?;
    let note_file = notes_dir.join(format!("{}.txt", today));
    fs::write(note_file, content).map_err(|e| e.to_string())
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcuts(["cmd+shift+n"])
                .unwrap()
                .with_handler(|app, _shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        // Toggle window visibility
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
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
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
            save_today_note,
            position_window_to_tray,
            show_and_position_window,
            toggle_window_visibility,
            show_settings_window,
            open_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
