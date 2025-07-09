import { useState, useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import Database from "@tauri-apps/plugin-sql";
import "./QuickCapture.css";

interface NoteEntry {
  id: number;
  content: string;
  created_at: string;
  updated_at: string;
  is_quick_capture: boolean;
}

function QuickCapture() {
  const [input, setInput] = useState("");
  const [recentNotes, setRecentNotes] = useState<NoteEntry[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isClosing, setIsClosing] = useState(false);

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

    const unlistenNotesUpdated = listen("notes-updated", () => {
      console.log("Quick Capture: Notes updated event received");
      loadRecentNotes();
    });

    const unlistenNoteCreated = listen("note-created", () => {
      console.log("Quick Capture: Note created event received");
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
    
    // Focus the input when the window opens
    const focusInput = () => {
      const inputElement = document.querySelector('input') as HTMLInputElement;
      if (inputElement) {
        inputElement.focus();
        inputElement.select();
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
      unlistenNotesUpdated.then(f => f());
      unlistenNoteCreated.then(f => f());
      unlistenFocus.then(f => f());
      unlistenShow.then(f => f());
      clearInterval(visibilityCheck);
    };
  }, []);

  async function loadRecentNotes() {
    try {
      setIsLoading(true);
      console.log("Quick Capture: Loading recent notes...");
      
      const db = await Database.load("sqlite:daily-notes.db");
      
      // Get notes from last 48 hours
      const now = new Date();
      const fortyEightHoursAgo = new Date(now.getTime() - (48 * 60 * 60 * 1000));
      const cutoffTime = fortyEightHoursAgo.toISOString();
      
      const notes = await db.select<NoteEntry[]>(
        "SELECT id, content, created_at, updated_at, is_quick_capture FROM notes WHERE created_at >= $1 AND content IS NOT NULL AND TRIM(content) != '' ORDER BY created_at DESC LIMIT 50",
        [cutoffTime]
      );
      
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
        const db = await Database.load("sqlite:daily-notes.db");
        const now = new Date().toISOString();
        
        // Create new quick capture note
        await db.execute(
          "INSERT INTO notes (content, created_at, updated_at, is_quick_capture) VALUES ($1, $2, $3, $4)",
          [input.trim(), now, now, true]
        );
        
        // Reload recent notes to show the new entry
        await loadRecentNotes();
        
        // Clear input and close window
        setInput("");
        setTimeout(async () => {
          await closeWindow();
        }, 100); // Brief delay to see the new note appear
      } catch (error) {
        console.error("Failed to save note:", error);
      }
    }
  }

  async function closeWindow() {
    setIsClosing(true);
    
    // Wait for exit animation to complete
    setTimeout(async () => {
      const window = getCurrentWindow();
      // Just hide the window instead of closing it to preserve transparency settings
      await window.hide();
      setInput(""); // Clear input for next time
      setIsClosing(false); // Reset state for next opening
    }, 200); // Match fade-out animation duration
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
  }, [input]);

  const formatTimestamp = (dateString: string) => {
    try {
      const date = new Date(dateString);
      const now = new Date();
      const diffHours = (now.getTime() - date.getTime()) / (1000 * 60 * 60);
      
      if (diffHours < 24) {
        return date.toLocaleString('de-DE', {
          hour: '2-digit',
          minute: '2-digit',
        });
      } else {
        return date.toLocaleString('de-DE', {
          day: '2-digit',
          month: '2-digit',
          hour: '2-digit',
          minute: '2-digit',
        });
      }
    } catch (error) {
      return dateString;
    }
  };

  const truncateText = (text: string, maxLength: number = 60) => {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength) + '...';
  };

  return (
    <div className={`quick-capture-container ${isClosing ? 'fade-out' : ''}`}>
      <div className={`quick-capture-modal ${isClosing ? 'slide-out' : ''}`}>
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
                recentNotes.map((note) => (
                  <div key={note.id} className="note-line">
                    <span className="note-time">
                      {formatTimestamp(note.created_at)}
                    </span>
                    <span className="note-content">
                      {truncateText(note.content)}
                    </span>
                    {note.is_quick_capture && (
                      <span className="note-badge quick">Quick</span>
                    )}
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
