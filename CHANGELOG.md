# Änderungshistorie

## 0.2.0 · 20. November 2026

- Oberfläche an die SVG-Vorlage angepasst. Links liegen acht Werkzeuggruppen, rechts der Objektinspektor, die Modellprüfung und der direkte 3MF-Export.
- R3DP-v2-Featurebaum mit v1-Migration, Kontrollpunkteditor, Arbeitsebenen, gerichteten Bedingungen, Maßvariablen und arithmetischen Beziehungen.
- Sweep, Loft mit Zwischenprofilen und Führungsversatz, Extrusion, Rotation, Shell, ebener Schnitt, Vereinigung, Differenz, Schnittmenge und Bearbeitung konvexer Kanten.
- Gitterverformung, Netzglättung, Unterteilung, lokale Änderungen, Flächenverbund und vorbereitete Netzreparatur.
- Acht Haken-Voreinstellungen, frei auflösbare Konstruktion und Wandhaken-Abnahme in R3DP, STL und 3MF. Linux- und Windows-Builds aktualisiert.

## 0.1.0 · 20. November 2026

Erster auslieferbarer Stand des Grundsystems und des parametrischen Modell- und Exportablaufs.

- Native Rust-Oberfläche, GPU-3D-Ansicht, orthografische und perspektivische Kamera, sieben feste Ansichten, Zoom, Orbit, Verschieben und Fokussieren.
- Druckbett, X/Y/Z-Achsen, Sperrfläche und parametrierbares Raster. Helle und dunkle Oberfläche sowie wählbare Kürzel.
- Sieben Körpergeneratoren, Auswahl per Sichtstrahl oder Objektliste, editierbare Maße, Farbe, Position und Drehung. Kopie, Löschen, Zentrieren, Absenken und Undo/Redo.
- R3DP-v1-Projekte, konfigurierbares P1S-Profil sowie Export von binärem STL, OBJ und Standard-3MF.
- Abnahmemodelle, Rust-Tests, Windows-Build-Skript, portabler Build und NSIS-Paket.
