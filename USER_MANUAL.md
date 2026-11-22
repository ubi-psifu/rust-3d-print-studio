# Bedienungsanleitung

## Ein Modell erstellen

1. Im linken Bereich **Quader** wählen. Der Körper startet mit 20 × 20 × 20 mm mittig auf dem Bett.
2. Das Objekt im Modell oder in der Liste **Objekte** auswählen. Ein gelber Rahmen kennzeichnet die Auswahl.
3. Im Eigenschaftenbereich einen Zahlenwert ziehen oder doppelt anklicken und eintippen. **Parameter übernehmen** bestätigt alle Felder gemeinsam. Ungültige Bohrmaße werden zurückgewiesen. Bei einem Kegel und Zylinder entsprechen X und Y den beiden Außendurchmessern. Gleiche Werte ergeben einen Kreis.
4. **Auf Bett absenken** setzt den tiefsten Punkt auf Z = 0. **Mittig platzieren** zentriert die Hüllbox und senkt sie ab. Unter **Werkzeuge** rastet die numerische Position beim Übernehmen auf Wunsch am Raster ein.
5. Größe, Rotation oder Farbe lassen sich erneut ändern. Orts- und Winkelwerte beziehen sich auf die Mitte der ursprünglichen Grundfläche. **Bearbeiten** bietet Rückgängig, Wiederholen, Duplizieren und Löschen.

Der Einstieg über **Bohrquader** erzeugt den Quader und seine offene, durchgehende Bohrung in einem Schritt. Die Bohrung hat einen eigenen Durchmesser. Ein Rohr funktioniert entsprechend. Beim Anlegen weiterer Objekte können sich Körper in der Bettmitte überdecken. Die Objektliste ermöglicht die gezielte Auswahl.

## Ansicht steuern

| Aktion | Bedienung |
|---|---|
| Orbit | Rechte Maustaste halten und ziehen |
| Kamera verschieben | Mittlere Maustaste halten und ziehen |
| Zoom | Mausrad im Arbeitsbereich |
| Ansichten vorne, hinten, links, rechts, oben, unten und isometrisch | Menü Ansicht |
| Orthografisch oder perspektivisch | Ansicht → Perspektive umschalten |
| Drahtgitter | Ansicht → Drahtgitter |
| Auswahl einpassen | Ansicht → Auswahl fokussieren |
| Gesamtes Bett | Ansicht → Druckbett einpassen |
| Helles Design, Rasterweite, Tastenkürzel zuordnen | Einstellungen |

Standardkürzel sind Strg+Z für Rückgängig, Strg+Y für Wiederholen, Strg+S für Speichern und Entf zum Löschen. Unter macOS ist die Befehlstaste für die drei Kombinationen vorgesehen. Bei Texteingaben sind die Modellkürzel inaktiv. Design und Kürzel gelten für die laufende Sitzung.

## Speichern und exportieren

Im Menü **Datei** den Befehl wählen und einen vollständigen Dateipfad samt Endung eingeben. Ein relativer Pfad bezieht sich auf das Arbeitsverzeichnis. Über **Speichern** wird das geöffnete Projekt aktualisiert. **Speichern unter** fragt beim Ersetzen einer vorhandenen Datei nach. Ein Punkt neben dem Projektnamen kennzeichnet bearbeitete Modelldaten. Kameraänderungen werden beim nächsten Speichern mitgeschrieben. Noch offene Eigenschaftsentwürfe vor dem Speichern übernehmen.

| Format | Öffnen | Schreiben |
|---|---|---|
| R3DP v1 | Projekt | Parametrisches Projekt |
| STL binär | Offen | Netz in mm |
| 3MF | Offen | Standard-Modellcontainer mit mehreren Mesh-Ressourcen |
| OBJ | Offen | Geometrie |
| ASCII-STL, STEP/STP, PLY, SVG, GLTF/GLB, G-Code | Offen | Offen |

„Offen“ bezeichnet eine ausstehende Funktion. Netzexport überträgt Geometrie. Zum späteren Ändern der Konstruktionsparameter zusätzlich das R3DP-Projekt speichern. Ein Standard-3MF aus dem Studio enthält keine Bambu-Druckaufträge. In Bambu Studio die Datei manuell öffnen und die aktuellen Maschinen-, Prozess- und Filamentprofile prüfen.

## Grenzen prüfen

Das Standardprofil verwendet 256 × 256 mm Grundfläche und 250 mm Arbeitshöhe bei 256 mm nominaler Höhe. Rot, Grün und Blau markieren die X-, Y- und Z-Achsen. Orange markiert die Profil-Sperrfläche vorne links. Die Statuszeile zählt die Körper mit Bereichswarnung. Zum Testen das Projekt **Profilgrenze.r3dp** öffnen. Dessen 251 mm hoher Quader überschreitet das mitgelieferte Profil.

Für ein angepasstes Profil **profiles/p1s.json** kopieren, Zahlen ändern und über **Druckvorbereitung → Druckerprofil laden** öffnen. **bed** definiert die Abmessungen in X und Y, **print_z** die zulässige Höhe, **excluded** enthält Rechtecke in der Reihenfolge xmin, ymin, xmax, ymax. **nozzle** und **filament** sind Durchmesser in mm. Der geladene Stand wird im Projekt gespeichert. Druckplatte, Sperrflächen und Höhe mit dem aktiven Slicerprofil abgleichen. Ein bestandener Grenztest garantiert keinen physischen Druckerfolg.
