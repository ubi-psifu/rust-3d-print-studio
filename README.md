# Rust 3D Print Studio 0.5.0

**Offline arbeiten:** Unter **Platte vorbereiten → Schnittansicht → Eigene Rust-Schichtplanung** erzeugst du ohne Zusatzprogramm Konturen und Linienfüllung. **Alle Schichten speichern** exportiert eine SVG pro Schicht in ein ZIP. Ein fertiger geometrischer Schichtplan liegt in `examples/modelle/Wandhaken-Schichtplan.zip`. Für P1S-Werkzeugwege nutzt **Übergabe → Platte lokal slicen** eine auf deinem Rechner installierte Bambu-Studio-Engine mit einer eigenen P1S-Vorlage. Beide Arbeitsabläufe funktionieren ohne Internetkonto.

**Druckvorbereitung → Platte vorbereiten** öffnet den neuen Arbeitsablauf für die Übergabe an Bambu Studio. Mit einer eigenen P1S-Projektvorlage kannst du Filamente je Objekt und vier Prozesswerte übernehmen. Die Schnittansicht zeigt die Modellkonturen bei der gewählten Höhe. Den Funktionsvergleich mit SimplyPrint findest du in [docs/SLICER_INTEGRATION.md](docs/SLICER_INTEGRATION.md).

Deutschsprachige native CAD-Anwendung mit parametrischen Skizzen, Pfadextrusionen, Loft, Rotationskörpern, Netzbooleschen und direkter Freiformbearbeitung. Die Oberfläche orientiert sich an der bereitgestellten SVG-Vorlage. Im Ordner **dist/** liegen Linux-Build, portable Windows-x64-EXE und Windows-Installer.

## Die Druckplatte belegen

Unter **Auswahl → Draufsicht** die Teile per Maus verschieben. **Umschalt + Klick** oder ein Rahmen um mehrere Teile wählt eine Gruppe. **Strg+D** legt die Kopie auf einen freien Platz. Für eine Serie **Alle anordnen**, das gewünschte Teil markieren und **Mit Kopien füllen** wählen.

## Den Wandhaken öffnen

**Datei → Projekt öffnen** und den Pfad zu **examples/modelle/Wandhaken.r3dp** eingeben. Im Inspektor **Befestigungsplatte → Breite** von 32 auf 36 mm ändern und **Parameter übernehmen** wählen. Das Studio berechnet die Konstruktion neu. **Modell prüfen** prüft das daraus abgeleitete Netz. **3MF exportieren** schreibt eine Modelldatei für den Slicer.

Im selben Ordner liegen **Wandhaken.stl**, **Wandhaken.3mf** und **Wandhaken-Einzelfeatures.r3dp**. Das letzte Projekt enthält dieselbe Konstruktion als Baum aus frei bearbeitbaren Profilen, Pfad, Extrusionen und booleschen Operationen.

## Start

- **Windows 11 x64** – ZIP vollständig entpacken und `dist/rust-3d-print-studio.exe` starten. Alternativ installiert `dist/Rust-3D-Print-Studio-Setup-x64.exe` das Programm samt Beispiel in das Benutzerprofil.
- **Linux x86_64** – Im entpackten Projektordner `chmod +x dist/rust-3d-print-studio-linux-x64` und `./dist/rust-3d-print-studio-linux-x64 examples/modelle/Wandhaken.r3dp` ausführen. Der mitgelieferte Build wurde auf Debian 13 getestet und benötigt X11 und OpenGL.
- **Aus dem Quellcode** – Mit Rust 1.88.0 im Projektordner `cargo run --release --locked` aufrufen. Der erste Build lädt freie Bibliotheken. Danach funktioniert die Anwendung lokal. Windows-Bauanleitung in BUILD_WINDOWS.md.

## Erster Druckkörper

Links **Bohrung → Bohrquader erstellen** wählen. Rechts die Abmessungen und den Bohrungsdurchmesser eingeben und **Parameter übernehmen** wählen. **Datei → Speichern unter** erhält die Parameter in einer R3DP-Datei. **Datei → 3MF exportieren** schreibt das Dreiecksnetz in Millimetern. Die 3MF-Datei in Bambu Studio öffnen, dort Druckplatte, Material und Prozess auswählen.

## Projektunterlagen

| Datei | Inhalt |
|---|---|
| DEVELOPMENT_PLAN.md | Reihenfolge und Abnahmeziele der Entwicklungsphasen |
| DEVELOPMENT_STATUS.md | Fertige Funktionen, Grenzen und offene Arbeit |
| USER_MANUAL.md | Bedienung, Kamera, Modellierung und Export |
| ARCHITECTURE.md | Komponenten, Einheiten und Formate |
| BUILD_WINDOWS.md | Reproduzierbarer Build und Installer |
| CHANGELOG.md | Versionshistorie |
| docs/SLICER_INTEGRATION.md | Funktionsvergleich und technische Integrationswege für SimplyPrint und Bambu Studio |
| docs/ABNAHME.md | Ausgeführte Prüfungen und Wiederholung |
| docs/BIBLIOTHEKEN.md | Lizenz, Veröffentlichungsaktivität und Einsatzentscheidung |
| THIRD_PARTY_NOTICES.txt | Lizenzinformationen zu den Bibliotheken und Schriften |

Projektcode unter MIT. Produktnamen bezeichnen die unterstützte Hardware und Software. Dieses Projekt wird unabhängig entwickelt.
