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
  const [currentNote, setCurrentNote] = useState("");
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadRecentNotes();
    loadCurrentNote();
    
    // Listen for note updates from other windows
    const unlistenNoteUpdate = listen<string>("note-updated", (event) => {
      setCurrentNote(event.payload);
      loadRecentNotes(); // Refresh recent notes as well
    });

    // Listen for window focus to reload current note
    const window = getCurrentWindow();
    const unlistenFocus = window.listen("tauri://focus", () => {
      loadCurrentNote();
      loadRecentNotes();
    });

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
      unlistenNoteUpdate.then(f => f());
      unlistenFocus.then(f => f());
    };
  }, []);

  async function loadCurrentNote() {
    try {
      const todayNote = await invoke<string>("get_current_note_from_state");
      setCurrentNote(todayNote);
    } catch (error) {
      console.error("Failed to load today's note:", error);
      // Fallback to empty string if there's an error
      setCurrentNote("");
    }
  }

  async function loadRecentNotes() {
    try {
      setIsLoading(true);
      const notes = await invoke<RecentNote[]>("get_recent_notes");
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
        
        // Reload current note to reflect the changes immediately
        await loadCurrentNote();
        
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
    const match = line.match(/^\[(\d{2}:\d{2})\]\s*(.*)$/);
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

        {/* Today's Note Preview (Read-only) - Shows the same data as main window */}
        <div className="current-note-preview">
          <div className="current-note-title">Heutige Notizen (Read-Only)</div>
          <div className="current-note-content">
            {currentNote ? (
              currentNote.split('\n').map((line, index) => {
                const { time, content } = extractTimeFromLine(line);
                return (
                  <div key={index} className="note-line">
                    {time && <span className="note-time">{time}</span>}
                    <span className="note-content">
                      {content || line}
                    </span>
                  </div>
                );
              })
            ) : (
              <div className="note-line empty">Noch keine Notizen für heute</div>
            )}
          </div>
        </div>

        <div className="recent-notes">
          <div className="recent-notes-title">Gestern</div>
          {isLoading ? (
            <div className="loading">Lade Notizen...</div>
          ) : (
            <div className="notes-list">
              {recentNotes.length === 0 ? (
                <div className="no-notes">Keine gestrigen Notizen vorhanden</div>
              ) : (
                // Only show yesterday's notes (skip today which is index 0)
                recentNotes.slice(1).map((note) => (
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
