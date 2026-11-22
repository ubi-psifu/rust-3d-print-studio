# Entwicklungsplan

## Nächste Abnahmen

1. Nach der erfolgreichen Windows-Server-Abnahme die Windows-11-Bedienung mit Hardware-Grafiktreiber prüfen. Für kurze Tastenkombinationen gibt es seit 0.12.2 Regressionstests und eine Fensterabnahme mit der portablen EXE.
2. Die ebenen Öffnungsflächen aus 0.13.0 als nächsten Schritt direkt in der 3D-Ansicht auswählen. Für gekrümmte Flächen und exakte Körper einen B-Rep-Kern erproben. STEP-Import und -Export benötigen einen geprüften Roundtrip.
3. Bei der Druckvorbereitung größere Importmodelle, Mehrfilament-Slicing und die physische Passung der Beispielteile prüfen.

## Ursprüngliche Entwicklungsphasen

1. Grundsystem v0.1: Eigenständiges Rust-Fenster mit egui, GPU-Tiefenpuffer, kalibriertem Druckbett und bedienbarer Kamera. Prüfen durch Start und Ansichtswechsel.
2. Modellierung: Parametrische Körper in f64, Auswahl, numerische Bearbeitung und Historie. Geometrie und Rückgängig-Verhalten testen.
3. Lokaler Arbeitsablauf: Versioniertes Projekt, sicherer Dateiersatz und Netzexport. Maße und Anzahl der Flächen unabhängig prüfen.
4. CAD-Ausbau: Skizzen, Zwangsbedingungen und B-Rep-Kern evaluieren. STEP erst nach belastbarem Roundtrip freigeben.
5. Druckvorbereitung: Profilgrenzen, Sperrflächen und anschließend Netzanalyse mit Reparaturvorschau.
6. Slicer: Modellübergabe gegen Bambu Studio prüfen. Druckparameter bleiben bis zur bestätigten Übernahme Projektmetadaten.
7. Freigabe: Plattformtests, Regressionen, Installationspaket und reproduzierbarer Release. Jede Stufe erhält einen dokumentierten Prüfstand.

Die Umsetzung je Version steht in der [Versionshistorie](CHANGELOG.md), bekannte Grenzen im [Entwicklungsstand](DEVELOPMENT_STATUS.md).
