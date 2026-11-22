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
