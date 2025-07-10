# Daily App v0.6.2

Eine minimalistische macOS-Menüleisten-App für tägliche Standup-Notizen, gebaut mit Tauri (React + Rust).

## Features ✨

✅ **Tray-Only App** - Lebt nur in der Menüleiste, kein Dock-Icon  
✅ **Tägliche Notizen** - Automatische SQLite-Datenbank mit intelligenter Organisation  
✅ **Auto-Save** - Notizen werden automatisch nach 1 Sekunde gespeichert  
✅ **Global Shortcuts** - `Cmd+Shift+N` öffnet/schließt das Fenster, `Cmd+Shift+Space` für Quick Capture  
✅ **Quick Capture** - Spotlight-ähnlicher Modal für schnelle Notizen mit 48h-Historie  
✅ **Smooth Animations** - Professionelle Ein-/Ausblend-Animationen für perfekte UX  
✅ **Manual Refresh** - Kleiner Reload-Button (↻) für manuelle Datenaktualisierung  
✅ **Real-time Sync** - Beide Fenster zeigen immer den aktuellen Stand der Notizen  
✅ **Settings Window** - Einstellungen mit App-Info, Shortcuts und Links  
✅ **Custom Titlebar** - Native macOS-Optik ohne Systemkontrollen  
✅ **Smart Positioning** - Fenster positioniert sich automatisch neben dem Tray-Icon  
✅ **Minimal UI Design** - Sauberes, fokussiertes Design ohne Ablenkungen  
✅ **Dark/Light Mode** - Folgt automatisch dem System-Theme  
✅ **Tastenkürzel** - ESC zum Schließen, Cmd+S zum Speichern  

## Technologie 🚀

- **Frontend**: React 18 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.x
- **Database**: SQLite mit tauri-plugin-sql
- **Plugins**: 
  - `tauri-plugin-global-shortcut` für Tastenkürzel
  - `tauri-plugin-positioner` für Tray-Positionierung
  - `tauri-plugin-opener` für externe Links
  - `tauri-plugin-sql` für SQLite-Datenbank
- **Styling**: Natives CSS mit minimalistischem macOS-Design und smooth Animationen

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
2. **Quick Capture**: `Cmd+Shift+Space` für schnelle Notizen mit smooth Animationen
3. **Schreiben**: Notizen werden automatisch in SQLite-Datenbank gespeichert
4. **Historie**: Quick Capture zeigt letzte 48 Stunden mit Manual-Refresh-Button
5. **Schließen**: ESC-Taste mit fade-out Animation
6. **Settings**: Über Tray-Rechtsklick → "Settings"
7. **Beenden**: Rechtsklick auf Tray → "Quit"

### Tastenkürzel
- `Cmd+Shift+N` - App öffnen/schließen (global)
- `Cmd+Shift+Space` - Quick Capture Modal öffnen (global)
- `ESC` - Fenster schließen
- `Cmd+S` - Notizen manuell speichern
- `Enter` - Notiz in Quick Capture speichern

### Dateispeicherung
- **Speicherort**: `~/Library/Application Support/com.andre.daily.app/`
- **Format**: SQLite-Datenbank (`daily-notes.db`)
- **Struktur**: Normalisierte Datenbankstruktur mit Timestamps
- **Auto-Save**: Nach 1 Sekunde Inaktivität
- **CRUD**: Vollständige Create, Read, Update, Delete Operationen

## Tastenkürzel ⌨️

- `Cmd+Shift+N` - App öffnen/schließen (global)
- `ESC` - Fenster schließen
- `Cmd+S` - Manuell speichern (optional)

## Datenbank 💾

Notizen werden gespeichert in SQLite-Datenbank unter:
```
~/Library/Application Support/com.andre.daily.app/daily-notes.db
```

**Datenbankschema:**
```sql
CREATE TABLE notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    is_quick_capture BOOLEAN DEFAULT FALSE
);
```

## Neue Features in v0.6.1 🆕

- **GitHub Actions CI/CD**: Vollautomatisierte Release-Pipeline mit DMG/App-Bundle-Erstellung
- **One-Click Releases**: Manueller Workflow-Trigger für sofortige Builds
- **Artifact Management**: 90-Tage Retention mit optimierten Upload-Einstellungen
- **Intel Mac Support**: Optionale x86_64-Builds für ältere Macs
- **Release Automation**: Automatische Release-Erstellung mit Download-Anleitungen

## Neue Features in v0.6.0 🔄

- **Smooth Animations**: Professionelle Ein-/Ausblend-Animationen mit CSS keyframes
- **Manual Refresh**: Kleiner Reload-Button (↻) in Quick Capture für manuelle Datenaktualisierung  
- **Enhanced UX**: Optimierte Benutzererfahrung mit visueller Feedback-Systemen
- **Performance**: Reduzierte automatische Refresh-Calls für bessere Batterielaufzeit
- **Dark Mode**: Vollständige Dark-Mode-Unterstützung für alle neuen UI-Elemente

## Technologie-Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri
- **UI**: Custom CSS mit macOS-Design
- **Build**: Tauri CLI

## Lizenz

MIT License
