# Copilot Instructions for Daily App v0.4.0

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

This is a Tauri application for macOS that provides a menu bar note-taking interface for daily standups.

## Project Structure
- **Frontend**: React 18 with TypeScript (Vite build system)
- **Backend**: Rust with Tauri 2.x framework
- **UI**: Simple, clean macOS-style design with system tray integration
- **Version**: 0.4.0
- **Author**: Andre Bellmann (andre@andre-bellmann.de)

## Key Features
- macOS menu bar app (system tray only, no dock icon)
- Daily notes that are automatically saved per day
- Notes are stored locally in app data directory (`~/Library/Application Support/com.andre.daily.app/`)
- Auto-save functionality with 1-second debouncing
- Global keyboard shortcut (Cmd+Shift+N) for show/hide
- Quick Capture modal (Cmd+Shift+Space) for instant note-taking with 48h history
- Settings window with app info, shortcuts, and author details
- Smart tray-relative window positioning
- Custom title bar without native macOS controls
- Glassmorphism UI design with dark/light mode support
- Minimal, focused UI for quick note-taking
- Settings button in main window titlebar
- Version display in main window footer

## Technical Implementation
- **Plugins Used**:
  - `tauri-plugin-global-shortcut` for keyboard shortcuts
  - `tauri-plugin-positioner` for tray-relative positioning
  - `tauri-plugin-opener` for opening external links
- **Window Management**: Tray-only app with intelligent positioning
- **File Storage**: Plain text files with UTF-8 encoding
- **Date Formatting**: German locale (DD.MM.YYYY)
- **Multi-window Support**: Main window, Settings window, and Quick Capture modal
- **Build System**: Vite with multiple entry points

## Architecture
- **Main Window**: `index.html` → `src/main.tsx` → `App.tsx`
- **Settings Window**: `settings.html` → `src/settings-main.tsx` → `Settings.tsx`
- **Quick Capture Window**: `quick-capture.html` → `src/quick-capture-main.tsx` → `QuickCapture.tsx`
- **Tray Menu**: Show Notes, Settings, Quit
- **Global Shortcuts**: Cmd+Shift+N for toggle visibility, Cmd+Shift+Space for Quick Capture
- **Auto-positioning**: Fallback strategy for tray positioning

## Important Context
- The app should remain hidden by default and only show when tray icon is clicked
- Notes are saved as text files named by date (YYYY-MM-DD.txt)
- The app uses German locale for date formatting
- Focus on simplicity and daily standup preparation use case
- Settings accessible via tray context menu and titlebar button
- Global shortcuts: Cmd+Shift+N (main window), Cmd+Shift+Space (Quick Capture)
- ESC key closes window, Cmd+S manually saves
- Settings window shows version, shortcuts, features, and author info

## Code Style
- Use modern React hooks (useState, useEffect)
- Prefer async/await for Tauri invoke calls
- Keep Rust code simple and focused on file I/O
- Use CSS for styling without external dependencies
- Follow Tauri 2.x best practices for plugins and window management
