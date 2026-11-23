# Rust 3D Print Studio

Mit dem Windows-Installer ab 0.12.1 lassen sich R3DP-Projekte im Explorer über **Öffnen mit** an das Studio übergeben.

Eine deutschsprachige Desktop-Anwendung zum Konstruieren von 3D-Druckteilen und zum Anordnen auf der Druckplatte des Bambu Lab P1S. Modelle und Projekte bleiben auf dem eigenen Rechner. Für die Modellierung ist kein Konto erforderlich.

## Herunterladen und starten

Öffne den [aktuellen GitHub-Release](https://github.com/ubi-psifu/rust-3d-print-studio/releases/latest). Unter **Assets** enthält die Datei **Rust-3D-Print-Studio-‹Version›.zip** die Programme, die Anleitung und die Beispiele. Die beiden automatisch erzeugten **Source code**-Archive enthalten den Quellcode.

| System | Nach dem Entpacken |
| --- | --- |
| Windows x64 | Starte `dist/rust-3d-print-studio.exe` direkt oder installiere das Studio mit `dist/Rust-3D-Print-Studio-Setup-x64.exe`. |
| Linux x86-64 | Führe im entpackten Ordner `chmod +x dist/rust-3d-print-studio-linux-x64` und anschließend `./dist/rust-3d-print-studio-linux-x64` aus. Die Oberfläche benötigt X11 und OpenGL. |

Version 0.21.0 exportiert einzelne Kreiszylinder und gerade Rohre lokal als exakte STEP-Körper. Öffne **Referenzrohr.r3dp** und wähle **Datei → Zylinder oder Rohr als STEP exportieren …**. Bei Version 0.12.1 wurden Installation und Deinstallation auf Windows Server 2022 geprüft. Das portable Studio öffnete und speicherte ein Beispielprojekt mit Software-OpenGL. Die Prüfung auf Windows 11 mit Hardware-Grafiktreiber steht aus. Für macOS gibt es in diesem Release kein fertiges Paket. [Prüfstand](DEVELOPMENT_STATUS.md)

## Zwei Dinge zum Ausprobieren

- **Eine Druckplatte belegen.** Öffne über **Datei → Projekt öffnen** das Beispiel `examples/modelle/Wuerfel.r3dp`. Wähle **Auswahl → Draufsicht** und ziehe den Würfel mit der Maus. **Strg+D** platziert eine Kopie. **Umschalt + Klick** erweitert die Auswahl. Für eine Serie stehen unter den Plattenwerkzeugen Rechteckmuster und automatisches Füllen bereit.
- **Einen Kreisbogen ändern.** Öffne `examples/modelle/Bogenlasche.r3dp`. Wähle im Skizzeninspektor unter **Kontur und Hilfslinien** den Befehl **Radius bemaßen**. Ändere unter **Bemaßung und Bedingungen** den Radius von 12,5 auf 15 mm und wähle **Parameter übernehmen**. Der Bogen wird flacher, seine fixierten Endpunkte bleiben 20 mm auseinander.

**F1** öffnet die eingebettete deutsche Hilfe. Beispiele für einen parametrischen Wandhaken, eine seitlich offene Schale und ein Gehäuse mit Deckel stehen im [Modellordner](examples/modelle).

## Modellierung und Datenaustausch

| Aufgabe | Umfang in 0.21.0 |
| --- | --- |
| Konstruieren | Skizzen mit Abstands-, Winkel- und Radiusmaßen, Extrusion, Sweep, Loft, Rotation und boolesche Netzoperationen. Projekte im Format R3DP erhalten die Konstruktionsparameter. |
| Modelle austauschen | STL, OBJ, einfache 3MF-Modellcontainer und ASCII-PLY für Dreiecksnetze. SVG-Konturen lassen sich als Relief verwenden. STEP-Export für einzelne Kreiszylinder und gerade Rohre. Allgemeiner STEP-Import und weitere exakte Körper sind offen. |
| Druck vorbereiten | P1S-Profil, Bauraumprüfung und geometrische Schichtvorschau im Studio. Die eigene Schichtplanung kann Konturen und Fülllinien als SVG-Stapel exportieren. |
| Lokal slicen | Für druckerspezifische Werkzeugwege verwendet das Studio eine separat installierte Bambu-Studio-CLI und eine passende P1S-Projektvorlage. Ein einfacher Modell-3MF-Export überträgt Geometrie. |

## Quellcode und Dokumentation

Mit Rust 1.88 und Cargo im Projektordner `cargo run --release --locked` ausführen. Für den ersten Build müssen die Abhängigkeiten verfügbar sein. Der Projektcode steht unter der MIT-Lizenz. Das Projekt wird unabhängig von Bambu Lab entwickelt.

[Bedienungsanleitung](USER_MANUAL.md) · [Windows-Bauanleitung](BUILD_WINDOWS.md) · [Entwicklungsstand](DEVELOPMENT_STATUS.md) · [Entwicklungsplan](DEVELOPMENT_PLAN.md) · [Versionshistorie](CHANGELOG.md) · [Architektur](ARCHITECTURE.md) · [Slicer-Integration](docs/SLICER_INTEGRATION.md) · [Lizenz](LICENSE) · [Drittanbieter](THIRD_PARTY_NOTICES.txt)
