# Abnahmeprotokoll · 0.2.0 · 20. November 2026

## J-Wandhaken

| Parameter | Abnahmewert |
|---|---:|
| Platte | 32 × 78 × 6 mm |
| Eckenradius / Rundung am Plattenrand | 5 / 1,2 mm |
| Schenkel / Biegeradius | 20 / 13 mm |
| Ausgangsdurchmesser / Endskalierung | 11 mm / 0,64 |
| Spitzenlänge / Spitzenwinkel | 10 mm / 15° |
| Bohrungen | 2, Achsabstand 56 mm |
| Durchgangsbohrung inklusive Zugabe | Ø 4,7 mm |
| 90°-Senkung inklusive Zugabe | Ø 9,2 mm |
| Verstärkungsrippe | 4,2 mm breit |

Die Platte liegt mit ihrer Rückseite bei Z = 0. Der Haken entsteht durch einen Sweep mit variablem Querschnitt mit gerundeter Spitze. Die Rippe und der Arm sind mit der durchbohrten Platte vereinigt.

**Bambu Studio 02.09.01.70** hat beide exportierten Dateien mit --info geprüft. Für beide meldete es 32 × 78 × 42,889664 mm, 2860 Dreiecke, **manifold = yes** und **number_of_parts = 1**. Die Volumina beider Importwege stimmen bis auf die Float-Rundung überein. Der CLI-Bericht gibt für 3MF lokale, zentrierte Koordinaten aus. Es wurde kein Druckauftrag ausgelöst.

Zusätzlich prüft ein Rust-Test die tatsächlich exportierte binäre STL nach dem Wiedereinlesen auf offene oder falsch orientierte Kanten, degenerierte Dreiecke und Selbstschnitt nicht benachbarter Flächen. Der 3MF-Test liest die Modellressource aus dem ZIP, prüft Einheit, Build-Objekt, Topologie und Selbstschnitt und vergleicht jedes Dreieck mit der STL im Rahmen der f32-Darstellung. Weitere Tests prüfen die beiden Senkringe, die Topologie eines Körpers mit zwei Durchgangslöchern, das Ändern der Plattenbreite, Undo/Redo und den Wiederaufbau aus R3DP.

## Anwendung und Builds

- Linux-Fenster mit 3D-Darstellung im virtuellen X11-Desktop geöffnet und kontrolliert. Plattenbreite im Inspektor von 32 auf 36 mm gezogen, übernommen und gespeichert. Die gespeicherte Datei enthielt 36 mm als Quellparameter und abgeleitete Breite. Nach Undo und erneutem Speichern betrug die Breite wieder 32 mm.
- cargo fmt --all --check, cargo check --workspace --all-targets --locked, cargo clippy --workspace --all-targets --locked -- -D warnings, cargo test --workspace --locked und cargo build --workspace --release --locked erfolgreich. Insgesamt 26 Tests.
- Zusätzlicher Release-Build für x86_64-pc-windows-gnu und NSIS-x64-Installer. Lauf auf einem Windows-Gerät und Installationsabnahme stehen aus.
- Reproduzieren mit cargo run --release --example wandhaken --locked im Projektordner. Dieser Befehl aktualisiert die vier benannten Beispieldateien in examples/modelle.

Die Hakenabnahme betrifft digitale Geometrie und Austauschformate. Material, Montagerichtung, Schrauben, Verankerung und Lastfall wurden mechanisch nicht geprüft.

# Abnahmeprotokoll · 0.1.0 · 20. November 2026

Ausgeführt auf Linux x86_64 mit Rust 1.88.0. Alle fünf Cargo-Prüfungen liefen mit Erfolg.

```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --release --locked
```

Neun Tests prüfen geschlossene und orientierte Körper einschließlich Bohrung, Transformation, Profilgrenzen, Sperrfläche, Undo/Redo-Verzweigung, Projekt-Roundtrip und sicheren Dateiersatz, beschädigte Eingaben, Exportstrukturen und Auswahl des vordersten Körpers bei beiden Projektionen.

Das Linux-Release öffnete das Beispiel Bohrquader in einem virtuellen X11-Fenster mit Mesa-Rendering. Oberfläche, Druckbett, Sperrfläche, Bohrung und Auswahlrahmen wurden im Fenster kontrolliert. Ein Klick auf Quader fügte 16 Dreiecke hinzu. Rechtsziehen und Mausrad änderten die Projektion im Bild.

## Bambu Studio

Bambu Studio 02.09.01.70 las jede der sechs mitgelieferten Austauschdateien mit **--info**. Für STL und 3MF kam jeweils derselbe Befund zurück.

| Beispiel | Größe X × Y × Z in mm | Dreiecke | Teile | Manifold |
|---|---|---:|---:|---|
| Wuerfel | 20 × 20 × 20 | 16 | 1 | yes |
| Bohrquader | 55 × 32 × 16 | 384 | 1 | yes |
| Profilgrenze | 20 × 20 × 251 | 16 | 1 | yes |

Das Volumen des Würfels beträgt laut Bambu Studio 8000 mm³, das des Bohrquaders 27358,043 mm³. Der Grenzfall demonstriert den Unterschied zwischen geschlossener Oberfläche und erlaubter Druckhöhe. Bei 3MF meldet --info zentrierte lokale Extremkoordinaten. Absolute Plattenplatzierung in der Bambu-Oberfläche, Slicing und physischer Druck wurden nicht abgenommen.

## Wiederholen

1. `cargo run --release --example abnahme --locked -- Abnahmemodelle` erzeugt die drei Modellvarianten in einem neuen Ordner und schützt bereits bestehende Verzeichnisse.
2. Ein R3DP-Beispiel im Studio öffnen, einen Parameter ändern, übernehmen und unter neuem Namen speichern. Datei erneut öffnen und Maß prüfen.
3. Bohrquader als STL und 3MF exportieren. Mit `bambu-studio --info Modell.3mf` und entsprechend für STL prüfen. In der grafischen Oberfläche zusätzlich Position und Druckorientierung kontrollieren.
4. Profilgrenze.r3dp öffnen. Es muss eine Bereichswarnung erscheinen. Höhe auf 250 mm setzen und übernehmen. Die Warnung verschwindet.

Das Sperrrechteck 0…18 mm in X und 0…28 mm in Y wurde mit dem P1S-0,4-mm-Profil (GM014) im getesteten Bambu-Studio-Paket verglichen. [Bambu-Studio-Release](https://github.com/bambulab/BambuStudio/releases/tag/v02.09.01.70)
