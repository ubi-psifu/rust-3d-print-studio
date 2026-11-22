# Architektur und Datenmodell

**cad/references.rs** erzeugt eine Kantensignatur aus normierten Endpunkten und den Normalen der Nachbarflächen im skalierten Raum. Die Auflösung prüft beide Endpunktreihenfolgen und liefert die ursprüngliche Orientierung. Bei der Öffnungsseite prüft ein Nachbarschaftsgraph die Verbindung aller Dreiecke auf der äußeren Ebene. Die optionalen Felder im Feature-Baum erlauben das Lesen älterer Projekte.

Der Bemaßungseditor in `src/dimension_ui.rs` verwendet die gelösten lokalen Skizzenpunkte und dieselbe Segmentsampling-Funktion wie beim Aufbau des Volumenkörpers. Die Vorschau wird bis zu einer Änderung an der Skizze wiederverwendet. `Sketch::set_distance` prüft Änderungen auf einer Kopie und aktualisiert ein bereits vorhandenes Punktpaar in seiner ursprünglichen Richtung. Einfügen und Entfernen von Punkten aktualisieren die Indexbezüge innerhalb der Skizze. Das Dateiformat bleibt lesbar für vorhandene Projekte.

Die Hilfemodule verwenden **USER_MANUAL.md** als eingebettete Kapitelquelle. **src/help.rs** ergänzt kurze werkzeugbezogene Anleitungen, ordnet Kontexten Artikel zu und indiziert Titel und Text einmal beim Start. Der aktuelle Bearbeitungsdialog bleibt beim Anzeigen der Hilfe im Anwendungszustand erhalten.

`cad/sketch_solver.rs` löst in lokalen Skizzenkoordinaten. Der Solver projiziert skalare Restabweichungen über numerische Ableitungen auf freie Punktkoordinaten. Fixierte Koordinaten sind aus den Korrekturen ausgeschlossen. Skizzen ohne serialisierte Solverwahl verwenden beim Laden den gerichteten Altpfad. `recovery.rs` verarbeitet Sicherungs- und Aufräumaufträge je Instanz in einem Hintergrundthread in Auftragsreihenfolge. Für den atomaren Austausch verwendet es den R3DP-Schreiber. `ply.rs` verarbeitet das begrenzte ASCII-Geometrieprofil.

`layout.rs` plant Veränderungen an einer Auswahl in Millimetern. `layout_ui.rs` zeigt denselben Plan, der beim Bestätigen auf eine Projektkopie angewendet wird. Die Historie speichert die Änderung als Einheit. `import.rs` erzeugt Mesh-Features aus unterstützten Austauschformaten. `svg.rs` liest geschlossene Vektorkonturen. `cad/accessories.rs` erzeugt Schild, Gehäuse und Deckel aus gespeicherten Parametern.

## Lokale Engine

`section` berechnet Ebenenschnitte und schneidet parallele Fülllinien nach der Gerade/Ungerade-Regel mit den Konturen eines Objekts. `layers` verwendet die transformierten Netze über sämtliche Schichten wieder, tastet jede Schicht in der Mitte ab und legt getrennte SVG-Gruppen für Kontur und Füllung an. Der Modellstapel endet an der höchsten Geometrie. Die Zeichnungen dokumentieren die geometrische Planung.

`engine` übergibt die aktuelle Platte als neues Projekt an einen lokalen CLI-Prozess. Quelle, Bericht und Ergebnis liegen im temporären Auftragsverzeichnis. Das Programm prüft Exitstatus, Erfolgsbericht für eine Platte, ZIP-Eintrag und vollständig lesbaren Werkzeugweg mit positiver Schichtzahl. Erst danach veröffentlicht es die neue Druckdatei atomar. Es öffnet keine Verbindung zum Drucker. Die Prüfung des Containers ersetzt keine Prüfung der Bewegungen und Materialparameter in der Slicervorschau.

## Slicer-Anbindung

Das Modul `slicer` liest begrenzt den Modellvorspann und die globalen Projekteinstellungen aus einer lokalen ZIP-Vorlage. Unbekannte JSON-Schlüssel in den globalen Einstellungen bleiben erhalten. Das Modul `files` schreibt einen neuen Container mit eigenen Netzen, Modellnamen, Instanzliste und Filamentnummern. Die Bambu-Formatkennung samt Versionsnummer stammt aus der Vorlage, Rust Studio ist im Designer-Feld eingetragen. Bambu Studio benötigt die Formatkennung zur Interpretation der persönlichen Projekteinstellungen. Standard-3MF-Export und Bambu-Projektübergabe haben eigene Befehle. Die Projektübergabe läuft in einem Hintergrundthread.

`section` schneidet Weltkoordinaten-Dreiecke mit einer horizontalen Ebene. `section_ui` berechnet im Hintergrund höchstens einen Schnitt pro laufender Ansicht und zeichnet nur das Ergebnis der aktuell gewählten Höhe. `preparation_ui` verbindet Vorlagen, Objekte und Export. Der lokale Programmstart übergibt einen absoluten Modellpfad als einzelnes Prozessargument. Die Sitzung enthält keine Geräteanmeldung.

Ein Cargo-Workspace mit einem Paket enthält eine testbare Bibliothek und eine Desktop-Binärdatei. Kleinere Module teilen sich das Paket, damit Geometrie- und Formatänderungen ohne Fenster getestet werden können. Nur die Desktop-Seite besitzt die Ereignisschleife.

| Modul | Verantwortung |
|---|---|
| src/main.rs | Natives Fenster, Abschlussbestätigung, GPU- und UI-Zeichenreihenfolge |
| src/ui.rs | Deutsche Menüs, Auswahl, Eigenschaftsentwurf, Befehle und Dateidialoge |
| src/renderer.rs | Tiefengeprüfte 3D-Netze, Lichtschattierung, Kamera, Raster, Ray-Picking |
| src/geometry.rs | Parametrische Quellen, Triangulierung, Rotation und Hüllbox |
| src/plate.rs | Rechteckbelegung für Kopien und automatische Anordnung unter Berücksichtigung des aktiven Druckerprofils |
| src/project.rs | Projektzustand, konfigurierbares Druckerprofil, Bereichsprüfung, Undo/Redo |
| src/files.rs | Begrenzter JSON-Import, atomarer Dateiersatz, STL-, OBJ- und 3MF-Ausgabe |

## Geometrie und Anzeige

Rechtshändiges System mit Z nach oben, Koordinaten in mm. Der lokale Ursprung liegt in der Mitte der Grundfläche. X/Y/Z-Abmessungen werden vor der Rotation angewandt. Die Euler-Winkel wirken auf Punkte in der Reihenfolge X, Y, Z, danach folgt die Translation. Modellrechnung, Exportnetz und Grenzen verwenden f64. GPU-Puffer sowie binäre STL-Koordinaten verwenden f32. Der Renderer erneuert die Netzdaten eines Körpers, wenn sich seine Parameter ändern. Auswahl und Export verwenden dieselben Dreiecke. Ein Tiefenpuffer verdeckt rückwärtige Flächen. Eine Orthoprojektion hat unabhängig von der Tiefe denselben Maßstab.

Die Kontur des Bohrquaders besitzt einen äußeren rechteckigen Ring und einen inneren Kreisring mit gleicher Segmentzahl. Die Mantel- und Ringflächen teilen Eckpunktindizes und bilden einen zusammenhängenden Körper. Das ist eine parametrisierte Durchgangsbohrung. Für allgemeine Netzboolesche verwendet Version 0.2 den separaten Feature-Evaluator.

## Basisfelder des Projektformats

UTF-8-JSON mit den Pflichtfeldern version, name, printer, view und objects. Das mitgelieferte Beispiel ist zugleich eine maschinenlesbare Formatvorlage. Objekte enthalten shape, size, position, rotation, bore, name und color. Winkel sind Gradwerte, Kamerawinkel yaw/pitch sind Radiant. Meshes sind vollständig aus diesen Daten ableitbar. Version 2 ergänzt einen verschachtelten Feature-Baum. Direkte Netze liegen in dessen Polygon-Knoten. Unbekannte Felder, Versionen oder außerhalb der Grenzen liegende Parameter führen beim Öffnen zu einer Fehlermeldung.

Lesegrenze 2 MiB, höchstens 128 Objekte und 32 rechteckige Sperrflächen. Vor dem Speichern werden die Daten erneut validiert. Die Datei wird im Zielverzeichnis temporär geschrieben und synchronisiert. Erst dann ersetzt persist die Zieldatei. Bei neuem Ziel verhindert persist_noclobber den unbeabsichtigten Ersatz einer inzwischen angelegten Datei. Gleichzeitige Änderungen bereits geöffneter Projekte werden nicht zusammengeführt.

## Austausch

STL speichert Dreiecke mit Normalen, OBJ indizierte Eckpunkte und Flächen. Beim STL-Import im Slicer mm wählen. Der 3MF-Container enthält Content Types, Paketbeziehung, Modellteil und Build-Liste. Jedes Projektobjekt ist eine eigene Mesh-Ressource. Objekttransformationen sind in die Weltkoordinaten der Eckpunkte eingerechnet. Slicer-Einstellungen werden im Slicer festgelegt. Die Importmodule verarbeiten Projekt- und Profil-JSON, Netze sowie unterstützte XML- und ZIP-Formate mit Größenbegrenzungen.

## Verlauf und Profilprüfung

Bis zu 64 Modellierungsstände werden im Arbeitsspeicher gehalten. Neue Befehle verwerfen die Redo-Verzweigung. Parameterfelder bilden einen Entwurf. Erst die Übernahme erzeugt einen Schritt in der Historie. Die Prüfung vergleicht die transformierte Hüllbox mit dem geladenen Bauraum und den Sperrrechtecken. Das kann in Sperrzonen konservativ warnen. Sie bewertet weder dünne Wände noch die Druckstabilität.

## Parametrischer Kern ab Version 0.2

**src/cad/** enthält einen deterministisch aus seinen Eingaben berechenbaren Feature-Baum. Blattknoten beschreiben Grundkörper, Skizzen, Profile, Flächengitter und explizite Polygonnetze. Elternknoten nehmen diese als Quellen für Boolesche, Transformationen, Verformung, Aushöhlung oder Kantenbearbeitung. Der Hakengenerator erzeugt intern denselben Baum, den die allgemeine Bearbeitung verwendet. Parametrische Kurven werden in f64 ausgewertet. Boolmesh 0.2.2 schneidet orientierte Dreiecksnetze in f64.

Im Projekt steht die Konstruktionsvorschrift. Abgeleitete Mesh-Caches liegen im Arbeitsspeicher. Beim Öffnen wird der Baum vollständig neu berechnet. Beim Übernehmen eines Eigenschaftsentwurfs übernimmt ein Hintergrundthread die Berechnung. Erst ein gültiges Ergebnis ersetzt das bisherige Objekt in einem Undo-Schritt. Projektladen, externe Exporte und einzelne Menüoperationen arbeiten synchron. Die Modellhistorie ist als strukturierter Feature-Baum dauerhaft, die lineare Undo-Historie ist sitzungsbezogen.

**curve.rs** wertet Polylinien, kubische Bézierzüge, Ellipsenbögen und rationale B-Splines mit De Boor aus. **sketch.rs** setzt Kontrollpunkte auf eine Arbeitsebene oder in den Raum und projiziert gerichtete Bedingungen iterativ. Widersprüchliche Restfehler verhindern die Übernahme. **modifiers.rs** enthält u. a. lokale Ebenengleichungen für den Wandversatz und tangentiale Schnittebenen für konvexe Kanten. **surfaces.rs** trianguliert Flächengitter und vernäht Ränder mit einstellbarer Toleranz. **inspect.rs** prüft nach einer Breitphase Dreieckspaare ohne gemeinsame Eckpunktindizes auf Überschneidung.

R3DP v2 öffnet bestehende v1-Projekte. Speichern erzeugt v2. Die Rollen „parametrisches Feature“ und „direktes Polygonnetz“ sind im Schema explizit. Die Umwandlung in ein Netz erfordert **Parameter übernehmen** und ist über Undo umkehrbar. Ein analytischer B-Rep-Kern und dauerhaft stabile Flächenbenennungen bleiben offen.
