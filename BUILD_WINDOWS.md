# Windows 11 x64

Für 0.11.0 wurden Windows-x64-Programm und NSIS-Setup unter Linux erstellt. Die Laufzeitabnahme auf Windows steht aus.

Version 0.9 enthält die grafische Bemaßung und die Beispiellasche. Der Installer legt das Projekt unter `examples/Bemasste-Lasche.r3dp` ab.

Ab 0.8 ist das Handbuch in der EXE eingebettet. F1 funktioniert unabhängig vom Installationspfad und von einem Webbrowser.

Der x64-Build 0.7.0 enthält den neuen Skizzensolver und die Wiederherstellung im lokalen Benutzerprofil. Für den Start der mitgelieferten EXE sind weder Rust noch eine Internetverbindung erforderlich. Windows-Laufzeit und Installation auf einem Windows-Gerät stehen aus.

Für 0.6.0 wurden die x64-EXE und das NSIS-Paket unter Linux erstellt. Ein Lauf auf Windows und eine Installation auf einem Windows-Testgerät stehen aus.

Für CAD, Schichtplanung und SVG-Export genügt die portable Studio-EXE. Für eine druckerspezifische .gcode.3mf werden zusätzlich eine lokale Bambu-Studio-Installation und eine eigene ungeslicte P1S-Projektvorlage benötigt. Installer und Vorlage können per Datenträger auf den Offline-Rechner übertragen werden. Der Windows-Build aus Quellcode benötigt vorab die in Cargo.lock fixierten Abhängigkeiten, die Windows-Toolchain und das SDK.

Bambu Studio ist für die Funktionen **Modellieren**, **Schnittansicht** und **3MF speichern** optional. Für den direkten Start die offizielle Desktopsoftware separat installieren. Unter **Platte vorbereiten → Übergabe** kann der Pfad zur installierten EXE gesetzt werden.

## Mitgelieferten Build starten

ZIP vollständig entpacken. Die portable Datei liegt unter **dist/rust-3d-print-studio.exe**. Der Installer **dist/Rust-3D-Print-Studio-Setup-x64.exe** installiert im Benutzerprofil und legt Einträge im Startmenü und in der Liste installierter Apps an. Beide Dateien sind unsigniert. Das Öffnen eines Beispiels gelingt über die Startmenü-Verknüpfung „Beispiel“ (Wandhaken) oder mit dem Projektpfad als erstem Argument.

## Auf Windows bauen

Rustup mit dem stabilen MSVC-x64-Toolchain, Visual Studio Build Tools mit „Desktopentwicklung mit C++“ und Windows SDK installieren. Für das Installationspaket zusätzlich NSIS 3.11 mit makensis im PATH bereitstellen. Im entpackten Projektordner PowerShell öffnen.

```powershell
rustup toolchain install 1.88.0 --component clippy --component rustfmt
rustup override set 1.88.0
.\scripts\windows.ps1
# Optional den Installer mitbauen
.\scripts\windows.ps1 -Installer
```

Ein einfacher Anwendungsbuild benötigt nur `cargo build --workspace --release --locked`. Die EXE liegt dann in **target/release/**. Das Skript verwendet ein explizites x64-Ziel und kopiert das Ergebnis nach **dist/**.

## Herkunft der beigelegten EXE

Cross-Compilation unter Linux mit Rust 1.88.0, Target x86_64-pc-windows-gnu und MinGW-w64 GCC. Compiler, Linker und NSIS meldeten Erfolg. PE-Header und DLL-Importliste wurden geprüft. Die Laufzeitprüfung auf Windows 10/11 sowie Installation und Deinstallation auf einer echten Windows-Maschine stehen aus. macOS ist ebenfalls noch nicht abgenommen.
