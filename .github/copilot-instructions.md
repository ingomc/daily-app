# Copilot Instructions for Daily App

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

This is a Tauri application for macOS that provides a menu bar note-taking interface for daily standups.

## Project Structure
- **Frontend**: React with TypeScript (Vite build system)
- **Backend**: Rust with Tauri framework
- **UI**: Simple, clean macOS-style design with system tray integration

## Key Features
- macOS menu bar app (system tray)
- Daily notes that are automatically saved per day
- Notes are stored locally in app data directory
- Auto-save functionality with debouncing
- Minimal, focused UI for quick note-taking

## Important Context
- The app should remain hidden by default and only show when tray icon is clicked
- Notes are saved as text files named by date (YYYY-MM-DD.txt)
- The app uses German locale for date formatting
- Focus on simplicity and daily standup preparation use case

## Code Style
- Use modern React hooks (useState, useEffect)
- Prefer async/await for Tauri invoke calls
- Keep Rust code simple and focused on file I/O
- Use CSS for styling without external dependencies
