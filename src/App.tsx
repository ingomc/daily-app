import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
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
  }, []);

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
            placeholder={`Was hast du heute gemacht?

• Daily Standup vorbereitet
• Feature XY implementiert
• Meeting mit Team ABC
• Code Review durchgeführt

ESC - Schließen • ⌘+S - Speichern • ⌘+Shift+Space - Quick Capture`}
            rows={15}
            autoFocus
          />
        </div>
        
        <div className="footer">
          <small>Auto-save • ESC schließen • ⌘+Shift+Space Quick Capture • v0.4.0</small>
        </div>
      </div>
    </div>
  );
}

export default App;
