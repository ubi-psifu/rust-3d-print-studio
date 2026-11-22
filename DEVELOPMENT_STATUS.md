# Entwicklungsstand 0.1.0

| Phase | Stand | Nachweis und verbleibende Arbeit |
|---|---|---|
| 1 Grundsystem | Implementiert | Linux-Fenster mit deutscher Oberfläche, Tiefenpuffer, P1S-Bett, Orbit, Pan, Zoom, feste Ansichten, Raster, Sperrzone und Themes. Physische GPU und Windows-Laufzeit noch zu prüfen. |
| 2 Grundmodellierung | Teilweise | Quader, Zylinder, Kegel, Pyramide, Rohr, Bohrquader, Sechskant, numerische Abmessungen, Rotation, Position, Objektfarbe, Auswahl und Undo/Redo. Weitere Generatoren, Mehrfachauswahl, Gruppen und interaktive Transformationsgriffe offen. |
| 3 Dateien | Teilweise | R3DP öffnen/speichern, STL binär, OBJ und 3MF exportieren. Netzimport, ASCII-STL, Autosicherung, Wiederherstellung, Projektvorschau und zuletzt geöffnete Dateien offen. |
| 4 CAD | Teilweise | Parametrisch editierbare Durchgangsbohrung als definierter Körpergenerator. Allgemeine Boolesche Operationen, Skizzenlöser, B-Rep, STEP, Fillets und Fasen offen. |
| 5 Druckoptimierung | Teilweise | Konfigurierbare Arbeitsgrenzen und Sperrflächen sowie Platzierung einzelner Objekte. Mesh-Reparatur, Selbstschnitt, Wandstärke, Überhang und Kollisionsprüfung offen. |
| 6 Slicer | Teilweise | Geometrieübergabe beider Hauptformate im Bambu-Studio-CLI getestet. Direkter Slicerstart, Prozessprofile, AMS und G-Code-Vorschau offen. |
| 7 Freigabe | Erster Build | Tests, Beispielmodelle, Bedienungs- und Bauanleitung sowie Windows-Paket vorhanden. Plattformabnahme, Langzeit- und Lasttests vor Version 1.0 offen. |

## Bekannte Grenzen

- Das Projekt speichert Formparameter, Transformationen, Farben, Kamera und Druckerprofil. Erstellungsdaten, dauerhafte Konstruktionshistorie, Filamente und Druckprozessparameter folgen mit späteren Schemaversionen.
- Höchstens 128 Körper und 64 Undo-Schritte. Änderungen an einem Zahlenfeld erst mit „Parameter übernehmen“ bestätigen. Unbestätigte Entwürfe werden bei Wechsel der Auswahl verworfen. Kameraänderungen allein setzen keine Änderungsmarke.
- Projekt- und Exportdialoge verwenden ein Pfadfeld. Speichern und Export laufen synchron. Die Höchstzahl und feste Auflösung der Körper begrenzen die Arbeit. Es gibt keine Hintergrundverarbeitung großer Importnetze.
- Die Hüllboxprüfung kann bei schräger oder hohler Geometrie konservativ warnen. Überlappende Objekte werden einzeln exportiert, ohne Union oder Überschneidungswarnung.
- Die Anzeige verwendet eine Einzelobjektauswahl. Transparenz, Schnittdarstellung, permanente Tastaturprofile und reparaturbezogene Fehleransichten stehen aus.
- Binäre Abnahme auf Windows und macOS sowie ein physischer Testdruck stehen aus. Der virtuelle Linux-Test verwendet Mesa-Software-Rendering über denselben OpenGL-Pfad.

Der nächste Entwicklungsschritt ist ein begrenzter STL-Import mit eigenem Netzobjekttyp und R3DP-Migration. Damit bleiben importierte Dreiecke von den bereits gespeicherten Konstruktionsparametern unterscheidbar.
