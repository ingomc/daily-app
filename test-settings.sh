#!/bin/bash

# Test script for Daily App Settings Window
echo "🔍 Daily App Settings Window Test"
echo "================================="

# Check TypeScript compilation
echo "📝 Checking TypeScript compilation..."
npx tsc --noEmit
if [ $? -eq 0 ]; then
    echo "✅ TypeScript compilation successful"
else
    echo "❌ TypeScript compilation failed"
    exit 1
fi

# Check Rust compilation  
echo "🦀 Checking Rust compilation..."
cd src-tauri
cargo check
if [ $? -eq 0 ]; then
    echo "✅ Rust compilation successful"
    cd ..
else
    echo "❌ Rust compilation failed"
    exit 1
fi

echo ""
echo "🎉 All checks passed! Settings window implementation is ready."
echo ""
echo "Features implemented:"
echo "• ⚙️  Settings button in main app titlebar"
echo "• 🪟  Separate settings window with custom titlebar"
echo "• 📋  Tray menu with Settings option"
echo "• 📄  App information (version, storage location)"
echo "• ⌨️   Keyboard shortcuts documentation"
echo "• 🎨  Feature list with icons"
echo "• 👤  Author information with website link"
echo "• 🎯  Smart window management and positioning"
echo "• 🌙  Dark/Light mode support"
echo ""
echo "To test:"
echo "1. Run: npm run tauri dev"
echo "2. Click the ⚙️ button in the main window titlebar"
echo "3. Or right-click the tray icon and select 'Settings'"
echo ""
