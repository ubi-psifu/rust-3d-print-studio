# Bibliotheksprüfung

Die Spalte Datum bezeichnet die Veröffentlichung der jüngsten stabilen, nicht zurückgezogenen Version im abgefragten Crates.io-Register. Sie ist ein Indiz für Aktivität. Das Cargo-Lockfile und die Builds prüfen die Kompatibilität der tatsächlich integrierten Auswahl. Für Kandidaten steht der Integrationstest aus.

| Paket und Dokumentation | Betrachtete Version | Lizenz laut Register | Letzte stabile Veröffentlichung | Entscheidung |
|---|---|---|---|---|
| [egui](https://docs.rs/egui/0.31.1) | 0.31.1 | MIT OR Apache-2.0 | 2026-10-15 | Oberfläche im Build |
| [eframe](https://docs.rs/eframe/0.37.0) | 0.37.0 | MIT OR Apache-2.0 | 2026-10-15 | Alternativer Fensterrahmen, würde zusätzlich zum gewählten Backend ein Fenster verwalten |
| [wgpu](https://docs.rs/wgpu/30.0.2) | 30.0.2 | MIT OR Apache-2.0 | 2026-11-12 | Alternative für spätere Compute- und Render-Pipelines |
| [glam](https://docs.rs/glam/0.27.0) | 0.27.0 | MIT OR Apache-2.0 | 2026-11-08 | f64 im Modell und f32 in der Ansicht |
| [nalgebra](https://docs.rs/nalgebra/0.35.0) | 0.35.0 | Apache-2.0 | 2026-05-24 | Für diesen Umfang deckt glam die lineare Algebra ab |
| [truck-modeling](https://docs.rs/truck-modeling/0.6.0) | 0.6.0 | Apache-2.0 | 2024-09-20 | B-Rep-Kandidat für Phase 4, STEP-Roundtrip separat prüfen |
| [parry3d](https://docs.rs/parry3d/0.31.1) | 0.31.1 | Apache-2.0 | 2026-09-18 | Kandidat für präzise Kollisionsprüfung |
| [stl_io](https://docs.rs/stl_io/0.11.0) | 0.11.0 | MIT | 2026-03-15 | Kandidat für STL-Import, binärer Export ist bereits direkt implementiert |
| [tobj](https://docs.rs/tobj/4.0.5) | 4.0.5 | MIT | 2026-08-02 | Kandidat für OBJ-Import |
| [serde](https://docs.rs/serde/1.0.228) | 1.0.228 | MIT OR Apache-2.0 | 2026-07-18 | Schemaableitung für Projekte und Profile |
| [serde_json](https://docs.rs/serde_json/1.0.145) | 1.0.145 | MIT OR Apache-2.0 | 2026-10-11 | Projektformat und Profilimport |
| [quick-xml](https://docs.rs/quick-xml/0.41.0) | 0.41.0 | MIT | 2026-08-22 | Kandidat für begrenzten XML-Import |
| [zip](https://docs.rs/zip/4.6.1) | 4.6.1 | MIT | 2026-11-20 | 3MF-Ausgabe mit unkomprimierten ZIP-Einträgen |
| [rfd](https://docs.rs/rfd/0.18.0) | 0.18.0 | MIT | 2026-11-07 | Kandidat für native Dateidialoge, derzeit deutsche Pfadeingabe |
| [rayon](https://docs.rs/rayon/1.12.0) | 1.12.0 | MIT OR Apache-2.0 | 2026-04-14 | Für spätere parallele Netzanalyse |
| [tracing](https://docs.rs/tracing/0.1.44) | 0.1.44 | MIT | 2025-12-18 | Für spätere strukturierte Diagnostik |
| [thiserror](https://docs.rs/thiserror/2.0.21) | 2.0.21 | MIT OR Apache-2.0 | 2026-09-23 | Bei wachsendem Fehlerkatalog vorgesehen |
| [egui-macroquad](https://docs.rs/egui-macroquad/0.17.3) | 0.17.3 | MIT/Apache-2.0 | 2025-05-21 | Integration von egui in die 3D-Ereignisschleife |
| [macroquad](https://docs.rs/macroquad/0.4.14) | 0.4.14 | MIT OR Apache-2.0 | 2026-07-30 | Fenster und GPU-Rendering mit Tiefentest |
| [tempfile](https://docs.rs/tempfile/3.23.0) | 3.23.0 | MIT OR Apache-2.0 | 2026-03-11 | Temporäre Datei für absturzarmen Dateiersatz |

## Ergänzung 0.2.0

boolmesh 0.2.2 ist mit Rust 1.88 als reine Rust-Bibliothek eingebunden und verwendet separat glam 0.32.1. Die Schnittstelle zum Studio übergibt f64-Punktkoordinaten und Dreiecksindizes. Der Kern nutzt die MPL-2.0. Die getesteten Operationen umfassen Vereinigung, Differenz und Schnittmenge. Der unveränderte Paketquellcode liegt unter third_party. [API-Dokumentation](https://docs.rs/boolmesh/0.2.2/boolmesh/)

## Ergänzung 0.4

ZIP 4.6.1 liest die Deflate-Kompression von Bambu-Projekten mit dem bereits im Abhängigkeitsbaum enthaltenen flate2. webbrowser 1.2.7 öffnet auf ausdrücklichen Klick die feste SimplyPrint-Adresse im Standardbrowser. Die offizielle Bambu-Studio-Anwendung wird separat installiert. Die Paketversionen stehen in Cargo.lock.
