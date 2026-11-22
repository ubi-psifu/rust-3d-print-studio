# 0.15.0 · Kanten für Fasen und Rundungen anklicken

Der Kantenquader zeigt die direkte Bindung einer scharfen Kante in der 3D-Ansicht. Die markierte Kante lässt sich mit Anfangsmaß, Endmaß und Facettenzahl bearbeiten.

# 0.14.0 · Öffnungsfläche im Modell anklicken

Der Inspektor zeigt den Außenkörper einer Aushöhlung in einer eigenen Auswahlansicht. Ein Klick auf eine sichtbare Fläche bindet sie an den Entwurf. F1 erklärt die Auswahl offline.

# 0.13.0 · Schräge Öffnungsflächen

Ebene Flächen am Außenkörper lassen sich geometrisch als Öffnung einer Schale binden. Die Keilschale zeigt die Anwendung auf einer Schräge. Die F1-Hilfe findet CAD-Themen auch in Handbüchern mit Windows-Zeilenenden.

# 0.12.2 · Kurze Tastenkombinationen

Auswahl, Duplizieren, Rückgängig, Wiederholen und Speichern berücksichtigen die beim Tastendruck gehaltene Befehlstaste, auch wenn beide Tasten vor dem nächsten Bild losgelassen werden.

# 0.12.1 · Windows-Projekte öffnen

Der Installer registriert R3DP für „Öffnen mit“ und verwendet die Cargo-Paketversion in der Windows-Programmliste.

# 0.12.0 · 22. November 2026

Treibendes Radiusmaß für Dreipunkt-Kreisbögen und Beispiel Bogenlasche.

# 0.11.0 · 22. November 2026

Innenwinkel im Skizzeninspektor über drei Kontrollpunkte setzen, durch Anklicken der Gradzahl bearbeiten und im Projekt speichern.

# 0.10.0 · 22. November 2026

- Geometrisch gebundene Kanten für Fasen und Rundungen mit Anfangs- und Endmaß. Die Zuordnung übersteht geänderte Netznummern und achsenweise Größenänderungen bei unveränderter normierter Geometrie.
- Sechs lokale Öffnungsseiten für Hohlkörper. Fehlende oder getrennte Öffnungsflächen werden gemeldet.
- Beispiel einer seitlich offenen Schale. Drei zusätzliche Regressionstests zu Größenänderung, fehlendem Bezug und Projekt-Rundlauf.

# 0.9.0

- Grafische Abstandsmaße in Skizzenextrusionen und skizzierten Sweep-Pfaden. Bestehendes Maß über seine Zahl wählen oder zwei Punkte für ein Maß anklicken.
- Segmente und Bedingungen behalten ihre Zuordnung beim Einfügen von Kontrollpunkten. Eine Meldung benennt beim Entfernen eines benutzten Punkts das betroffene Segment oder die Bedingung.
- Parametrische Beispiellasche, integrierte Anleitung und vier neue Regressionstests.

# 0.8.0 · 21. November 2026

- Kontextbezogene Offline-Hilfe über F1 und Hilfeschaltflächen.
- Durchsuchbares Handbuch mit Kapitelauswahl. Bearbeitungsdialoge behalten ihren Zustand.

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
