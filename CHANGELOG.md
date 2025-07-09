# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2025-07-10

### Added
- **Quick Capture Animations**: Professional fade-in/fade-out animations for Quick Capture modal
- **Manual Refresh Button**: Small, unobtrusive reload button (↻) in Quick Capture for manual data refresh
- **Animation System**: Sophisticated CSS animations with cubic-bezier timing functions
- **Loading States**: Visual feedback with spinning reload button during data refresh
- **Dark Mode Support**: Complete dark mode styling for new UI elements

### Improved
- **User Experience**: Smooth entrance and exit transitions with scale and slide effects
- **Performance**: Optimized data loading logic with reduced automatic refresh calls
- **Visual Polish**: Enhanced button interactions with subtle hover animations
- **Accessibility**: Improved button accessibility with proper hover states and tooltips

### Technical
- **CSS Architecture**: Modular CSS classes with clear animation keyframes
- **State Management**: Proper animation state management with React hooks
- **Animation Timing**: Optimized animations for 60fps smooth experience
- **Code Quality**: Well-structured animation logic with comprehensive documentation

### Fixed
- **Animation Cleanup**: Proper cleanup of animation timers and event listeners
- **Loading Performance**: Removed problematic automatic refresh attempts
- **UI Consistency**: Unified design language across all interactive elements

## [0.5.0] - 2025-07-09

### Fixed
- **Real-time Synchronization**: Fixed major bug where Quick Capture modal wouldn't show the latest notes after reopening
- **State Management**: Enhanced shared state management to ensure both windows always display the same, up-to-date content
- **Window Focus Events**: Added proper focus listeners to reload current note when windows become visible
- **Data Consistency**: Both main window and Quick Capture now use the same state-based data source

### Improved
- **Minimal UI Design**: Removed gradients and glassmorphism effects for a cleaner, more minimal appearance
- **Performance**: Optimized note loading with intelligent state caching and automatic refresh
- **User Experience**: Enhanced autofocus behavior in Quick Capture for better keyboard navigation
- **History Display**: Quick Capture history now only shows yesterday's notes, with today's note in the preview section

### Technical
- **Enhanced Backend**: Improved `get_current_note_from_state` command with automatic state initialization and date validation
- **Event-Driven Architecture**: Robust event emission and listening for real-time updates across all windows
- **Window Lifecycle**: Better handling of window show/hide cycles with proper state refresh
- **Error Handling**: Improved fallback mechanisms for state loading and synchronization

## [0.4.0] - 2025-07-09

### Added
- **Quick Capture Modal**: Spotlight-inspired instant note capture with global shortcut (Cmd+Shift+Space)
- **48-Hour History**: Quick Capture displays recent notes from the last 48 hours
- **Multi-Window Architecture**: Support for main window, settings, and Quick Capture modal
- **Enhanced Global Shortcuts**: Both Cmd+Shift+N (main window) and Cmd+Shift+Space (Quick Capture)
- **Timestamped Notes**: Quick Capture adds automatic timestamps to entries
- **Glassmorphism Design**: Beautiful frosted glass effects across all windows
- **Always On Top**: Quick Capture modal appears above all applications

### Improved
- **Backend Commands**: Extended Rust API with quick capture and recent notes functionality
- **Window Management**: Better handling of multiple windows and focus management
- **User Experience**: Instant input focus, ESC/Enter shortcuts, and smooth animations
- **Version Display**: Updated version information throughout the application

### Technical
- **New Entry Points**: Added quick-capture.html and supporting TypeScript files
- **Vite Configuration**: Multi-entry build system for all windows
- **Enhanced Styling**: Quick Capture specific CSS with macOS-native feel

## [0.3.0] - 2025-07-09

### Added
- **Settings Website Link**: Fixed and improved website opening functionality
- **Enhanced Error Handling**: Better fallback mechanisms for external links
- **Version Updates**: Consistent versioning across all components

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
