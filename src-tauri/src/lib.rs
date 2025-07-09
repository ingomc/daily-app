use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Window, WindowEvent
};
use tauri_plugin_positioner::{Position, WindowExt};
use tauri_plugin_sql::{Migration, MigrationKind};

// Simplified commands
#[tauri::command]
async fn get_today_note() -> Result<String, String> {
    Ok("".to_string())
}

#[tauri::command]
async fn save_today_note(_content: String) -> Result<(), String> {
    Ok(())
}

async fn create_tray_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_notes = MenuItemBuilder::new("Notizen anzeigen").id("show_notes").build(app)?;
    let settings = MenuItemBuilder::new("Einstellungen").id("settings").build(app)?;
    let quit = MenuItemBuilder::new("Beenden").id("quit").build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&show_notes, &settings, &quit])
        .build()?;

    // Tray icon creation with proper positioner integration
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|app, event| {
            // Handle tray events for positioner plugin
            tauri_plugin_positioner::on_tray_event(app.app_handle(), &event);
            
            // Handle click events
            if let TrayIconEvent::Click { button, button_state, .. } = event {
                if button == MouseButton::Left && button_state == MouseButtonState::Up {
                    if let Some(window) = app.app_handle().get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                            
                            // Safe positioning - center instead of tray center
                            let _ = window.move_window(Position::Center);
                        }
                    }
                }
            }
        })
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "show_notes" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    
                    // Safe positioning - center instead of tray center
                    let _ = window.move_window(Position::Center);
                }
            }
            "settings" => {
                if let Some(window) = app.get_webview_window("settings") {
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    let _ = tauri::WebviewWindowBuilder::new(
                        app,
                        "settings",
                        tauri::WebviewUrl::App("settings.html".into()),
                    )
                    .title("Einstellungen - Daily App")
                    .inner_size(400.0, 500.0)
                    .resizable(false)
                    .maximizable(false)
                    .minimizable(false)
                    .center()
                    .build();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}

fn handle_window_event(window: &Window<tauri::Wry>, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            window.hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_notes_table",
            sql: "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                is_quick_capture BOOLEAN NOT NULL DEFAULT 0
            );",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create_notes_index",
            sql: "CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes(created_at);",
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcuts(["CommandOrControl+Shift+N", "CommandOrControl+Shift+Space"])?
                .with_handler(|app, shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        if shortcut.matches(tauri_plugin_global_shortcut::Modifiers::CONTROL | tauri_plugin_global_shortcut::Modifiers::SHIFT, tauri_plugin_global_shortcut::Code::KeyN) || 
                           shortcut.matches(tauri_plugin_global_shortcut::Modifiers::SUPER | tauri_plugin_global_shortcut::Modifiers::SHIFT, tauri_plugin_global_shortcut::Code::KeyN) {
                            // Main window toggle
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                    // Safe positioning with fallback
                                    let _ = window.move_window(tauri_plugin_positioner::Position::Center);
                                }
                            }
                        } else if shortcut.matches(tauri_plugin_global_shortcut::Modifiers::CONTROL | tauri_plugin_global_shortcut::Modifiers::SHIFT, tauri_plugin_global_shortcut::Code::Space) || 
                                  shortcut.matches(tauri_plugin_global_shortcut::Modifiers::SUPER | tauri_plugin_global_shortcut::Modifiers::SHIFT, tauri_plugin_global_shortcut::Code::Space) {
                            // Quick capture toggle
                            if let Some(window) = app.get_webview_window("quick-capture") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                    let _ = window.move_window(tauri_plugin_positioner::Position::Center);
                                }
                            } else {
                                // Create the quick capture window if it doesn't exist
                                if let Ok(quick_capture_window) = tauri::WebviewWindowBuilder::new(app, "quick-capture", tauri::WebviewUrl::App("quick-capture.html".into()))
                                    .inner_size(400.0, 300.0)
                                    .center()
                                    .resizable(false)
                                    .minimizable(false)
                                    .maximizable(false)
                                    .decorations(false)
                                    .always_on_top(true)
                                    .build() {
                                    let _ = quick_capture_window.show();
                                    let _ = quick_capture_window.set_focus();
                                }
                            }
                        }
                    }
                })
                .build()
        )
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:daily-notes.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_today_note,
            save_today_note,
        ])
        .setup(|app| {
            // Create tray menu synchronously during setup
            let app_handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                if let Err(e) = create_tray_menu(&app_handle).await {
                    eprintln!("Failed to create tray menu: {}", e);
                }
            });

            Ok(())
        })
        .on_window_event(handle_window_event)
        .run(tauri::generate_context!())?;
    
    Ok(())
}
