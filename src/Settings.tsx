import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { openUrl } from "@tauri-apps/plugin-opener";
import "./Settings.css";

function Settings() {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    // Trigger fade-in animation
    setTimeout(() => setIsVisible(true), 100);
  }, []);

  const appVersion = "0.2.0";
  const authorName = "Andre Bellmann";
  const authorEmail = "andre@andre-bellmann.de";
  const websiteUrl = "https://andre-bellmann.de";

  async function closeWindow() {
    getCurrentWindow().hide();
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

  async function openWebsite() {
    try {
      console.log("Opening website:", websiteUrl);
      // Try the invoke command first as it's more reliable
      await invoke("open_url", { url: websiteUrl });
    } catch (error) {
      console.error("Failed to open website via invoke:", error);
      // Fallback: try the plugin method
      try {
        await openUrl(websiteUrl);
      } catch (fallbackError) {
        console.error("Plugin method also failed:", fallbackError);
        alert("Could not open website. Please visit https://andre-bellmann.de manually.");
      }
    }
  }

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // ESC to hide window
      if (event.key === 'Escape') {
        getCurrentWindow().hide();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  return (
    <div className={`settings-container ${isVisible ? 'visible' : ''}`}>
      {/* Custom Titlebar */}
      <div className="titlebar" onMouseDown={handleTitleBarMouseDown}>
        <div className="titlebar-content">
          <span className="titlebar-title">Settings</span>
          <button 
            className="titlebar-close" 
            onClick={closeWindow}
            type="button"
          >
            ×
          </button>
        </div>
      </div>

      <div className="settings-content">
        {/* App Info Section */}
        <section className="settings-section">
          <h2>App Information</h2>
          <div className="info-grid">
            <div className="info-item">
              <label>Version:</label>
              <span>{appVersion}</span>
            </div>
            <div className="info-item">
              <label>Storage Location:</label>
              <span>~/Library/Application Support/com.andre.daily.app/</span>
            </div>
          </div>
        </section>

        {/* Shortcuts Section */}
        <section className="settings-section">
          <h2>Keyboard Shortcuts</h2>
          <div className="shortcuts-list">
            <div className="shortcut-item">
              <span className="shortcut-key">⌘ + Shift + N</span>
              <span className="shortcut-desc">Show/Hide App (Global)</span>
            </div>
            <div className="shortcut-item">
              <span className="shortcut-key">⌘ + S</span>
              <span className="shortcut-desc">Save Note (Manual)</span>
            </div>
            <div className="shortcut-item">
              <span className="shortcut-key">ESC</span>
              <span className="shortcut-desc">Close Window</span>
            </div>
          </div>
        </section>

        {/* Features Section */}
        <section className="settings-section">
          <h2>Features</h2>
          <div className="features-list">
            <div className="feature-item">
              <span className="feature-icon">📝</span>
              <span>Daily notes automatically saved per day</span>
            </div>
            <div className="feature-item">
              <span className="feature-icon">💾</span>
              <span>Auto-save with 1-second debouncing</span>
            </div>
            <div className="feature-item">
              <span className="feature-icon">🎯</span>
              <span>Menu bar app with tray positioning</span>
            </div>
            <div className="feature-item">
              <span className="feature-icon">🌙</span>
              <span>Dark/Light mode support</span>
            </div>
          </div>
        </section>

        {/* Author Section */}
        <section className="settings-section">
          <h2>About</h2>
          <div className="author-info">
            <div className="author-details">
              <div className="author-name">{authorName}</div>
              <div className="author-email">{authorEmail}</div>
              <button 
                className="website-button"
                onClick={openWebsite}
                type="button"
                title="Click to open website in browser"
              >
                Visit Website →
              </button>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}

export default Settings;
