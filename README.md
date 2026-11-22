# Rust 3D Print Studio 0.1.0

Lokales, deutschsprachiges Konstruktionsprogramm für erste parametrische Druckteile. Das Grundsystem umfasst eine native 3D-Ansicht, sieben Körpergeneratoren, Projektdateien und Netzexport. Das Paket enthält den Quellstand, den getesteten Linux-Build, die Windows-x64-EXE und das Installationspaket unter **dist/**.

## Start

- **Windows 11 x64** – ZIP vollständig entpacken und `dist/rust-3d-print-studio.exe` starten. Alternativ installiert `dist/Rust-3D-Print-Studio-Setup-x64.exe` das Programm samt Beispiel in das Benutzerprofil.
- **Linux x86_64** – Im entpackten Projektordner `chmod +x dist/rust-3d-print-studio-linux-x64` und `./dist/rust-3d-print-studio-linux-x64 examples/modelle/Bohrquader.r3dp` ausführen. Der mitgelieferte Build wurde auf Debian 13 getestet und benötigt X11 und OpenGL.
- **Aus dem Quellcode** – Mit Rust 1.88.0 im Projektordner `cargo run --release --locked` aufrufen. Der erste Build lädt freie Bibliotheken. Danach funktioniert die Anwendung lokal. Windows-Bauanleitung in BUILD_WINDOWS.md.

## Erster Druckkörper

Links **Bohrquader** wählen. Rechts die Abmessungen und den Bohrungsdurchmesser eingeben und **Parameter übernehmen** wählen. **Datei → Speichern unter** erhält die Parameter in einer R3DP-Datei. **Datei → 3MF exportieren** schreibt das Dreiecksnetz in Millimetern. Die 3MF-Datei in Bambu Studio öffnen, dort Druckplatte, Material und Prozess auswählen.

## Projektunterlagen

| Datei | Inhalt |
|---|---|
| DEVELOPMENT_PLAN.md | Reihenfolge und Abnahmeziele der Entwicklungsphasen |
| DEVELOPMENT_STATUS.md | Fertige Funktionen, Grenzen und offene Arbeit |
| USER_MANUAL.md | Bedienung, Kamera, Modellierung und Export |
| ARCHITECTURE.md | Komponenten, Einheiten und Formate |
| BUILD_WINDOWS.md | Reproduzierbarer Build und Installer |
| CHANGELOG.md | Versionshistorie |
| docs/ABNAHME.md | Ausgeführte Prüfungen und Wiederholung |
| docs/BIBLIOTHEKEN.md | Lizenz, Veröffentlichungsaktivität und Einsatzentscheidung |
| THIRD_PARTY_NOTICES.txt | Lizenzinformationen zu den Bibliotheken und Schriften |

Projektcode unter MIT. Produktnamen bezeichnen die unterstützte Hardware und Software. Dieses Projekt wird unabhängig entwickelt.
