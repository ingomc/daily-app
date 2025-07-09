use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, WindowEvent
};
use tauri_plugin_positioner::{Position, WindowExt};
use tauri_plugin_sql::{Migration, MigrationKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteEntry {
    pub id: i64,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_quick_capture: bool,
}

// App state - simplified for SQLite
#[derive(Debug, Default)]
struct AppData {}

type AppState = Arc<Mutex<AppData>>;

fn format_timestamp(dt: &DateTime<Utc>) -> String {
    let local: DateTime<Local> = dt.with_timezone(&Local);
    local.format("%d.%m %H:%M").to_string()
}

#[tauri::command]
async fn create_note(
    app: AppHandle,
    request: CreateNoteRequest,
) -> Result<NoteEntry, String> {
    use tauri_plugin_sql::ManagerExt;
    
    let db = app.db("sqlite:daily-notes.db").map_err(|e| format!("Database error: {}", e))?;
    let now = Utc::now();
    let created_at = now.to_rfc3339();
    let updated_at = created_at.clone();

    let result = db
        .execute(
            "INSERT INTO notes (content, created_at, updated_at, is_quick_capture) VALUES (?1, ?2, ?3, ?4)",
            [&request.content, &created_at, &updated_at, &request.is_quick_capture.to_string()],
        )
        .await
        .map_err(|e| format!("Failed to create note: {}", e))?;

    let note = NoteEntry {
        id: result.last_insert_id(),
        content: request.content,
        created_at,
        updated_at,
        is_quick_capture: request.is_quick_capture,
    };

    // Emit update events to all windows
    let _ = app.emit("notes-updated", &note);
    let _ = app.emit("note-created", &note);

    Ok(note)
}

#[tauri::command]
async fn update_note(
    app: AppHandle,
    request: UpdateNoteRequest,
) -> Result<NoteEntry, String> {
    use tauri_plugin_sql::ManagerExt;
    
    let db = app.db("sqlite:daily-notes.db").map_err(|e| format!("Database error: {}", e))?;
    let now = Utc::now();
    let updated_at = now.to_rfc3339();

    db.execute(
        "UPDATE notes SET content = ?1, updated_at = ?2 WHERE id = ?3",
        [&request.content, &updated_at, &request.id.to_string()],
    )
    .await
    .map_err(|e| format!("Failed to update note: {}", e))?;

    // Get the updated note
    let rows = db
        .select("SELECT * FROM notes WHERE id = ?1", [&request.id.to_string()])
        .await
        .map_err(|e| format!("Failed to fetch updated note: {}", e))?;

    if let Some(row) = rows.first() {
        let note = NoteEntry {
            id: row.get::<i64>("id").unwrap_or(0),
            content: row.get::<String>("content").unwrap_or_default(),
            created_at: row.get::<String>("created_at").unwrap_or_default(),
            updated_at: row.get::<String>("updated_at").unwrap_or_default(),
            is_quick_capture: row.get::<bool>("is_quick_capture").unwrap_or(false),
        };

        // Emit update events
        let _ = app.emit("notes-updated", &note);
        let _ = app.emit("note-updated", &note);

        Ok(note)
    } else {
        Err("Note not found".to_string())
    }
}

#[tauri::command]
async fn delete_note(app: AppHandle, id: i64) -> Result<(), String> {
    use tauri_plugin_sql::ManagerExt;
    
    let db = app.db("sqlite:daily-notes.db").map_err(|e| format!("Database error: {}", e))?;

    db.execute("DELETE FROM notes WHERE id = ?1", [&id.to_string()])
        .await
        .map_err(|e| format!("Failed to delete note: {}", e))?;

    // Emit delete events
    let _ = app.emit("notes-updated", ());
    let _ = app.emit("note-deleted", id);

    Ok(())
}

#[tauri::command]
async fn get_notes(
    app: AppHandle,
    limit: Option<i64>,
    offset: Option<i64>,
    is_quick_capture: Option<bool>,
) -> Result<RecentNotesResponse, String> {
    use tauri_plugin_sql::ManagerExt;
    
    let db = app.db("sqlite:daily-notes.db").map_err(|e| format!("Database error: {}", e))?;
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let mut query = "SELECT * FROM notes".to_string();
    let mut params: Vec<String> = vec![];

    if let Some(quick_capture) = is_quick_capture {
        query.push_str(" WHERE is_quick_capture = ?");
        params.push(quick_capture.to_string());
    }

    query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
    params.push(limit.to_string());
    params.push(offset.to_string());

    let rows = db
        .select(&query, params)
        .await
        .map_err(|e| format!("Failed to fetch notes: {}", e))?;

    let notes: Vec<NoteEntry> = rows
        .into_iter()
        .map(|row| NoteEntry {
            id: row.get::<i64>("id").unwrap_or(0),
            content: row.get::<String>("content").unwrap_or_default(),
            created_at: row.get::<String>("created_at").unwrap_or_default(),
            updated_at: row.get::<String>("updated_at").unwrap_or_default(),
            is_quick_capture: row.get::<bool>("is_quick_capture").unwrap_or(false),
        })
        .collect();

    // Get total count
    let count_query = if is_quick_capture.is_some() {
        "SELECT COUNT(*) as count FROM notes WHERE is_quick_capture = ?".to_string()
    } else {
        "SELECT COUNT(*) as count FROM notes".to_string()
    };

    let count_params = if let Some(quick_capture) = is_quick_capture {
        vec![quick_capture.to_string()]
    } else {
        vec![]
    };

    let count_rows = db
        .select(&count_query, count_params)
        .await
        .map_err(|e| format!("Failed to count notes: {}", e))?;

    let total_count = count_rows
        .first()
        .and_then(|row| row.get::<i64>("count"))
        .unwrap_or(0);

    Ok(RecentNotesResponse { notes, total_count })
}

#[tauri::command]
async fn get_recent_notes(app: AppHandle) -> Result<Vec<NoteEntry>, String> {
    use tauri_plugin_sql::ManagerExt;
    
    let db = app.db("sqlite:daily-notes.db").map_err(|e| format!("Database error: {}", e))?;
    
    // Get notes from the last 48 hours
    let forty_eight_hours_ago = Utc::now() - chrono::Duration::hours(48);
    let cutoff_time = forty_eight_hours_ago.to_rfc3339();

    let rows = db
        .select(
            "SELECT * FROM notes WHERE created_at >= ?1 ORDER BY created_at DESC LIMIT 50",
            [&cutoff_time],
        )
        .await
        .map_err(|e| format!("Failed to fetch recent notes: {}", e))?;

    let notes: Vec<NoteEntry> = rows
        .into_iter()
        .map(|row| {
            NoteEntry {
                id: row.get::<i64>("id").unwrap_or(0),
                content: row.get::<String>("content").unwrap_or_default(),
                created_at: row.get::<String>("created_at").unwrap_or_default(),
                updated_at: row.get::<String>("updated_at").unwrap_or_default(),
                is_quick_capture: row.get::<bool>("is_quick_capture").unwrap_or(false),
            }
        })
        .collect();

    Ok(notes)
}

#[tauri::command]
async fn get_today_note(app: AppHandle) -> Result<String, String> {
    use tauri_plugin_sql::ManagerExt;
    
    let db = app.db("sqlite:daily-notes.db").map_err(|e| format!("Database error: {}", e))?;
    let today = Local::now().date_naive();
    let start_of_day = today.and_hms_opt(0, 0, 0).unwrap().and_utc().to_rfc3339();
    let end_of_day = today.and_hms_opt(23, 59, 59).unwrap().and_utc().to_rfc3339();

    let rows = db
        .select(
            "SELECT * FROM notes WHERE created_at >= ?1 AND created_at <= ?2 ORDER BY created_at ASC",
            [&start_of_day, &end_of_day],
        )
        .await
        .map_err(|e| format!("Failed to fetch today's notes: {}", e))?;

    let mut content = String::new();
    for row in rows {
        let created_at = row.get::<String>("created_at").unwrap_or_default();
        let note_content = row.get::<String>("content").unwrap_or_default();
        
        if let Ok(dt) = DateTime::parse_from_rfc3339(&created_at) {
            let timestamp = format_timestamp(&dt.with_timezone(&Utc));
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(&format!("[{}] {}", timestamp, note_content));
        }
    }

    Ok(content)
}

#[tauri::command]
async fn append_to_today_note(
    app: AppHandle,
    content: String,
) -> Result<String, String> {
    let request = CreateNoteRequest {
        content,
        is_quick_capture: false,
    };
    
    create_note(app.clone(), request).await?;
    
    // Return the formatted today's note
    get_today_note(app).await
}

#[tauri::command]
async fn save_today_note(
    app: AppHandle,
    content: String,
) -> Result<String, String> {
    let db = app.db();
    let today = Local::now().date_naive();
    let start_of_day = today.and_hms_opt(0, 0, 0).unwrap().and_utc().to_rfc3339();
    let end_of_day = today.and_hms_opt(23, 59, 59).unwrap().and_utc().to_rfc3339();

    // Clear today's notes first
    db.execute(
        "DELETE FROM notes WHERE created_at >= ?1 AND created_at <= ?2",
        [&start_of_day, &end_of_day],
    )
    .await
    .map_err(|e| format!("Failed to clear today's notes: {}", e))?;

    // Parse the content and create individual notes
    let now = Utc::now();
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        // Parse timestamp and content from line format: [DD.MM HH:MM] content
        if let Some(end_bracket) = line.find(']') {
            if line.starts_with('[') && end_bracket > 1 {
                let note_content = line[end_bracket + 1..].trim();
                if !note_content.is_empty() {
                    let request = CreateNoteRequest {
                        content: note_content.to_string(),
                        is_quick_capture: false,
                    };
                    create_note(app.clone(), request).await?;
                }
            }
        } else if !line.trim().is_empty() {
            // Line without timestamp, just save as is
            let request = CreateNoteRequest {
                content: line.trim().to_string(),
                is_quick_capture: false,
            };
            create_note(app.clone(), request).await?;
        }
    }

    // Emit update events
    let _ = app.emit("notes-updated", ());
    let _ = app.emit("note-updated", &content);

    Ok(content)
}

#[tauri::command]
async fn show_quick_capture_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("quick-capture") {
        let _ = window.show();
        let _ = window.set_focus();
        
        // Emit refresh events to ensure fresh data
        let _ = app.emit("refresh-data", ());
        let _ = app.emit("window-shown", "quick-capture");
        let _ = app.emit("force-refresh", ());
        
        Ok(())
    } else {
        Err("Quick capture window not found".to_string())
    }
}

#[tauri::command]
async fn position_window_to_tray(app: AppHandle, window_label: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(&window_label) {
        match window.move_window(Position::TrayLeft) {
            Ok(_) => Ok(()),
            Err(_) => {
                // Fallback positioning
                match window.move_window(Position::TopRight) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to position window: {}", e)),
                }
            }
        }
    } else {
        Err(format!("Window '{}' not found", window_label))
    }
}

#[tauri::command]
fn show_settings_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    use tauri::{WebviewWindowBuilder, WebviewUrl};
    
    WebviewWindowBuilder::new(
        &app,
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

// Legacy compatibility commands
#[tauri::command]
async fn get_current_note_from_state(app: AppHandle) -> Result<String, String> {
    get_today_note(app).await
}

fn handle_window_event(_window: &tauri::Window, event: &WindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        if let Some(main_window) = _window.app_handle().get_webview_window("main") {
            let _ = main_window.hide();
        }
        if let Some(quick_window) = _window.app_handle().get_webview_window("quick-capture") {
            let _ = quick_window.hide();
        }
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
                is_quick_capture BOOLEAN NOT NULL DEFAULT FALSE
            );",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "add_indices",
            sql: "
                CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes(created_at);
                CREATE INDEX IF NOT EXISTS idx_notes_is_quick_capture ON notes(is_quick_capture);
            ",
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:daily-notes.db", migrations)
                .build(),
        )
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
                        let shortcut_str = format!("{:?}", shortcut).to_lowercase();
                        
                        if shortcut_str.contains("cmd+shift+n") || shortcut_str.contains("cmd") && shortcut_str.contains("shift") && shortcut_str.contains("n") {
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    match window.move_window(Position::TrayBottomRight) {
                                        Ok(_) => {},
                                        Err(_) => {
                                            let _ = window.center();
                                        }
                                    }
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                    let _ = app.emit("refresh-data", ());
                                }
                            }
                        } else if shortcut_str.contains("cmd+shift+space") || shortcut_str.contains("space") {
                            let app_clone = app.clone();
                            tauri::async_runtime::spawn(async move {
                                let _ = show_quick_capture_window(app_clone).await;
                            });
                        }
                    }
                })
                .build(),
        )
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            create_note,
            update_note,
            delete_note,
            get_notes,
            get_recent_notes,
            get_today_note,
            append_to_today_note,
            save_today_note,
            get_current_note_from_state,
            show_quick_capture_window,
            position_window_to_tray,
            show_settings_window,
            open_url
        ])
        .setup(|app| {
            app.manage(std::sync::Mutex::new(State::default()));
            
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
                            match window.move_window(Position::TrayBottomRight) {
                                Ok(_) => {},
                                Err(_) => {
                                    let _ = window.center();
                                }
                            }
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = app.emit("refresh-data", ());
                        }
                    }
                    "settings" => {
                        let _ = show_settings_window(app.clone());
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
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
                                let _ = app.emit("refresh-data", ());
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
