# Bedienungsanleitung

Unter Windows kannst du nach der Installation ein Projekt im Explorer mit der rechten Maustaste anklicken und unter **Öffnen mit** Rust 3D Print Studio auswählen.

## CAD-Körper als STEP exportieren

Öffne **examples/modelle/STEP-Platte.r3dp** und verschiebe oder dupliziere ein Teil. Wähle **Datei → CAD-Körper als STEP exportieren …**. Das Studio schreibt alle Körper des Projekts mit ihren Objektnamen und ihrer aktuellen Lage in eine lokale STEP-Datei. Die Teile bleiben einzeln lesbar. Auch Kopien mit gleichem Namen bleiben eigenständige Körper.

Öffne **examples/modelle/Referenzbohrquader.r3dp** und wähle den Bohrquader im Objektbaum. Ändere im Inspektor Breite, Tiefe, Höhe oder Bohrungsdurchmesser. Der Durchmesser muss kleiner als Breite und Tiefe sein. **Datei → CAD-Körper als STEP exportieren …** schreibt sechs ebene Außenflächen und eine zylindrische Innenfläche. Position und Drehung werden übernommen. Die Durchgangsbohrung sitzt mittig und folgt der lokalen Z-Achse.

Öffne **examples/modelle/Referenzrohr.r3dp**. Das Rohr hat 14 mm Außendurchmesser, 8 mm Bohrungsdurchmesser und 19 mm Höhe. Ändere die Maße im Inspektor. X und Y müssen gleich sein, der Bohrungsdurchmesser muss kleiner als der Außendurchmesser sein.

Wähle **Datei → CAD-Körper als STEP exportieren …** und gib einen lokalen Pfad mit der Endung **.step** oder **.stp** an. Die Datei enthält Außenmantel, Innenmantel und zwei ringförmige Stirnflächen. Position und Drehung auf der Platte werden übernommen. Für einen Vollzylinder verwende **Referenzzylinder.r3dp**.

Unterstützt werden unbearbeitete Kreiszylinder, gerade konzentrische Rohre und Bohrquader. Bei einem ungeeigneten Objekt nennt das Studio dessen Nummer und Namen. Eine bestehende Zieldatei bleibt dann erhalten. Der allgemeine STEP-Import ist offen.

## Schalenvorschau

Öffne **Keilschale.r3dp**, markiere die Schale und wähle im Inspektor **Frei geneigt**. Der Azimut dreht die Flächennormale in der Draufsicht von +X nach +Y. Der Höhenwinkel kippt die Flächennormale aus der Druckplattenebene in Richtung +Z oder −Z. Zum Ausprobieren verwende **45° Azimut**, **30° Höhenwinkel** und **50 % Schnittlage**.

**Vorschau anzeigen** blickt auf die goldene Materialfläche. Die Schnittlage bezieht sich auf die tatsächliche Modellausdehnung entlang der Normalen. **Normalabstand** gibt den vorzeichenbehafteten Abstand der Ebene vom Ursprung der Druckplatte in Millimetern an. Mit **Esc** kehrst du zu den Eingaben zurück. **Ergebnis übernehmen** übernimmt die vollständige Schale.

Öffne **Keilschale.r3dp** und markiere die Schale. Wähle im Inspektor **Seitlich** für einen Schnitt quer zur X-Achse der Druckplatte, **Frontal** für die Y-Achse oder **Horizontal** für die Z-Achse. Die **Schnittlage %** reicht von 1 bis 99 und bezieht sich auf die Ausdehnung der platzierten Schale entlang der gewählten Achse. Bei 50 % liegt die Ebene in der Mitte.

**Vorschau anzeigen** zeigt den Teil auf der Seite der kleineren Koordinaten. Bei einem senkrechten Schnitt blickst du zunächst auf die freigelegte Fläche. Gold kennzeichnet das geschnittene Material. Die Schaltfläche **Schnitt bei … mm** zeigt Achse und absolute Plattenkoordinate an und schaltet zum vollständigen Entwurf um. **Bisherigen Körper anzeigen** zeigt zum Vergleich den bisherigen Projektstand. Drehe die Ansicht mit der rechten Maustaste. **Esc** führt zu deinen Eingaben zurück. **Ergebnis übernehmen** speichert die vollständige Schale als einen rückgängig machbaren Schritt. **F1** öffnet dieses Kapitel offline.

## Kantenvorschau

Öffne **Kantenquader.r3dp**, wähle den Quader und ändere die Kante oder ihre Maße. Klicke im Inspektor auf **Vorschau anzeigen**. Drehe das Ergebnis mit der rechten Maustaste oder zoome mit dem Mausrad. Über **Bisherigen Körper anzeigen** kannst du den alten und den neuen Stand aus derselben Blickrichtung vergleichen.

Mit **Zurück zum Entwurf** oder **Esc** bleiben deine Eingaben im Inspektor erhalten. **Ergebnis übernehmen** schreibt die berechnete Geometrie als einen rückgängig machbaren Schritt ins Projekt. Eine fehlgeschlagene Berechnung zeigt eine Fehlermeldung im Studio. F1 öffnet diese Anleitung auch offline.

## Kanten und Öffnungsflächen binden

Öffne **Kantenquader.r3dp**, markiere den Quader und wähle im Inspektor **Kante in 3D wählen …**. Bewege den Zeiger von der sichtbaren Fläche an ihren Rand. Die hervorgehobene Linie zeigt die Kante, die du mit einem Linksklick bindest. Mit der rechten Maustaste drehst du den Außenkörper. **Esc** bricht ab und **F1** öffnet diese Anleitung. Stelle Anfangsmaß, Endmaß und Facettenzahl ein und klicke auf **Parameter übernehmen**. Eine Facette erzeugt eine Fase.

Unter **Modellieren → Kante abrunden / fasen** eine scharfe Kante über ihre Nummer wählen. **Kante geometrisch binden** merkt sich Anfang, Ende und Nachbarflächen vor dem nächsten Umbau. **Parameter übernehmen** baut den Körper. Wird die Quelle in X, Y oder Z gestreckt, bleibt die Zuordnung für gleiche normierte Endpunkte und Flächennormalen erhalten. Auch die Richtung von Anfangs- und Endradius bleibt erhalten. Wird die Kante geteilt, ersetzt oder mehrdeutig, meldet der Editor den fehlenden Bezug. Dann die Quelle prüfen und eine neue Kante binden.

Unter **Modellieren → Aushöhlen** die Option **Öffnungsfläche verwenden** aktivieren und eine der sechs lokalen Außenseiten wählen. Eine ebene zusammenhängende Fläche an diesem Extremum dient als Öffnung. Das Beispiel **Seitlich-offene-Schale.r3dp** hat 40 × 30 × 20 mm Außenmaß, 2 mm Wandstärke und eine Öffnung rechts. Die Auswahl bleibt beim erneuten Öffnen des Projekts erhalten.

Für eine schräge Öffnung öffne das Beispiel **Keilschale.r3dp**. Der Keil ist 40 × 30 × 20 mm groß und entlang seiner Schräge offen. Klicke im Inspektor der Aushöhlung auf **Öffnungsfläche in 3D wählen …**. Die Einzelansicht zeigt den vollständigen Außenkörper. Drehe ihn mit der rechten Maustaste, verschiebe die Ansicht mit der mittleren und zoome mit dem Mausrad. Ein Linksklick auf die Schräge bindet die Fläche im Entwurf. Mit **Esc** oder **Abbrechen** kehrst du zum Entwurf zurück. **F1** öffnet diese Anleitung. Für die numerische Auswahl unter **Freie ebene Öffnungsfläche** gibst du die Nummer eines Quelldreiecks ein (beginnend bei 0) und klickst auf **Öffnungsfläche binden**. Die Rückmeldung nennt den Mittelpunkt des gewählten Dreiecks in lokalen Millimetern. Mit **Parameter übernehmen** baust du die Schale neu auf. Gespeichert werden die Ebene und ein Punkt auf der Fläche. Bei achsenweiser Größenänderung wandert der Bezug mit. Fehlt die Fläche oder ist die Ebene mehrdeutig, meldet das Studio den Fehler.

## Hilfe im Studio

Drücke **F1**, um die Hilfe zum aktiven Werkzeug oder zum geöffneten Dialog zu lesen. Über den acht Werkzeugschaltflächen erklärt F1 das Werkzeug unter dem Mauszeiger. Beim Bearbeiten eines parametrischen Körpers öffnet **?** neben dem Namen des Konstruktionsschritts die zugehörige Anleitung. In den Plattenwerkzeugen und der Druckvorbereitung richtet sich die Hilfe nach dem aktuellen Modus oder Reiter.

Über **Hilfe → Anleitung durchsuchen** öffnest du das Handbuch. Gib ein Stichwort ein und wähle einen Treffer. Im Kapitelmenü kannst du ohne Suchbegriff blättern. **Zurück zur Kontexthilfe** führt zum Ausgangsthema. Mit **Esc** oder **Schließen** kehrst du zu deinen Eingaben zurück. Alle Texte sind im Programm enthalten.

## Abstände und Winkel grafisch bemaßen

Öffne `examples/modelle/Bogenlasche.r3dp`. Wähle im Skizzeninspektor unter **Kontur und Hilfslinien** beim Bogen **Radius bemaßen**. Unter **Bemaßung und Bedingungen** erscheint der aktuelle Radius von 12,5 mm. Ändere ihn auf 15 mm und wähle **Parameter übernehmen**. Die fixierten Endpunkte bleiben 20 mm auseinander und der Bogen wird flacher. Das Maß bezieht sich auf die drei Bogenpunkte. Es bleibt bei einer Änderung des Segmenttyps als Bedingung an diesen Punkten erhalten, bis du es entfernst.

Für ein Winkelmaß im Skizzeninspektor **Innenwinkel** wählen. Klicke auf einen Endpunkt, dann den Scheitelpunkt und zuletzt den anderen Endpunkt. Gib den gewünschten Winkel in Grad ein und klicke auf **Maß setzen**. Eine vorhandene grüne Gradzahl kannst du direkt anklicken und bearbeiten. Mit **Parameter übernehmen** wird der Körper neu berechnet. Bei räumlichen Skizzen gilt der Winkel zwischen den beiden räumlichen Strecken, auch in der XY-, XZ- oder YZ-Ansicht. Die Wahl eines Winkelmaßes aktiviert den gemeinsamen Solver. Er kann freie Punkte verschieben. Zwei parallel liegende Schenkel als Ausgangslage gegebenenfalls zuerst leicht gegeneinander drehen.

Öffne zum Ausprobieren `examples/modelle/Bemasste-Lasche.r3dp`. Wähle die Lasche rechts in der Objektliste. Klappe unter **Skizzenextrusion** die **Grafische Bemaßung** auf. Die Vorschau zeigt die berechnete Kontur und vorhandene Abstandsmaße in Millimetern. Klicke auf die gelbe **50.00**, ändere den Wert auf **60 mm** und wähle **Maß setzen**. Nach **Parameter übernehmen** wird die Lasche 60 mm breit. **Strg+Z** nimmt die Modelländerung zurück.

Für ein neues Maß zwei weiße Kontrollpunkte nacheinander anklicken. Die Punkte müssen sich in der gewählten Ansicht unterscheiden lassen. Bei räumlichen Pfadskizzen helfen die Ansichten XY, XZ und YZ. Der Maßwert ist der räumliche Abstand der beiden Punkte. Die Pfeile sind seine Projektion in die Ansicht. Ein bestehendes Abstandsmaß für dasselbe Punktpaar wird aktualisiert. Widerspricht das Maß den übrigen Bedingungen, erscheint die Meldung im Editor.

Mit **+ Punkt** wird nach dem numerisch gewählten Kontrollpunkt ein neuer Punkt eingefügt. Vorhandene Segmente und Bedingungen behalten ihre Endpunkte. Um einen benutzten Punkt zu entfernen, zuerst die angegebenen Segment- oder Bedingungsbezüge ändern. Ein neu eingefügter Punkt gehört erst dann zur Kontur, wenn ein Segment auf ihn verweist.

## Eine bemaßte Platte ändern

Öffne `examples/modelle/Bemasste-Platte.r3dp`, wähle die Platte und öffne im Skizzeninspektor **Bemaßung und Bedingungen**. Ändere das Abstandsmaß 48 auf 60 mm und wähle **Parameter übernehmen**. Die Extrusion übernimmt die neue Kontur. Die fixierte Ecke bleibt relativ zum Skizzenursprung an ihrem Platz.

**Punkte gemeinsam lösen** verteilt die Korrektur auf die freien beteiligten Punkte. Ein fixierter Punkt behält seine eingegebenen lokalen Koordinaten. Horizontal und vertikal richten die Strecke AB an der U- oder V-Achse der Arbeitsebene aus. Senkrecht, parallel und gleich lang beziehen sich auf AB und CD. Bei Spiegelung ist AB die Achse, C und D sind die Punktpartner. Bei Tangente bezeichnen AB und BD die beiden Griffe. Die Nummerierung der Punkte beginnt bei 0. Bei einer unlösbaren Änderung bleibt das zuletzt übernommene Modell erhalten. Bei alten Projekten ist die gemeinsame Lösung zunächst ausgeschaltet. Sie lässt sich pro Skizze aktivieren.

## Ungespeicherte Arbeit zurückholen

Das Studio sichert übernommene Projektänderungen alle 30 Sekunden im Hintergrund. Jede Programminstanz verwendet einen eigenen Sitzungsordner. Nach einem Abbruch das Studio ohne Projektargument starten oder **Datei → Sicherung wiederherstellen …** wählen. Sitzung anhand ihres Alters wählen und das wiederhergestellte Projekt mit **Strg+S** unter einem Projektpfad speichern. Das Studio bewahrt die gefundene Absturzkopie auf. Ihr Pfad erscheint beim Zeigen auf die Sitzung. Unerledigte Eingaben im Eigenschafteneditor und Änderungen seit der letzten Sicherung können beim Abbruch verloren gehen.

Unter Windows liegen die Sitzungen in `%LOCALAPPDATA%/Rust-3D-Print-Studio/Sicherungen`, unter Linux im XDG-Statusordner, sonst in `~/.local/state/Rust-3D-Print-Studio/Sicherungen`, und unter macOS in `~/Library/Application Support/Rust-3D-Print-Studio/Sicherungen`. Nach regulärem Projektspeichern, Verwerfen beim Projektwechsel oder Beenden entfernt das Studio seine laufende Sicherung.

## Serien ausrichten und kopieren

Wähle in der linken Leiste **Auswahl** und **Draufsicht**. Klicke ein Teil an und ziehe es mit gedrückter linker Maustaste. **XY**, **X** und **Y** bestimmen die erlaubten Bewegungsrichtungen. Mit **Umschalt + Klick** nimmst du weitere Teile in die Auswahl auf. Ein Rahmen von einer freien Stelle aus erfasst mehrere Teile.

Öffne **Plattenwerkzeuge …**. Unter **Rechteck** bestimmen Spalten, Reihen und Kantenabstand die Serie. Das Original zählt als erstes Feld. Klicke in der Vorschau auf ein Kopierfeld, um es auszulassen. Rot markierte Felder zeigen Konflikte mit anderen Teilen, Sperrflächen oder dem Plattenrand an. **Kreis** bietet Radius, Anzahl und Mitdrehen. **Kopierschritt** wiederholt einen XY-Versatz. **Ausrichten** legt Minimum, Mitte oder Maximum an einem Referenzteil an. **Verteilen** setzt einen Kantenabstand. Mit **Übernehmen** wird die Vorschau zu einem rückgängig machbaren Arbeitsschritt.

## Gehäuse, Schrift und Relief

Unter **Erstellen → Gehäuse mit Deckel …** gibst du die lichten Innenmaße, Wandstärke, Einstecktiefe und das Spiel je Seite ein. Bei 0,25 mm Spiel ist der Kragen insgesamt 0,5 mm schmaler als die Öffnung. Beide Teile lassen sich anschließend einzeln ändern. Änderungen an gemeinsamen Maßen an beiden Teilen nachführen. Die physische Passung hängt vom Druckverfahren und Material ab.

**Erstellen → Schild und Anhänger** erzeugt ein bearbeitbares Schriftteil. **Erstellen → SVG als Relief …** verarbeitet einfache geschlossene Konturen. Offene Pfade zuvor in der Zeichenanwendung schließen. Zum Testen liegt `examples/modelle/Rahmen.svg` bei.

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
| STL binär | Dreiecksnetz | Netz in mm |
| 3MF | Globale Profileinstellungen aus ungeslicter P1S-Vorlage laden. Einfache Core-Modell-3MF als Geometrie importieren. | Standard-Modellcontainer oder ungeslictes Bambu-Projekt mit Filamentzuordnung |
| OBJ | Polygonnetz | Geometrie |
| ASCII-STL | Dreiecksnetz | Netz in mm |
| ASCII-PLY 1.0 | XYZ und indizierte Polygonflächen eines geschlossenen Netzes | Eine indizierte Dreiecksgeometrie in mm |
| SVG | Geschlossene Konturen als Relief | Schichtzeichnungen |
| STEP/STP | Offen | Kreiszylinder, gerades Rohr oder Bohrquader |
| Binäres PLY, GLTF/GLB, G-Code | Offen | Offen |

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
| Bedingungen | Abstand richtet B im angegebenen Abstand zu A aus. Parallelität richtet C–D parallel zu A–B aus. Tangente richtet B–D an A–B aus. Spiegelung spiegelt C über die Achse A–B nach D. Koinzidenz setzt B auf A. Diese Zuordnung beschreibt den gerichteten Altmodus. Mit „Punkte gemeinsam lösen“ können sich alle beteiligten freien Punkte bewegen. |
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

## Tastenkürzel auf der Druckplatte

Mit Strg+A wählst du alle Objekte aus. Strg+D legt eine Kopie der Auswahl auf der Platte ab. Strg+S speichert das Projekt. Du kannst beide Tasten sofort loslassen. Beim Bearbeiten eines Textfeldes bleiben die Tasten dem Feld zugeordnet.
