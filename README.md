# Vereinsverwaltung

Eine einfache Vereinsverwaltungssoftware für meinen Bruder.

Frontend in Svelte und Backend in Rust mit axum. Alle Daten inklusive Mitgliederdateien werden in einer sqlite Datenbank gespeichert

## Features

- Authentifizierung über Benutzername und Passwort
- Liste an Mitgliedern inklusive Bezahlstatus für das derzeitige Jahr
- Mitglieder erstellen mit dazugehörigen Informationen
- Mitglieder können Notizen beigefügt haben
- Für jedes Mitglied können dazugehörige Dateien hochgeladen werden

## Build

Das gesamte Frontend wird beim Buildprozess in die Serverbinary eingebettet. Heißt, dass das Frontend zuerst gebaut werden muss.

1. `cd frontend`
2. `npm run build`
3. `cd ..`
4. `cargo build --release`

## Konfiguration

Der Server wird über Umgebungsvariablen konfiguriert.

- `VEREINSVERWALTUNG_HOST` - Hostadresse des Servers. Wird für CORS benötigt
- `VEREINSVERWALTUNG_USER` - Benutzername des ersten Benutzers (nur relevant bei Datenbankinitialisierung)
- `VEREINSVERWALTUNG_PASSWORD` - Passwort des ersten Benutzers (nur relevant bei Datenbankinitialisierung)

Die Umgebungsvariablen können ebenfalls über eine `.env` Datei im aktuellen Arbeitsverzeichnis eingelesen werden. Die Datenbank wird ebenfalls dort gespeichert.

Das Arbeitsverzeichnis im Container liegt unter `/data`. Dort sollte also ein Mount eingerichtet werden, um die Daten zu persistieren sowie die Konfiguration zu übergeben.

## Notizen

Die einzelen Benutzer der Software müssen derzeit manuell in die Datenbank eingefügt werden. Der erste Benutzer kann jedoch über die Umgebungsvariablen erstellt werden.
