# Daily App v0.2.0

Eine minimalistische macOS-Menüleisten-App für tägliche Standup-Notizen, gebaut mit Tauri (React + Rust).

## Features ✨

✅ **Tray-Only App** - Lebt nur in der Menüleiste, kein Dock-Icon  
✅ **Tägliche Notizen** - Automatische Datei-Organisation nach Datum (YYYY-MM-DD.txt)  
✅ **Auto-Save** - Notizen werden automatisch nach 1 Sekunde gespeichert  
✅ **Global Shortcut** - `Cmd+Shift+N` öffnet/schließt das Fenster von überall  
✅ **Settings Window** - Einstellungen mit App-Info, Shortcuts und Links  
✅ **Custom Titlebar** - Native macOS-Optik ohne Systemkontrollen  
✅ **Smart Positioning** - Fenster positioniert sich automatisch neben dem Tray-Icon  
✅ **Glasmorphismus UI** - Modernes, transparentes Design mit macOS-Integration  
✅ **Dark/Light Mode** - Folgt automatisch dem System-Theme  
✅ **Tastenkürzel** - ESC zum Schließen, Cmd+S zum Speichern  

## Technologie 🚀

- **Frontend**: React 18 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.x
- **Plugins**: 
  - `tauri-plugin-global-shortcut` für Tastenkürzel
  - `tauri-plugin-positioner` für Tray-Positionierung
  - `tauri-plugin-opener` für externe Links
- **Styling**: Natives CSS mit macOS-Glasmorphismus

## Installation 🛠️

### Voraussetzungen
- macOS 12.0 oder höher
- Node.js 18+ (für Entwicklung)
- Rust 1.70+ (für Entwicklung)

### Entwicklung

```bash
# Repository klonen
git clone <repository-url>
cd daily-app

# Abhängigkeiten installieren
npm install

# App im Entwicklungsmodus starten
npm run tauri dev

# Produktions-Build erstellen
npm run tauri build
```

## Nutzung 📝

### Grundfunktionen
1. **Öffnen**: Klick auf Tray-Icon oder `Cmd+Shift+N`
2. **Schreiben**: Notizen werden automatisch gespeichert
3. **Schließen**: ESC-Taste oder Klick auf ×-Button
4. **Settings**: Über Tray-Rechtsklick → "Settings"
5. **Beenden**: Rechtsklick auf Tray → "Quit"

### Tastenkürzel
- `Cmd+Shift+N` - App öffnen/schließen (global)
- `ESC` - Fenster schließen
- `Cmd+S` - Notizen manuell speichern

### Dateispeicherung
- **Speicherort**: `~/Library/Application Support/com.andre.daily.app/`
- **Format**: Textdateien mit Namen `YYYY-MM-DD.txt`
- **Encoding**: UTF-8
- **Auto-Save**: Nach 1 Sekunde Inaktivität

## Tastenkürzel ⌨️

- `Cmd+Shift+N` - App öffnen/schließen (global)
- `ESC` - Fenster schließen
- `Cmd+S` - Manuell speichern (optional)

## Dateispeicher 💾

Notizen werden gespeichert unter:
```
~/Library/Application Support/com.tauri.daily-app/daily-notes/
```

Format: `YYYY-MM-DD.txt` (z.B. `2025-07-09.txt`)
```
~/Library/Application Support/com.andre.daily.app/daily-notes/
```

Jede Notiz wird als Textdatei im Format `YYYY-MM-DD.txt` gespeichert.

## Technologie-Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri
- **UI**: Custom CSS mit macOS-Design
- **Build**: Tauri CLI

## Lizenz

MIT License
