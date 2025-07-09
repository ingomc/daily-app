import { useState, useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import Database from "@tauri-apps/plugin-sql";
import "./App.css";

interface NoteEntry {
  id: number;
  content: string;
  created_at: string;
  updated_at: string;
  is_quick_capture: boolean;
}

function App() {
  const [allNotes, setAllNotes] = useState<NoteEntry[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [editingId, setEditingId] = useState<number | null>(null);
  const [editingContent, setEditingContent] = useState("");
  const [isVisible, setIsVisible] = useState(false);

  // Get today's date in a readable format
  const today = new Date().toLocaleDateString('de-DE', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });

  useEffect(() => {
    loadAllNotes();
    
    // Listen for note updates from database changes
    const unlistenNoteUpdate = listen("note-updated", () => {
      console.log("Main App: Received note-updated event");
      loadAllNotes();
    });

    const unlistenNotesUpdate = listen("notes-updated", () => {
      console.log("Main App: Received notes-updated event");
      loadAllNotes();
    });

    const unlistenRefresh = listen("refresh-data", () => {
      console.log("Main App: Received refresh-data event");
      loadAllNotes();
    });
    
    // Listen for window focus to reload current note
    const window = getCurrentWindow();
    const unlistenFocus = window.listen("tauri://focus", () => {
      loadAllNotes();
    });
    
    // Trigger fade-in animation
    setTimeout(() => setIsVisible(true), 100);

    return () => {
      unlistenNoteUpdate.then(f => f());
      unlistenNotesUpdate.then(f => f());
      unlistenRefresh.then(f => f());
      unlistenFocus.then(f => f());
    };
  }, []);

  async function loadAllNotes() {
    try {
      setIsLoading(true);
      
      // Load database and get all notes
      const db = await Database.load("sqlite:daily-notes.db");
      
      const notes = await db.select<NoteEntry[]>(
        "SELECT id, content, created_at, updated_at, is_quick_capture FROM notes WHERE content IS NOT NULL AND TRIM(content) != '' ORDER BY created_at DESC",
        []
      );
      
      console.log("Main App: Loaded all notes count:", notes.length);
      setAllNotes(notes);
    } catch (error) {
      console.error("Failed to load all notes:", error);
      setAllNotes([]);
    } finally {
      setIsLoading(false);
    }
  }

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // ESC to hide window
      if (event.key === 'Escape') {
        getCurrentWindow().hide();
      }
      // Cmd+R / Ctrl+R to refresh notes
      if ((event.metaKey || event.ctrlKey) && event.key === 'r') {
        event.preventDefault();
        loadAllNotes();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  async function openSettings() {
    try {
      // Create or show settings window
      const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow');
      
      const existingWindow = await WebviewWindow.getByLabel('settings');
      if (!existingWindow) {
        const newWindow = new WebviewWindow('settings', {
          url: 'settings.html',
          title: 'Einstellungen - Daily App',
          width: 400,
          height: 500,
          resizable: false,
          maximizable: false,
          minimizable: false,
          center: true,
        });
        await newWindow.show();
        await newWindow.setFocus();
      } else {
        await existingWindow.show();
        await existingWindow.setFocus();
      }
    } catch (error) {
      console.error("Failed to open settings:", error);
    }
  }

  async function handleTitleBarMouseDown(e: React.MouseEvent) {
    if (e.buttons === 1) { // Left mouse button
      if (e.detail === 2) {
        // Double click - toggle maximize (not needed for our use case, but good to have)
        getCurrentWindow().toggleMaximize();
      } else {
        // Single click - start dragging
        getCurrentWindow().startDragging();
      }
    }
  }

  // Load all notes when component mounts
  useEffect(() => {
    loadAllNotes();
  }, []);

  async function deleteNote(id: number) {
    try {
      const db = await Database.load("sqlite:daily-notes.db");
      await db.execute("DELETE FROM notes WHERE id = $1", [id]);
      console.log("Note deleted successfully:", id);
      
      // Reload notes to update the list
      await loadAllNotes();
    } catch (error) {
      console.error("Failed to delete note:", error);
    }
  }

  async function startEdit(noteEntry: NoteEntry) {
    setEditingId(noteEntry.id);
    setEditingContent(noteEntry.content);
  }

  async function saveEdit() {
    if (editingId && editingContent.trim()) {
      try {
        const db = await Database.load("sqlite:daily-notes.db");
        const now = new Date().toISOString();
        
        await db.execute(
          "UPDATE notes SET content = $1, updated_at = $2 WHERE id = $3",
          [editingContent.trim(), now, editingId]
        );
        
        console.log("Note updated successfully:", editingId);
        
        // Reset editing state
        setEditingId(null);
        setEditingContent("");
        
        // Reload notes to update the list
        await loadAllNotes();
      } catch (error) {
        console.error("Failed to update note:", error);
      }
    }
  }

  function cancelEdit() {
    setEditingId(null);
    setEditingContent("");
  }

  if (isLoading) {
    return (
      <div className="container loading visible">
        <p>Lade Notizen...</p>
      </div>
    );
  }

  return (
    <div className={`container ${isVisible ? 'visible' : ''}`}>
      {/* Custom Titlebar */}
      <div className="titlebar" onMouseDown={handleTitleBarMouseDown}>
        <div className="titlebar-content">
          <button 
            className="titlebar-settings" 
            onClick={openSettings}
            type="button"
            title="Settings"
          >
            ⚙️
          </button>
          <span className="titlebar-title">Daily Notes</span>
        </div>
      </div>

      <div className="content">
        <div className="header">
          <p className="date">{today}</p>
        </div>
        
        <div className="note-area">
          <div className="section-title">
            Alle Notizen
          </div>
          <div className="notes-list">
            {isLoading ? (
              <div className="notes-list-loading">Laden...</div>
            ) : allNotes.length === 0 ? (
              <div className="notes-list-empty">Noch keine Notizen vorhanden</div>
            ) : (
              allNotes.map((noteEntry) => (
                <div key={noteEntry.id} className="note-entry">
                  {editingId === noteEntry.id ? (
                    // Edit mode
                    <>
                      <div className="note-header">
                        <span className="note-date">
                          {new Date(noteEntry.created_at).toLocaleDateString('de-DE', {
                            day: '2-digit',
                            month: '2-digit'
                          })}
                        </span>
                        <span className="note-time">
                          {new Date(noteEntry.created_at).toLocaleTimeString('de-DE', {
                            hour: '2-digit',
                            minute: '2-digit'
                          })}
                        </span>
                      </div>
                      <input
                        className="note-edit-input"
                        value={editingContent}
                        onChange={(e) => setEditingContent(e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') {
                            saveEdit();
                          } else if (e.key === 'Escape') {
                            cancelEdit();
                          }
                        }}
                        autoFocus
                      />
                      <div className="note-actions">
                        <button className="note-action-btn save" onClick={saveEdit}>✓</button>
                        <button className="note-action-btn cancel" onClick={cancelEdit}>✕</button>
                      </div>
                    </>
                  ) : (
                    // View mode
                    <>
                      <div className="note-header">
                        <span className="note-date">
                          {new Date(noteEntry.created_at).toLocaleDateString('de-DE', {
                            day: '2-digit',
                            month: '2-digit'
                          })}
                        </span>
                        <span className="note-time">
                          {new Date(noteEntry.created_at).toLocaleTimeString('de-DE', {
                            hour: '2-digit',
                            minute: '2-digit'
                          })}
                        </span>
                      </div>
                      <div className="note-content" title={noteEntry.content}>
                        {noteEntry.content}
                      </div>
                      <div className="note-actions">
                        <button className="note-action-btn edit" onClick={() => startEdit(noteEntry)}>✏️</button>
                        <button className="note-action-btn delete" onClick={() => deleteNote(noteEntry.id)}>🗑️</button>
                      </div>
                    </>
                  )}
                </div>
              ))
            )}
          </div>
        </div>
        
        <div className="footer">
          <small>ESC schließen • ⌘+R aktualisieren • ⌘+Shift+Space Quick Capture • v0.5.0</small>
        </div>
      </div>
    </div>
  );
}

export default App;
