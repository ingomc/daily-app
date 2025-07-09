import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
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
    loadRecentNotes();
    // Focus the input when the window opens
    setTimeout(() => {
      const inputElement = document.querySelector('input');
      if (inputElement) {
        inputElement.focus();
      }
    }, 100);
  }, []);

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
        closeWindow();
      } else if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        handleSubmit();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [input]);

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

        <div className="recent-notes">
          <div className="recent-notes-title">Letzte 48 Stunden</div>
          {isLoading ? (
            <div className="loading">Lade Notizen...</div>
          ) : (
            <div className="notes-list">
              {recentNotes.length === 0 ? (
                <div className="no-notes">Noch keine Notizen vorhanden</div>
              ) : (
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
            ⏎ Speichern • ⎋ Schließen
          </span>
        </div>
      </div>
    </div>
  );
}

export default QuickCapture;
