# Architektur und Datenmodell

Ein Cargo-Workspace mit einem Paket enthält eine testbare Bibliothek und eine Desktop-Binärdatei. Kleinere Module teilen sich das Paket, damit Geometrie- und Formatänderungen ohne Fenster getestet werden können. Nur die Desktop-Seite besitzt die Ereignisschleife.

| Modul | Verantwortung |
|---|---|
| src/main.rs | Natives Fenster, Abschlussbestätigung, GPU- und UI-Zeichenreihenfolge |
| src/ui.rs | Deutsche Menüs, Auswahl, Eigenschaftsentwurf, Befehle und Dateidialoge |
| src/renderer.rs | Tiefengeprüfte 3D-Netze, Lichtschattierung, Kamera, Raster, Ray-Picking |
| src/geometry.rs | Parametrische Quellen, Triangulierung, Rotation und Hüllbox |
| src/project.rs | Projektzustand, konfigurierbares Druckerprofil, Bereichsprüfung, Undo/Redo |
| src/files.rs | Begrenzter JSON-Import, atomarer Dateiersatz, STL-, OBJ- und 3MF-Ausgabe |

## Geometrie und Anzeige

Rechtshändiges System mit Z nach oben, Koordinaten in mm. Der lokale Ursprung liegt in der Mitte der Grundfläche. X/Y/Z-Abmessungen werden vor der Rotation angewandt. Die Euler-Winkel wirken auf Punkte in der Reihenfolge X, Y, Z, danach folgt die Translation. Modellrechnung, Exportnetz und Grenzen verwenden f64. GPU-Puffer sowie binäre STL-Koordinaten verwenden f32. Der Renderer erneuert die Netzdaten eines Körpers, wenn sich seine Parameter ändern. Auswahl und Export verwenden dieselben Dreiecke. Ein Tiefenpuffer verdeckt rückwärtige Flächen. Eine Orthoprojektion hat unabhängig von der Tiefe denselben Maßstab.

Die Kontur des Bohrquaders besitzt einen äußeren rechteckigen Ring und einen inneren Kreisring mit gleicher Segmentzahl. Die Mantel- und Ringflächen teilen Eckpunktindizes und bilden einen zusammenhängenden Körper. Das ist eine parametrisierte Durchgangsbohrung. Ein allgemeiner B-Rep- oder Boolescher Kern folgt in Phase 4.

## Projektformat R3DP v1

UTF-8-JSON mit den Pflichtfeldern version, name, printer, view und objects. Das mitgelieferte Beispiel ist zugleich eine maschinenlesbare Formatvorlage. Objekte enthalten shape, size, position, rotation, bore, name und color. Winkel sind Gradwerte, Kamerawinkel yaw/pitch sind Radiant. Meshes sind vollständig aus diesen Daten ableitbar. Die Erweiterung auf importierte Netze und separate Geometrieressourcen benötigt eine spätere Schemaversion. Unbekannte Felder, Versionen oder außerhalb der Grenzen liegende Parameter führen beim Öffnen zu einer Fehlermeldung.

Lesegrenze 2 MiB, höchstens 128 Objekte und 32 rechteckige Sperrflächen. Vor dem Speichern werden die Daten erneut validiert. Die Datei wird im Zielverzeichnis temporär geschrieben und synchronisiert. Erst dann ersetzt persist die Zieldatei. Bei neuem Ziel verhindert persist_noclobber den unbeabsichtigten Ersatz einer inzwischen angelegten Datei. Gleichzeitige Änderungen bereits geöffneter Projekte werden nicht zusammengeführt.

## Austausch

STL speichert Dreiecke mit Normalen, OBJ indizierte Eckpunkte und Flächen. Beim STL-Import im Slicer mm wählen. Der 3MF-Container enthält Content Types, Paketbeziehung, Modellteil und Build-Liste. Jedes Projektobjekt ist eine eigene Mesh-Ressource. Objekttransformationen sind in die Weltkoordinaten der Eckpunkte eingerechnet. Slicer-Einstellungen werden im Slicer festgelegt. Der Dateiparser öffnet ausschließlich R3DP-Projekte und explizit ausgewählte JSON-Druckerprofile. Es gibt in dieser Version keinen XML- oder ZIP-Import.

## Verlauf und Profilprüfung

Bis zu 64 Modellierungsstände werden im Arbeitsspeicher gehalten. Neue Befehle verwerfen die Redo-Verzweigung. Parameterfelder bilden einen Entwurf. Erst die Übernahme erzeugt einen Schritt in der Historie. Die Prüfung vergleicht die transformierte Hüllbox mit dem geladenen Bauraum und den Sperrrechtecken. Das kann in Sperrzonen konservativ warnen. Sie bewertet weder dünne Wände noch die Druckstabilität.
