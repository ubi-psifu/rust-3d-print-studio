# Bedienungsanleitung

## Zwei lokale Arbeitsabläufe

**Geometrischer Schichtplan:** **Druckvorbereitung → Platte vorbereiten → Schnittansicht** öffnen und **Eigene Rust-Schichtplanung** aktivieren. Schichthöhe und prozentuale Linienfüllung einstellen. Der Schichtregler zeigt die Konturen und das rechtwinklige Füllraster. In jeder zweiten Schicht dreht sich die Füllrichtung um 90 Grad. **Alle Schichten speichern** schreibt eine neue ZIP-Datei mit maßstäblichen SVG-Zeichnungen und einer JSON-Übersicht. Das ZIP lässt sich entpacken und jede Zeichnung lokal öffnen. Die Schichtberechnung funktioniert unabhängig von einer Bambu-Installation. Der eigene Kern berechnet noch keine versetzten Wandbahnen, Stützen, Deckschichten, Bewegungsreihenfolge oder Maschinenbefehle.

**Druckdatei für den P1S:** Bambu Studio mit den passenden Profilen einmal lokal installieren oder den Installer auf den Offline-Rechner übertragen. Ein eigenes ungeslictes P1S-Projekt mit dem gewünschten Filament als Vorlage speichern. Diese unter **Druckprofil** laden. Unter **Übergabe** die ausführbare Bambu-Studio-Datei und einen neuen Namen mit Endung `.gcode.3mf` angeben. **Platte lokal slicen** berechnet die ganze Platte im Hintergrund. Nach Erfolg erscheinen Schichtzahl, die verfügbare Zeitschätzung und Hinweise der Engine. **Geslicte Datei zur Vorschau öffnen** zeigt die Werkzeugwege in der lokal installierten Bambu-Software. Den nächsten Druckschritt bestimmst du selbst.

Die geometrischen Vorschauwerte im Reiter **Schnittansicht** gelten für den Rust-Schichtplan. Für das tatsächliche P1S-Slicing gelten die Vorlage und die unter **Druckprofil** gewählten Prozesswerte.

## Vom CAD zur Slicer-Platte

1. Modelle auf der Platte auswählen, verschieben und kopieren. Vor der Übergabe **Modell prüfen** aufrufen.
2. Für Filamente und Prozesse in Bambu Studio einmal einen P1S mit passender Düse, gewünschte Filamente und ein Prozessprofil auswählen. Ein eigenes **ungeslictes Projekt als .3mf** speichern. Eine Modell-3MF, eine einzelne Preset-JSON oder eine geslicte Druckplatte eignet sich dafür nicht.
3. Im Rust-Studio **Platte vorbereiten → Druckprofil** öffnen, den Pfad zur eigenen Vorlage eingeben und **Profil aus Vorlage laden** drücken. Unter **Prozesswerte für diese Platte ändern** lassen sich Schichthöhe, Wandlinien, Füllung und Stützen einstellen. Ohne Vorlage kannst du direkt eine Modell-3MF ausgeben.
4. Unter **Filamente** jedem Objekt die Nummer seines Filamentprofils geben. Diese Nummer bleibt beim Kopieren und Speichern im R3DP-Projekt erhalten. Die tatsächliche AMS-Fachbelegung wählst du später am Slicer.
5. Unter **Schnittansicht** die Z-Höhe verschieben. Die Draufsicht zeigt die Schnittkonturen über dem Raster und der Sperrfläche. Mit Vorlage folgen die Farben den Filamenten, sonst den Objektfarben.
6. Unter **Übergabe** einen neuen .3mf-Dateinamen eingeben, vorzugsweise mit vollständigem Pfad. **3MF speichern** funktioniert auch ohne installierten Slicer. **Speichern und im Slicer öffnen** benötigt die installierte Bambu-Studio-Programmdatei. Bei fehlender Erkennung den vollständigen EXE-, Binär- oder AppImage-Pfad eingeben.
7. Im Slicer **Platte slicen** und die Werkzeugwege in der Vorschau kontrollieren. Flächenbemalung, erweiterte Prozessparameter und Gerätewahl sind dort verfügbar.

Vorlagen dienen ausschließlich als Quelle der **globalen Einstellungen**, einschließlich darin enthaltener Maschinen- und Filament-Skripte. Verwende eine eigene vertrauenswürdige Vorlage. Die Geometrie, plattenspezifische und objektspezifische Einstellungen, Bemalung und Zusatzdateien der Vorlage werden nicht kopiert. Der Export verlangt einen neuen Dateinamen. Die ausgewählte Vorlage und die vier Prozessänderungen gelten für die laufende Sitzung und sind in der ausgegebenen Bambu-Projektdatei enthalten.

## Druckplatte belegen

Links **Auswahl** öffnen. **Draufsicht** passt die ganze Platte ins Fenster. Zum Verschieben einen Körper mit der linken Maustaste greifen und ziehen. Die Höhe bleibt erhalten. Für die genaue Positionierung **Am Raster einrasten** einschalten. Ein Zug in einer nahezu waagerechten Kameraansicht lässt sich über **Draufsicht** fortsetzen.

| Gewünschte Aktion | Bedienung |
|---|---|
| Einzelnen Körper wählen | In die Modellfläche oder auf den Namen rechts klicken. Ein Klick ins Leere hebt die Auswahl auf. |
| Körper hinzufügen oder abwählen | Umschalt oder Strg halten und auf Körper oder Namen klicken. |
| Mehrere Körper einrahmen | Auf einer freien Stelle mit der linken Maustaste ziehen. Körper mit Mittelpunkt im Rahmen werden ausgewählt. Umschalt behält die bisherige Auswahl bei. |
| Auswahl gemeinsam verschieben | Einen bereits markierten Körper ziehen. Relative Abstände und Z-Höhen bleiben erhalten. |
| Kopie erstellen | Strg+D oder **Duplizieren**. Strg+C und Strg+V verwenden die interne Modell-Zwischenablage. Unter macOS ist die Befehlstaste vorgesehen. |
| Ganze Platte neu belegen | **Alle anordnen** senkt jeden Körper aufs Bett ab und sucht freie Positionen mit dem eingestellten Abstand. Bei Fehlschlag bleibt die bisherige Belegung erhalten. |
| Serie eines Teils drucken | Erst **Alle anordnen**, dann ein Teil auswählen und **Mit Kopien füllen** klicken. Mit Strg+Z lässt sich der gesamte Füllvorgang zurücknehmen. |
| Alles markieren | Strg+A oder **Alle auswählen**. Entf löscht die markierten Körper gemeinsam. |

Für die automatische Belegung sind **3 mm Abstand** zwischen Körpern voreingestellt. Der Abstand ist veränderbar. Das Verfahren verwendet rechteckige XY-Hüllboxen, lässt die Ausrichtung der Teile unverändert und spart die Sperrflächen des geladenen Profils aus. Kopien einer Mehrfachauswahl übernehmen deren Anordnung. Die manuelle Bewegung erlaubt Überlappungen für die Konstruktion. Bereichswarnungen erscheinen unten. Die automatische Belegung ist eine geometrische Platzierung ohne Prüfung des Werkzeugkopf-Freiraums für sequenziellen Druck.


## Ein Modell erstellen

1. Links **Körper → Körper erstellen → Quader** wählen. Der Körper startet mit 20 × 20 × 20 mm mittig auf dem Bett.
2. Das Objekt im Modell oder in der Liste **Objekte** auswählen. Ein grüner Rahmen kennzeichnet die Auswahl.
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
| R3DP v1/v2 | Projekt, automatische Übernahme von v1 | Parametrisches Projekt v2 |
| STL binär | Offen | Netz in mm |
| 3MF | Globale Profileinstellungen aus ungeslicter P1S-Vorlage laden. Geometrieimport offen. | Standard-Modellcontainer oder ungeslictes Bambu-Projekt mit Filamentzuordnung |
| OBJ | Offen | Geometrie |
| ASCII-STL, STEP/STP, PLY, SVG, GLTF/GLB, G-Code | Offen | Offen |

„Offen“ bezeichnet eine ausstehende Funktion. Netzexport überträgt Geometrie. Zum späteren Ändern der Konstruktionsparameter zusätzlich das R3DP-Projekt speichern. Ein Standard-3MF aus dem Studio enthält keine Bambu-Druckaufträge. In Bambu Studio die Datei manuell öffnen und die aktuellen Maschinen-, Prozess- und Filamentprofile prüfen.

## Grenzen prüfen

Das Standardprofil verwendet 256 × 256 mm Grundfläche und 250 mm Arbeitshöhe bei 256 mm nominaler Höhe. Rot, Grün und Blau markieren die X-, Y- und Z-Achsen. Orange markiert die Profil-Sperrfläche vorne links. Die Statuszeile zählt die Körper mit Bereichswarnung. Zum Testen das Projekt **Profilgrenze.r3dp** öffnen. Dessen 251 mm hoher Quader überschreitet das mitgelieferte Profil.

Für ein angepasstes Profil **profiles/p1s.json** kopieren, Zahlen ändern und über **Druckvorbereitung → Druckerprofil laden** öffnen. **bed** definiert die Abmessungen in X und Y, **print_z** die zulässige Höhe, **excluded** enthält Rechtecke in der Reihenfolge xmin, ymin, xmax, ymax. **nozzle** und **filament** sind Durchmesser in mm. Der geladene Stand wird im Projekt gespeichert. Druckplatte, Sperrflächen und Höhe mit dem aktiven Slicerprofil abgleichen. Ein bestandener Grenztest garantiert keinen physischen Druckerfolg.

## Wandhaken und Konstruktionsbaum

**Körper → J-Wandhaken** fügt den Generator ein. Im Feld **Hakentyp** stehen Wand-, J-, S-, Decken-, Kleider-, Doppel-, Kabel- und Einhängehaken bereit. Eine Typwahl setzt den Entwurf auf die Voreinstellung dieses Typs. Frei hängende Varianten verwenden ihren Pfad ohne Montageplatte. Bei den montierten Varianten bilden Breite, Höhe, Dicke und Außenradien die Platte. Der Durchmesser, die Ovalität, der Biegeradius, die Schenkellänge, die Spitzenlänge, der Spitzenwinkel und die Endskalierung bestimmen den Arm. **Rippenbreite 0** entfernt die Rippe, **Lochanzahl 0** entfernt die Bohrungen. Gleiche Bohr- und Senkdurchmesser ergeben eine einfache Durchgangsbohrung.

Die Anzeige **Abmessungen** gibt die Maße der letzten übernommenen Geometrie in lokalen Achsen aus. **Messen** zeigt die achsparallele Hüllbox nach der Rotation. Unvereinbare Maße erzeugen eine Meldung in der Statusleiste. Der letzte gültige Körper bleibt sichtbar und exportierbar. Eine Änderung der Eigenschaftsfelder wird mit **Parameter übernehmen** bestätigt.

Mit **In einzelne Features auflösen** wird der Generator im Entwurf durch seine Konstruktion ersetzt. Die Eingänge einer Vereinigung oder Differenz lassen sich bis zur Skizze aufklappen. So können die Pfadkontrollpunkte, Profile, Senkungen oder der Rippenumriss unabhängig geändert werden. **Wandhaken-Einzelfeatures.r3dp** öffnet diesen Zustand direkt.

## Skizzen und allgemeine Körper

| Werkzeug | Bedienung |
|---|---|
| Skizzenkontur | Punkte in der Zeichenfläche ziehen oder X/Y/Z eingeben. XY, XZ und YZ wählen die Bearbeitungsansicht. Ursprung und U-/V-Achse definieren die Arbeitsebene. |
| Kontursegmente | Linie verwendet A–B, Bogen A–B–C mit B als Durchgangspunkt, kubischer Bézier A–B–C–D. Indizes beginnen bei 0. Hilfsgeometrie wird bei der Körperbildung ausgelassen. Extrusion erfordert einen geschlossenen Zug ohne doppelte Endsegmente. |
| Bedingungen | Abstand richtet B im angegebenen Abstand zu A aus. Parallelität richtet C–D parallel zu A–B aus. Tangente richtet B–D an A–B aus. Spiegelung spiegelt C über die Achse A–B nach D. Koinzidenz setzt B auf A. Diese Regeln wirken gerichtet. |
| Sweep | Polyline, räumliche Skizze, Bézier, Ellipsenbogen oder NURBS auswählen. Lokale Querschnittspunkte bearbeiten oder Rund-, Ellipsen- bzw. Rechteckprofil wählen. Endskalierung, Profilachse und Verdrehung einstellen. |
| Geschlossener Sweep | Profil achsparallel oder über die Profilachse ausrichten, Endskalierung 1, Verdrehung 0 und gerundete Spitze aus. **Pfad schließen** verbindet den letzten Ring mit dem ersten. |
| Loft | Profile mit gleicher Punktanzahl und entsprechender Umlaufrichtung bearbeiten. **Glatter Loft mit Führung** interpoliert Zwischenringe. Ein dreidimensionaler Führungsversatz je Intervall beeinflusst dessen Mitte. |
| Rotation | Ebenes Profil, Achsenpunkt, Achsenrichtung und Drehwinkel einstellen. Für einen Hohlkörper ein Profil mit Abstand von der Achse verwenden. |
| Maßbeziehungen | Unter Modellieren am ausgewählten Objekt anfügen. Beispiel beim Wandhaken: Variable **mass1** auf 18 setzen, Ziel **/Hook/width**, Formel **mass1 * 2**. Nach Übernahme beträgt die Plattenbreite 36 mm. Dezimalpunkt, Klammern und die vier Grundrechenarten sind verfügbar. |
| Boolesche | Grundkörper auswählen, **Schneiden → Körperschnitt** öffnen, Werkzeugkörper und Operation wählen. Beide Quellen bleiben im Ergebnisbaum. Bei einer Differenz bestimmt die Auswahlreihenfolge, welcher Körper abgetragen wird. |
| Kanten und Schnitt | **Modellieren → Kante abrunden / fasen** verwendet nummerierte scharfe, konvexe Stützkanten. Anfangs- und Endmaß dürfen verschieden sein. Eine Facette ergibt eine Fase. Beim Ebenenschnitt bleibt die Seite in Richtung der Normalen erhalten. |

## Direkte Netzbearbeitung und Freiform

**Modellieren → In direktes Polygonnetz umwandeln** erzeugt einen Entwurf mit expliziten Dreiecken. Nach dem Übernehmen lässt sich eine Fläche über ihren Index wählen und über ihre drei Eckpunkte bearbeiten. Für eine Kante wirkt der Versatz auf die ersten beiden Eckpunkte, für die Fläche auf alle drei. Angrenzende Flächen teilen diese Punkte. Unterteilung halbiert jede Dreieckskante mit gemeinsam genutzten Mittelpunkten. Glätten verschiebt Punkte in Richtung ihrer Nachbarschaft.

**Kontrollgitter** verformt einen Körper trilinear über acht Eckpunkte. Die X-Symmetrie koppelt die Versätze gegenüberliegender Gitterpunkte. **Biegen / Verdrehen / Verjüngen** wirkt entlang der lokalen Z-Höhe. Skalierung und Spiegelung sind über den Transformationsknoten möglich. **Flächenverbund** beginnt mit sechs bearbeitbaren Flächengittern. Die Nahttoleranz bestimmt, welche benachbarten Randpunkte verschweißt werden. Offene Ränder werden beim Aufbau eines Volumens gemeldet.

**Netzreparatur vorbereiten** schweißt nahe Punkte, entfernt doppelte oder entartete Dreiecke und korrigiert widersprüchliche Nachbarorientierungen. Vor Übernahme erscheinen die Punkt- und Flächenzahlen im Entwurf. Die globale Änderung lässt sich rückgängig machen. **Modell prüfen** meldet Profilgrenzen, offene oder inkonsistente Kanten, Selbstschnitte nicht benachbarter Dreiecke und die Fläche steiler abwärts gerichteter Dreiecke oberhalb der Unterseite. Überhänge sind eine geometrische Orientierungshilfe. Material, Schichthaftung, Schrauben, Wandbefestigung und Lastfall bedürfen separater praktischer Prüfung.
