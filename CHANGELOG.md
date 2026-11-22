# 0.7.0 · 21. November 2026

- Bidirektionale Skizzenbedingungen für freie und fixierte Punkte, lokale U-/V-Richtung, gleiche Länge und Rechtwinkligkeit. Kompatibler Rechenweg für bestehende Skizzen.
- Bemaßte Beispielplatte mit parametrisierter Extrusion.
- Lokale Autosicherung im 30-Sekunden-Takt und Wiederherstellung als Projektkopie.
- ASCII-STL-Export und ASCII-PLY-Import und -Export. Sieben zusätzliche Tests.

# 0.6.0 · 21. November 2026

- Rechteck- und Kreismuster mit Vorschau, auslassbare Kopien, Ausrichten, Verteilen und wiederholte XY-Schritte.
- Netzimport für binäre und ASCII-STL, OBJ und einfache Modell-3MF.
- Schrift und geschlossene SVG-Konturen als Relief oder Gravur. Parametrisches Gehäuse mit passendem Deckel.
- Fünf neue Regressionstests für Layout, Dateiaustausch und Geometrie.

# Änderungshistorie

## 0.5.0 · Offline-Schichtplanung

Eigene Rust-Linienfüllung mit Löchern und wechselnder Richtung, vollständiger SVG-Schichtexport und lokaler CLI-Auftrag für P1S-Druckdateien. Die SimplyPrint-Verknüpfung in der Oberfläche entfällt auf Wunsch nach rein lokalem Betrieb.

## 0.4.0 · 20. November 2026

- Druckvorbereitung mit vier Reitern, horizontaler Schnittansicht, lokalen P1S-Vorlagen, Filamentzuordnung und vier Prozessparametern.
- Neuer Bambu-Projektexport und Start der separat installierten Desktopsoftware. Browserzugang zum SimplyPrint-Angebot.
- Vier zusätzliche Regressionstests und reproduzierbare Abnahme mit der offiziellen CLI.

## 0.3.0

- Direkte XY-Verschiebung auf der Greifhöhe des Mauszeigers, Rasterfang, Mehrfach- und Rahmenauswahl.
- Interne Zwischenablage, Gruppen duplizieren, Druckbett automatisch anordnen und freie Bereiche mit Kopien füllen.
- Drei zusätzliche Platzierungstests sowie grafischer Abnahmetest für Auswahl, Ziehen, Kopie und Undo.

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
