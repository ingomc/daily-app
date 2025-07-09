use serde::{Deserialize, Serialize};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WindowEvent
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

    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "show_notes" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    
                    #[cfg(not(target_os = "linux"))]
                    {
                        let _ = window.move_window(Position::TrayCenter);
                    }
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
        .on_tray_icon_event(|tray, event| {
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
                        let _ = window.show();
                        let _ = window.set_focus();
                        
                        #[cfg(not(target_os = "linux"))]
                        {
                            let _ = window.move_window(Position::TrayCenter);
                        }
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn handle_window_event(event: &WindowEvent, window: &tauri::WebviewWindow) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            window.hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
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
            // Create tray menu
            tauri::async_runtime::spawn(async move {
                if let Err(e) = create_tray_menu(app.handle()).await {
                    eprintln!("Failed to create tray menu: {}", e);
                }
            });

            // Setup global shortcuts
            let app_handle = app.handle().clone();
            let global_shortcut = app.handle().global_shortcut();
            
            // Main window toggle shortcut
            global_shortcut.register("CommandOrControl+Shift+N", move || {
                if let Some(window) = app_handle.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                        
                        #[cfg(not(target_os = "linux"))]
                        {
                            let _ = window.move_window(Position::TrayCenter);
                        }
                    }
                }
            })?;

            // Quick capture shortcut
            let app_handle_quick = app.handle().clone();
            global_shortcut.register("CommandOrControl+Shift+Space", move || {
                if let Some(window) = app_handle_quick.get_webview_window("quick-capture") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    let _ = window.center();
                } else {
                    let _ = tauri::WebviewWindowBuilder::new(
                        &app_handle_quick,
                        "quick-capture",
                        tauri::WebviewUrl::App("quick-capture.html".into()),
                    )
                    .title("Quick Capture")
                    .inner_size(500.0, 400.0)
                    .resizable(false)
                    .maximizable(false)
                    .minimizable(false)
                    .decorations(false)
                    .transparent(true)
                    .always_on_top(true)
                    .center()
                    .build();
                }
            })?;

            Ok(())
        })
        .on_window_event(handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
