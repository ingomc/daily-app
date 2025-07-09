# Daily App v0.5.0 Release Notes

**Released:** July 9, 2025  
**Author:** Andre Bellmann  

## 🚀 What's New in v0.5.0

### 🐛 Major Bug Fixes
- **Fixed Real-time Synchronization**: The biggest issue where Quick Capture modal wouldn't show the latest notes after reopening has been completely resolved
- **Perfect Data Consistency**: Both main window and Quick Capture now always display the same, up-to-date content
- **Window Focus Improvements**: Enhanced focus listeners ensure notes are reloaded when windows become visible

### 🎨 Design Improvements
- **Minimal UI Design**: Removed gradients and glassmorphism effects for a cleaner, more minimal macOS-native appearance
- **Reduced Visual Noise**: Streamlined interface focuses on the essential note-taking functionality
- **Better History Display**: Quick Capture history now only shows yesterday's notes, with today's note prominently displayed in the preview section

### ⚡ Performance & Technical Enhancements
- **Enhanced State Management**: Intelligent shared state with automatic initialization and date validation
- **Event-Driven Architecture**: Robust real-time updates across all windows using Tauri events
- **Optimized Loading**: Smart caching with automatic refresh when needed
- **Better Error Handling**: Improved fallback mechanisms for state loading and synchronization

## 🔧 Technical Details

### Backend Improvements
- Enhanced `get_current_note_from_state` command with automatic state refresh
- Improved window lifecycle management
- Better event emission for cross-window communication

### Frontend Enhancements
- Added window focus event listeners in both React components
- Improved autofocus behavior in Quick Capture
- Enhanced error handling and fallback states

## 📦 Installation

1. Download the latest `.dmg` file from the release
2. Open the DMG and drag "Daily App" to your Applications folder
3. Launch the app - it will appear in your menu bar
4. Use `Cmd+Shift+N` for the main window or `Cmd+Shift+Space` for Quick Capture

## 🎯 What This Release Fixes

**Before v0.5.0:**
- Submitting a note in Quick Capture would update the main window ✅
- But reopening Quick Capture wouldn't show the latest notes ❌

**After v0.5.0:**
- Submitting a note in Quick Capture updates the main window ✅
- Reopening Quick Capture shows all the latest notes ✅
- Both windows always stay perfectly synchronized ✅

## 🙏 Feedback

This release represents a significant stability improvement for the Daily App. If you encounter any issues or have suggestions, please reach out to andre@andre-bellmann.de.

---

**Download:** [Latest Release](https://github.com/andre-bellmann/daily-app/releases/latest)  
**System Requirements:** macOS 10.15+ (Catalina or newer)
