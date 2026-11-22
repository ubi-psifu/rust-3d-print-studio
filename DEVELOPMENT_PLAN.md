# Entwicklungsplan
1. Grundsystem v0.1: Eigenständiges Rust-Fenster mit egui, GPU-Tiefenpuffer, kalibriertem Druckbett und bedienbarer Kamera. Prüfen durch Start und Ansichtswechsel.
2. Modellierung: Parametrische Körper in f64, Auswahl, numerische Bearbeitung und Historie. Geometrie und Rückgängig-Verhalten testen.
3. Lokaler Arbeitsablauf: Versioniertes Projekt, sicherer Dateiersatz und Netzexport. Maße und Anzahl der Flächen unabhängig prüfen.
4. CAD-Ausbau: Skizzen, Zwangsbedingungen und B-Rep-Kern evaluieren. STEP erst nach belastbarem Roundtrip freigeben.
5. Druckvorbereitung: Profilgrenzen, Sperrflächen und anschließend Netzanalyse mit Reparaturvorschau.
6. Slicer: Modellübergabe gegen Bambu Studio prüfen. Druckparameter bleiben bis zur bestätigten Übernahme Projektmetadaten.
7. Freigabe: Plattformtests, Regressionen, Installationspaket und reproduzierbarer Release. Jede Stufe erhält einen dokumentierten Prüfstand.
