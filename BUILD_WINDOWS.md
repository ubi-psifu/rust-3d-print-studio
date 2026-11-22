# Rust 3D Print Studio unter Windows

Nach der Installation ab 0.12.1 lässt sich im Explorer unter **Öffnen mit** das Studio für R3DP-Projekte auswählen. Die portable EXE kann einen Projektpfad als erstes Argument entgegennehmen.

## Windows-x64-Paket verwenden

Unter **Assets** auf der [Release-Seite](https://github.com/ubi-psifu/rust-3d-print-studio/releases/latest) die ZIP-Datei mit dem Programmnamen herunterladen und vollständig entpacken.

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

## Prüfstand des Downloads 0.12.0

EXE und NSIS-Setup wurden unter Linux für Windows x64 gebaut. Die 62 automatisierten Tests liefen unter Linux. Ein praktischer Windows-Test von Programmstart, Installation und Deinstallation ist offen. Das Handbuch ist in der EXE eingebettet und lässt sich mit F1 offline öffnen.
