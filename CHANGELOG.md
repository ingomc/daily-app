# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-07-09

### Added
- **Settings Window**: New settings dialog with app information, shortcuts, and author details
- **Tray Icon Positioning**: Intelligent window positioning relative to macOS menu bar tray icon
- **Global Shortcut Support**: Added Cmd+Shift+N shortcut to toggle app visibility
- **Auto-Save Functionality**: Notes are automatically saved with debouncing (1-second delay)
- **Custom Window Title Bar**: Removed native macOS window controls for cleaner appearance
- **Fade-in Animation**: Smooth appearance transition when window opens
- **Error Handling**: Robust fallback positioning and error handling for tray positioning
- **German Date Formatting**: Daily notes are formatted with German locale
- **ESC Key Support**: Press ESC to close the window
- **Cmd+S Manual Save**: Option to manually save notes with keyboard shortcut

### Technical Improvements
- **Tauri 2.x Integration**: Built with latest Tauri framework
- **React 18 + TypeScript**: Modern frontend stack with type safety
- **Plugin Architecture**: 
  - `tauri-plugin-global-shortcut` for keyboard shortcuts
  - `tauri-plugin-positioner` for tray-relative positioning
  - `tauri-plugin-opener` for external links
- **Rust Backend**: Efficient file I/O operations for note storage
- **Tray-only Application**: No dock icon, pure menu bar app experience
- **Window State Management**: Proper show/hide/focus handling

### Fixed
- **Tray Position Initialization**: Resolved "Tray position not set" errors
- **Window Positioning Fallbacks**: Multiple positioning strategies for robust placement
- **Feature Dependencies**: Proper tray-icon feature activation for positioning plugin

### App Details
- **Author**: Andre Bellmann
- **Website**: [andre-bellmann.de](https://andre-bellmann.de)
- **Storage Location**: `~/Library/Application Support/com.andre.daily.app/`
- **File Format**: Plain text files named by date (YYYY-MM-DD.txt)

## [0.1.0] - 2025-07-09

### Added
- Initial release
- Basic note-taking functionality
- Daily file storage system
- macOS menu bar integration
- React + Tauri foundation
