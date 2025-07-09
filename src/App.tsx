import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { moveWindow, Position } from "@tauri-apps/plugin-positioner";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "./App.css";

function App() {
  const [note, setNote] = useState("");
  const [isLoading, setIsLoading] = useState(true);

  // Get today's date in a readable format
  const today = new Date().toLocaleDateString('de-DE', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });

  useEffect(() => {
    loadTodayNote();
    // Position the window near the tray when it loads
    moveWindow(Position.TrayLeft).catch(console.error);
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
      <div className="container loading">
        <p>Lade Notizen...</p>
      </div>
    );
  }

  return (
    <div className="container">
      <div className="header">
        <h2>Daily Notes</h2>
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
⌘+S - Speichern (automatisch)`}
          rows={15}
          autoFocus
        />
      </div>
      
      <div className="footer">
        <small>Automatisch gespeichert</small>
      </div>
    </div>
  );
}

export default App;
