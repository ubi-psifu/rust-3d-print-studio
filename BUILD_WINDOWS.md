# Windows 11 x64

## Mitgelieferten Build starten

ZIP vollständig entpacken. Die portable Datei liegt unter **dist/rust-3d-print-studio.exe**. Der Installer **dist/Rust-3D-Print-Studio-Setup-x64.exe** installiert im Benutzerprofil und legt Einträge im Startmenü und in der Liste installierter Apps an. Beide Dateien sind unsigniert. Das Öffnen eines Beispiels gelingt über die Startmenü-Verknüpfung „Beispiel“ oder mit dem Projektpfad als erstem Argument.

## Auf Windows bauen

Rustup mit dem stabilen MSVC-x64-Toolchain, Visual Studio Build Tools mit „Desktopentwicklung mit C++“ und Windows SDK installieren. Für das Installationspaket zusätzlich NSIS 3 mit makensis im PATH bereitstellen. Im entpackten Projektordner PowerShell öffnen.

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
