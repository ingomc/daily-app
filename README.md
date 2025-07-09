# Daily App

Eine macOS-Menüleisten-Anwendung für tägliche Notizen und Daily Standup Vorbereitung.

## Features

- **Menüleisten-Integration**: Lebt diskret in der macOS-Menüleiste
- **Tägliche Notizen**: Automatische Erstellung und Speicherung von Notizen pro Tag
- **Auto-Save**: Notizen werden automatisch gespeichert
- **Minimalistisches Design**: Fokus auf schnelle Notiz-Eingabe
- **Lokale Speicherung**: Alle Daten werden lokal gespeichert

## Installation

### Voraussetzungen
- macOS 12.0 oder höher
- Node.js (für Entwicklung)
- Rust (für Entwicklung)

### Entwicklung

```bash
# Dependencies installieren
npm install

# App im Entwicklungsmodus starten
npm run tauri dev

# App für Produktion bauen
npm run tauri build
```

## Verwendung

1. Nach dem Start erscheint ein Icon in der macOS-Menüleiste
2. Klick auf das Icon öffnet/schließt das Notizen-Fenster
3. Rechtsklick auf das Icon zeigt das Kontextmenü
4. Notizen werden automatisch gespeichert
5. Jeder Tag hat seine eigene Notiz-Datei

## Tastenkürzel

- **Linksklick auf Tray-Icon**: Fenster öffnen/schließen
- **Rechtsklick auf Tray-Icon**: Menü öffnen
- **Automatischer Focus**: Textarea wird automatisch fokussiert

## Datenverzeichnis

Die Notizen werden gespeichert unter:
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
