import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import "./QuickCapture.css";

interface RecentNote {
  date: string;
  content: string;
  lines: string[];
}

function QuickCapture() {
  const [input, setInput] = useState("");
  const [recentNotes, setRecentNotes] = useState<RecentNote[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    // Load data immediately when window is created
    loadRecentNotes();
    
    // Listen for multiple refresh events from backend
    const unlistenRefresh = listen("refresh-data", () => {
      console.log("Quick Capture: Refresh event received");
      loadRecentNotes();
    });
    
    const unlistenForceRefresh = listen("force-refresh", () => {
      console.log("Quick Capture: Force refresh event received");
      loadRecentNotes();
    });
    
    // Listen for window focus/visibility to reload data
    const window = getCurrentWindow();
    
    const handleFocus = () => {
      console.log("Quick Capture: Window focused, reloading data...");
      loadRecentNotes();
    };
    
    const handleShow = () => {
      console.log("Quick Capture: Window shown, reloading data...");
      loadRecentNotes();
    };
    
    // Listen to multiple events to ensure data refresh
    const unlistenFocus = window.listen("tauri://focus", handleFocus);
    const unlistenShow = window.listen("tauri://window-shown", handleShow);
    
    // Also check every time window becomes visible
    const checkAndReload = () => {
      window.isVisible().then(visible => {
        if (visible) {
          console.log("Quick Capture: Window is visible, reloading data...");
          loadRecentNotes();
        }
      }).catch(() => {});
    };
    
    // Check visibility on interval as fallback
    const visibilityCheck = setInterval(checkAndReload, 500);
    
    // Focus the input when the window opens - more robust approach
    const focusInput = () => {
      const inputElement = document.querySelector('input') as HTMLInputElement;
      if (inputElement) {
        inputElement.focus();
        inputElement.select(); // Select any existing text
      }
    };
    
    // Try multiple times to ensure focus works
    const timeouts = [50, 150, 300];
    timeouts.forEach(delay => {
      setTimeout(focusInput, delay);
    });

    return () => {
      unlistenRefresh.then(f => f());
      unlistenForceRefresh.then(f => f());
      unlistenFocus.then(f => f());
      unlistenShow.then(f => f());
      clearInterval(visibilityCheck);
    };
  }, []);

  async function loadRecentNotes() {
    try {
      setIsLoading(true);
      console.log("Quick Capture: Loading recent notes...");
      const notes = await invoke<RecentNote[]>("get_recent_notes");
      console.log("Quick Capture: Loaded recent notes count:", notes.length);
      setRecentNotes(notes);
    } catch (error) {
      console.error("Failed to load recent notes:", error);
    } finally {
      setIsLoading(false);
    }
  }

  async function handleSubmit() {
    if (input.trim()) {
      try {
        await invoke("append_to_today_note", { content: input.trim() });
        
        // Reload recent notes to show the new entry
        await loadRecentNotes();
        
        // Clear input and close window
        setInput("");
        setTimeout(async () => {
          await closeWindow();
        }, 300); // Brief delay to see the new note appear
      } catch (error) {
        console.error("Failed to save note:", error);
      }
    }
  }

  async function closeWindow() {
    const window = getCurrentWindow();
    // Just hide the window instead of closing it to preserve transparency settings
    await window.hide();
    setInput(""); // Clear input for next time
  }

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        event.preventDefault();
        setInput(""); // Clear input first
        closeWindow();
      } else if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        handleSubmit();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [input]); // Add input dependency

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr);
    const today = new Date();
    const yesterday = new Date();
    yesterday.setDate(today.getDate() - 1);

    if (dateStr === today.toISOString().split('T')[0]) {
      return 'Heute';
    } else if (dateStr === yesterday.toISOString().split('T')[0]) {
      return 'Gestern';
    } else {
      return date.toLocaleDateString('de-DE', {
        weekday: 'short',
        month: 'short',
        day: 'numeric'
      });
    }
  };

  const truncateText = (text: string, maxLength: number = 60) => {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength) + '...';
  };

  const extractTimeFromLine = (line: string) => {
    // Match formats: [DD.MM HH:MM], [HH:MM], [DD.MM]
    const match = line.match(/^\[(\d{2}\.\d{2} \d{2}:\d{2}|\d{2}:\d{2}|\d{2}\.\d{2})\]\s*(.*)$/);
    if (match) {
      return { time: match[1], content: match[2] };
    }
    return { time: null, content: line };
  };

  return (
    <div className="quick-capture-container">
      <div className="quick-capture-modal">
        <div className="quick-capture-input">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder="Was machst du gerade?"
            autoFocus
          />
        </div>

        {/* All Notes from last 48h */}
        <div className="recent-notes">
          <div className="recent-notes-title">Letzte 48h</div>
          {isLoading ? (
            <div className="loading">Lade Notizen...</div>
          ) : (
            <div className="notes-list">
              {recentNotes.length === 0 ? (
                <div className="no-notes">Keine Notizen in den letzten 48h</div>
              ) : (
                // Show all notes from the last 48h - backend provides them sorted by date
                recentNotes.map((note) => (
                  <div key={note.date} className="note-group">
                    <div className="note-date">{formatDate(note.date)}</div>
                    <div className="note-lines">
                      {note.lines.length === 0 ? (
                        <div className="note-line empty">Keine Einträge</div>
                      ) : (
                        note.lines.slice(-10).map((line, index) => {
                          const { time, content } = extractTimeFromLine(line);
                          return (
                            <div key={index} className="note-line">
                              {time && <span className="note-time">{time}</span>}
                              <span className="note-content">
                                {truncateText(content)}
                              </span>
                            </div>
                          );
                        })
                      )}
                    </div>
                  </div>
                ))
              )}
            </div>
          )}
        </div>

        <div className="quick-capture-footer">
          <span className="footer-hint">
            ⏎ Speichern • ⎋ Schließen • Bearbeiten nur im Hauptfenster
          </span>
        </div>
      </div>
    </div>
  );
}

export default QuickCapture;
