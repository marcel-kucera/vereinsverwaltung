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

## Notizen

Die einzelen Benutzer der Software müssen derzeit manuell in die Datenbank eingefügt werden.
