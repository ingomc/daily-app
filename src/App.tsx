import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { register } from "@tauri-apps/plugin-global-shortcut";
import "./App.css";

function App() {
  const [note, setNote] = useState("");
  const [isLoading, setIsLoading] = useState(true);
  const [isVisible, setIsVisible] = useState(false);

  // Get today's date in a readable format
  const today = new Date().toLocaleDateString('de-DE', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });

  useEffect(() => {
    loadTodayNote();
    // Don't position immediately - let tray handle positioning
    // positionWindowToTray();
    // Trigger fade-in animation
    setTimeout(() => setIsVisible(true), 100);
    
    // Register global shortcut
    registerGlobalShortcut();
  }, []);

  async function registerGlobalShortcut() {
    try {
      await register('cmd+shift+n', async (event) => {
        if (event.state === 'Pressed') {
          await invoke("toggle_window_visibility");
        }
      });
      console.log('Global shortcut registered: Cmd+Shift+N');
    } catch (error) {
      console.warn('Failed to register global shortcut:', error);
    }
  }



  async function loadTodayNote() {
    try {
      setIsLoading(true);
      const todayNote = await invoke<string>("get_today_note");
      setNote(todayNote);
    } catch (error) {
      console.error("Failed to load today's note:", error);
    } finally {
      setIsLoading(false);
    }
  }

  async function saveNote() {
    try {
      await invoke("save_today_note", { content: note });
    } catch (error) {
      console.error("Failed to save note:", error);
    }
  }

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // ESC to hide window
      if (event.key === 'Escape') {
        getCurrentWindow().hide();
      }
      // Cmd+S / Ctrl+S to manually save (even though auto-save is enabled)
      if ((event.metaKey || event.ctrlKey) && event.key === 's') {
        event.preventDefault();
        saveNote();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [note]);

  async function closeWindow() {
    getCurrentWindow().hide();
  }

  async function openSettings() {
    try {
      await invoke("show_settings_window");
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

  // Auto-save when note changes (with debounce)
  useEffect(() => {
    if (!isLoading && note !== undefined) {
      const timeoutId = setTimeout(() => {
        saveNote();
      }, 1000);

      return () => clearTimeout(timeoutId);
    }
  }, [note, isLoading]);

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
          <button 
            className="titlebar-close" 
            onClick={closeWindow}
            type="button"
          >
            ×
          </button>
        </div>
      </div>

      <div className="content">
        <div className="header">
          <p className="date">{today}</p>
        </div>
        
        <div className="note-area">
          <textarea
            value={note}
            onChange={(e) => setNote(e.target.value)}
            placeholder={`Was hast du heute gemacht? (${today})

Beispiele:
• Daily Standup vorbereitet
• Feature XY implementiert
• Meeting mit Team ABC
• Code Review durchgeführt
• Bug in Komponente Y behoben
• Dokumentation aktualisiert

Tastenkürzel:
ESC - Fenster schließen
⌘+S - Speichern (automatisch)
⌘+Shift+N - Öffnen/Schließen (global)`}
            rows={15}
            autoFocus
          />
        </div>
        
        <div className="footer">
          <small>Automatisch gespeichert • ⌘+Shift+N zum Öffnen • v0.2.0</small>
        </div>
      </div>
    </div>
  );
}

export default App;
