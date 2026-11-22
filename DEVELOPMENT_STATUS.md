# Entwicklungsstand 0.8.0

## Kontexthilfe

Die acht Hauptwerkzeuge, Konstruktionsarten im Inspektor, sechs Plattenmodi, vier Druckvorbereitungsreiter und Datei- und Wiederherstellungsdialoge sind mit der Offline-Anleitung verbunden. F1 berücksichtigt auch den Mauszeiger über den Hauptwerkzeugen und ausgewählten Plattenbefehlen. Einzelne Zahlenfelder besitzen keine eigenen Hilfeartikel. Zwei neue Tests prüfen Kontextverweise und Suchverhalten.

## Version 0.7.0

Die neue Skizzenberechnung kann mehrere freie Punkte in der Ebene oder im Raum verändern. Ein neues Beispiel verbindet die Lösung mit der parametrischen Extrusion. Der Solver prüft die Restabweichung. Bei ungünstigen Startlagen kann eine lösbare Skizze die Iterationsgrenze erreichen. Exaktes B-Rep, STEP sowie ein vollständiger NURBS-Flächeneditor und allgemeine lokale Kantenrundungen stehen weiterhin aus.

Lokale Sicherungen mit getrennten Sitzungen und Wiederherstellung als Kopie sind integriert. Die 2-MiB-Grenze gilt auch für Sicherungen. ASCII-STL kann jetzt geschrieben werden. ASCII-PLY 1.0 unterstützt Geometrie ohne zusätzliche Farbfelder, Normalen oder andere Attribute. Der Export fasst alle Körper in einem Netz zusammen.

Formatierung, Check, Clippy, 48 automatisierte Tests und Release-Builds für Linux und Windows x64 wurden erfolgreich ausgeführt. Windows-Laufzeit, Installation auf einem Windows-System und physischer Druck stehen aus. Die folgenden Abschnitte enthalten auch Abnahmen älterer Versionen.

## Stand vom 21. November 2026

Plattenlayout mit Mustern und Referenzkanten, Import geschlossener Fremdnetze, Schrift- und SVG-Reliefs sowie parametrisches Gehäuse und Deckel sind integriert. Die Platzprüfung arbeitet mit achsparallelen Hüllboxen. Das Projektformat begrenzt die serialisierten Daten auf 2 MiB. Der 3MF-Geometrieimport deckt einfache Modellcontainer ab. Komplexe Bambu-Projekte verwenden den separaten Vorlagenablauf. Der SVG-Import deckt einen Teil der Pfadbefehle und Grundformen ab. Ältere R3DP-Projekte können geöffnet werden. Projekte mit neuen Konstruktionstypen benötigen beim erneuten Öffnen Version 0.6.

41 automatisierte Tests erfolgreich. Release für Linux und Windows x64 gebaut. Windows-Laufzeit, physische Passung und erneute Bambu-Studio-Abnahme der neuen Version stehen aus. Die nachfolgenden Slicer-Prüfungen stammen aus früheren Versionen.

## Offline-Engine

Eigener Rust-Kern für horizontale Konturen und wechselnde Linienfüllung mit Innenaussparungen. Export des vollständigen Modellstapels als SVG/ZIP. P1S-Druckdateien erstellt die separat installierte lokale Bambu-CLI. Offline-Abnahme unter Linux mit gesperrten IPv4-/IPv6-Sockets und Einzelfilamentprofil. Ein Mehrfilamentversuch mit der CLI 02.09.01.70 und einer aus Presets erzeugten Zweifilament-Vorlage endete mit einem Absturz des externen Prozesses. Die Ursache ist offen. Mehrfilament-Projektübergabe wurde in 0.4 geprüft. Das Mehrfilament-Slicing ist noch nicht abgenommen. Physischer Druck und Windows-Laufzeitprüfung stehen aus. Lokale Druckwarteschlange, Kamera und Bestandsverwaltung sind weiterhin offen.

## Lokale Druckvorbereitung

Vorlagenbasierte P1S-Übergabe, Objektfilamente, vier Prozessparameter, geometrische Schnittansicht und externer Slicerstart sind implementiert. Der Bambu-Pfad für Standardinstallationen wird gesucht. Weitere Installationsorte lassen sich manuell eintragen. Die funktionsbezogene Abgrenzung für Cloud, Gerätezugriff und Vorschau steht in [docs/SLICER_INTEGRATION.md](docs/SLICER_INTEGRATION.md).

Die Abnahme prüft das erneute Einlesen von zwei Teilen mit Bambu Studio 02.09.01.70. Eine physische AMS-Zuordnung und ein Druck waren nicht Teil der Abnahme. Zusätzliche Einschränkungen sind das Einzelplattenformat, P1S-Vorlagen mit rechteckiger Bettkontur sowie sitzungsbezogene Vorlagenwahl. Direkte Gerätekommunikation, integrierte Werkzeugwegvorschau und Cloud-Connector sind offen.

## Druckplatten-Werkzeuge

Direktes XY-Ziehen, Gruppen- und Rahmenauswahl, interne Modell-Zwischenablage, platzierte Kopien und automatische Plattenbelegung sind implementiert. Die Anordnung benutzt achsparallele Hüllboxen und behält die Ausrichtung bei. Konkave Teile können dadurch mehr Platz belegen als ihre tatsächliche Kontur. Die Projektgrenzen gelten auch für Serien.

## Abgeschlossen und überprüft

Die Linux-Oberfläche folgt der bereitgestellten SVG-Gliederung mit Werkzeugspalte, Modellansicht, Objektinspektor und grünen Prüf- und Exportschaltflächen. Das Startmodell entspricht dem Bohrquader mit 80 × 80 × 40 mm und 28 mm Bohrung. Aktive Funktionen und Bedienfolgen stehen im USER_MANUAL.md.

Der Wandhaken ist als Generator, frei editierbarer Feature-Baum, binäre STL und Standard-3MF enthalten. Die allgemeine Konstruktion kombiniert Loft, variable Pfadextrusion, Rippenextrusion und boolesche Schritte. Umfang der erneuten Abnahme siehe docs/ABNAHME.md.

| Bereich | Implementiert | Ausstehend oder begrenzt |
|---|---|---|
| Skizzen | Kontrollpunkte grafisch und numerisch, Ebenen und räumliche Skizzen, Linienzüge, Kreis-/Ellipsenbögen, Bézier, B-Spline/NURBS, Hilfsgeometrie, gerichtete Altbedingungen und zehn gemeinsam lösbare Bedingungen | Lokaler iterativer Solver mit begrenzter Iterationszahl. Eine Freiheitsgradanalyse und grafische Maßpfeile am Hauptmodell fehlen. |
| Volumen | Offene/geschlossene Sweeps, Profilskalierung und Verdrehung, gerundete Spitze, Loft, Führung über Zwischenversätze, Extrusion, Rotation, Shell und Flächenverbund | Geschlossener Sweep hat konstante Skalierung und keine zusätzliche Verdrehung. Keine beliebigen unabhängigen Loft-Leitkurven. Shell ist ein Netzversatz mit optionaler ebener Öffnung oben. |
| Konstruktionshistorie | Editierbarer Feature-Baum, variable Maße mit Formeln, Neuaufbau mit letztem gültigem Stand, v1-Migration | Kein exakter B-Rep-Kern, keine dauerhafte topologische Referenzierung. Nummern von Kanten und Flächen können sich bei größerer Quelländerung verschieben. |
| Freeform | Dreiecksecken, Kanten-/Flächenversatz, Unterteilung, Glättung, 2×2×2-Gitter, Symmetrie, Verdrehen, Verjüngen, Biegen, Skalieren | Kein NURBS-Flächeneditor mit Trimmkurven. Selektive Flächenersetzung und beliebige Gitterauflösung fehlen. |
| Körperbearbeitung | Drei Netzboolesche, ebener Schnitt, gewölbter Werkzeugkörper per Differenz, konvexe Stützkanten mit Fase/variabler Rundung, toleranzbasierte Reparatur | Freie gekrümmte Schnittfläche ohne geschlossenen Werkzeugkörper und allgemeine lokale Fillets an konkaven Kanten fehlen. Kein automatisches Füllen beliebiger Löcher. |
| Haken | Acht Voreinstellungen, variable Profile und Spitzen, Lochmuster, Senkungen, Rippe, gerundete Platte, auflösbare freie Konstruktion | Öffnungsweite und tatsächliche Gesamttiefe ergeben sich aus Pfad und Profil. Es gibt noch keine inverse Maßsteuerung dieser beiden Größen. Kantenübergänge an der Rippe sind scharf. |
| Druckprüfung | P1S-Grenzen, Sperrfläche, Kanteninzidenz/-orientierung, entartete Flächen, numerische Gültigkeit, Selbstschnittprüfung, Überhangfläche | Dreiecke mit gemeinsamen Eckpunktindizes werden vom Selbstschnitttest ausgenommen. Allgemeine Mindestwandstärke, Kerbspannungen, Haftung und Werkstoffschrumpfung sind nicht simuliert. |
| Dateien/Plattform | R3DP v1 lesen und v2 schreiben, STL binär, OBJ und 3MF exportieren, Windows-EXE und Installer gebaut, Linux-Fenster geprüft | STEP, Liste zuletzt geöffneter Projekte, native Windows-/macOS-Abnahme und Lastprüfung offen. |

Grenzen pro Projekt sind 2 MiB, 128 Körper und 64 Undo-Schritte. Pro Feature-Baum gelten 128 Auswertungsknoten und 24 Verschachtelungsebenen. Die gespeicherten CAD-Parameter bleiben mit f64 erhalten. Tessellierte Kantenrundungen, Senkungen und Kreisquerschnitte sind segmentabhängig. Die begrenzte Selbstschnittprüfung kann bei sehr aufwendigen Netzen einen Prüfaufwandfehler melden. Neue Objekte und Boolesche können während der ersten Berechnung die Oberfläche kurz anhalten.
