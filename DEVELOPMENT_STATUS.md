# Entwicklungsstand

## CAD-Flächenbezüge 0.13.0

Ebene Öffnungsflächen lassen sich unabhängig von der Dreiecksnummer speichern und neu auflösen. Das Beispiel Keilschale misst 40 × 30 × 20 mm bei 2 mm Wandstärke. Unter Linux bestanden 67 Rust-Tests, Formatierung, Workspace-Check und Clippy. Die Modellabnahme prüfte den Projektrundlauf, die geschlossene und konsistent orientierte Oberfläche und das analytische Volumen von 4 486 mm³. Die zweite Abnahme prüfte den Bezug nach Umsortierung und ungleicher Skalierung sowie die Ablehnung einer fehlenden Ebene. Der erste native Windows-Test deckte einen Fehler beim Erkennen von Kapitelüberschriften mit CRLF auf. Die Korrektur ist durch einen separaten Test abgesichert. Die Windows-Fensterprüfung verwendet in dieser Version die Keilschale und kontrolliert den gespeicherten Flächenbezug.

Bezüge gelten für eine zusammenhängende gleich orientierte Fläche in der gespeicherten normierten Ebene. Allgemeine gekrümmte B-Rep-Flächen, Auswahl durch Klick in die 3D-Ansicht und STEP bleiben offen.

## Plattenbedienung 0.12.2

Die Tastenkürzel für die Auswahl und Projektbearbeitung verwenden die beim Tastendruck erfassten Modifikatortasten. Unter Linux bestanden 64 Rust-Tests, Formatierung, Workspace-Check und Clippy mit Warnungen als Fehlern. Das Release enthält Builds für Linux und Windows x64. Der Windows-Fenstertest speichert eine Projektkopie über ein kurzes Strg+S und liest den Objektnamen zurück. Im ersten Lauf bestand die Installerprüfung. WScript.SendKeys bestätigte die Speicherung nicht. Der zweite Test sendet Windows-Tastenereignisse mit expliziten Scancodes ohne Wartezeiten zwischen Drücken und Loslassen. Das Ergebnis steht im Actions-Lauf zur Veröffentlichung. Windows 11 mit Hardware-Grafiktreiber und der STEP-Rundlauf bleiben offen.

## Windows-Installer 0.12.1

Explorer-Registrierung und Versionsanzeige sind angepasst. Die 62 Tests unter Linux und beide x64-Release-Builds sind erfolgreich. Das Skript scripts/windows-abnahme.ps1 prüft auf einem frischen Windows-Runner Installation, Registry und Deinstallation, optional auch Öffnen und Speichern im Fenster. Am 22. November 2026 bestand das Release-Paket die Abnahme auf Windows Server 2022. Der Test installierte das Setup im Benutzerprofil, verglich Programm und Beispiel mit den Dateien im ZIP, prüfte die installierte Version 0.12.1 und den registrierten Explorer-Befehl und kontrollierte die Deinstallation. Das portable Studio öffnete mit Mesa-Software-OpenGL eine Kopie der Bogenlasche aus einem Pfad mit Umlaut und Leerzeichen. Strg+S schrieb die Kopie neu. Der Test las die gespeicherte Datei zurück und prüfte den Objektnamen. Der Fenstertest ohne zusätzlichen Grafiktreiber auf diesem Runner endete mit Prozesscode 101. Die Bedienung auf Windows 11 mit einem Hardware-Grafiktreiber bleibt offen. [Windows-Abnahmelauf](https://github.com/ubi-psifu/rust-3d-print-studio/actions/runs/44999456694/job/137137560715)

## Funktionsumfang seit 0.12.0

| Bereich | Verfügbar | Offene Prüfung oder Grenze |
| --- | --- | --- |
| Skizzen | Abstände, Innenwinkel und Radien von Dreipunktbögen. Die Modelle lassen sich nach einer Maßänderung neu aufbauen und als R3DP speichern. | Bei drei kollinearen Punkten ist der Radius unbestimmt. Ein exakter Halbkreis mit fixierten Endpunkten kann den lokalen Solver an seine Grenze bringen. |
| Modellierung | Parametrische Konstruktion mit tessellierten Körpern, Netzboolesche, geometrisch gebundene scharfe Kanten und ebene Hohlkörperöffnungen auf sechs Außenseiten. | Allgemeine dauerhafte B-Rep-Flächenbezüge, exakte CAD-Volumenkörper und STEP-Austausch bleiben offen. |
| Druckplatte | Mehrfachauswahl, direktes Verschieben, Kopieren, Muster und automatische Anordnung im P1S-Profil. | Die Platzsuche nutzt achsparallele Hüllboxen. Bei konkaven Teilen können freie Bereiche ungenutzt bleiben. |
| Slicing | Geometrische Schnittansicht und SVG-Schichtstapel. Die lokale Bambu-Studio-CLI erzeugt mit passender Vorlage druckerspezifische Dateien. | Die erfolgreiche Mehrfilament-Übergabe und die Abnahme eines vollständigen Mehrfilament-Slicinglaufs sind getrennte Prüfschritte. Letztere ist offen. |
| Plattformen | Release-Paket mit Linux-x64-Programm, Windows-x64-EXE und NSIS-Installer. | Windows-11-Arbeitsplatztest und macOS-Abnahme stehen aus. |

Für 0.12.0 bestanden unter Linux 62 automatisierte Tests einschließlich der Radiusänderung mit Speichern und erneutem Öffnen. Formatierung, Workspace-Check, Clippy und beide Release-Builds liefen erfolgreich. [Release und Download](https://github.com/ubi-psifu/rust-3d-print-studio/releases/tag/v0.12.0)

## Frühere Prüfstände

Die folgenden Einträge beschreiben jeweils die damalige Version. Für heutige Funktionen und offene Arbeiten gilt die Übersicht oben.

### Version 0.11.0

Winkelmaße für ebene und räumliche Skizzen sind integriert. Der gemeinsame lokale Solver unterstützt Innenwinkel zwischen 0 und 180 Grad ausschließlich der Grenzen. Widersprüche beim grafischen Setzen belassen den Entwurf im vorherigen Zustand. Der Beispieltest ändert die Lasche von 60° auf 75°, baut den Körper neu auf und kontrolliert die Höhe nach dem Laden. 60 Tests und alle Cargo-Prüfungen bestanden. EXE und Installer wurden gebaut. Praktische Windows-Abnahme offen. Radiusbemaßung, STEP und exaktes B-Rep bleiben offen.

### Version 0.10.0

Rundungen und Fasen können einen geometrischen Kantenbezug speichern. Er erkennt die Endpunkte und die angrenzenden Normalen relativ zur lokalen Hüllbox. Fehlt ein eindeutiger Treffer, schlägt die Neuberechnung mit einer Meldung fehl. Hohlkörper unterstützen zusammenhängende ebene Öffnungsflächen auf sechs lokalen Außenseiten. Ältere Projekte behalten ihre nummerierte Kante bzw. die obere Öffnung. Beliebige gekrümmte Flächen, durch Teilung entstandene Kanten und semantische B-Rep-Bezüge bleiben offen.

57 Tests sowie die Cargo-Prüfungen unter Linux bestanden. Windows-Programm und Setup wurden unter Linux gebaut. Windows-Laufzeit und physischer Druck bleiben ungeprüft.

### Version 0.9.0

Die grafische Bemaßungsansicht für Skizzenextrusionen und skizzierte Sweep-Pfade zeigt berechnete lokale Punkte, Kontur und Abstandsmaße. Vorhandene Maßzahlen sowie Punktpaare können ausgewählt werden. Widersprüchliche Abstände werden geprüft, bevor sie in den Entwurf gelangen. Beim Einfügen und Entfernen von Kontrollpunkten führt der Editor die Punktindizes von Segmenten und Bedingungen nach. Verwendete Punkte sind gegen versehentliches Entfernen geschützt.

Die Bemaßungsansicht liegt im Inspektor. Winkel- und Radiusmaße, persistente Referenzen auf Modellkanten oder -flächen und der STEP-Austausch bleiben offen. Formelausdrücke mit Positionsindizes im Feature-Baum werden bei Strukturänderungen nicht umgeschrieben. Bei deckungsgleichen Punkten einer Projektion eine andere Ansicht oder die numerischen Punktfelder verwenden. Im Linux-Bedientest wurde eine 50-mm-Lasche auf 60 mm geändert, gespeichert, zurückgenommen und wiederhergestellt. Windows-Laufzeit und physischer Druck stehen aus.

### Kontexthilfe

Die acht Hauptwerkzeuge, Konstruktionsarten im Inspektor, sechs Plattenmodi, vier Druckvorbereitungsreiter und Datei- und Wiederherstellungsdialoge sind mit der Offline-Anleitung verbunden. F1 berücksichtigt auch den Mauszeiger über den Hauptwerkzeugen und ausgewählten Plattenbefehlen. Einzelne Zahlenfelder besitzen keine eigenen Hilfeartikel. Zwei neue Tests prüfen Kontextverweise und Suchverhalten.

### Version 0.7.0

Die neue Skizzenberechnung kann mehrere freie Punkte in der Ebene oder im Raum verändern. Ein neues Beispiel verbindet die Lösung mit der parametrischen Extrusion. Der Solver prüft die Restabweichung. Bei ungünstigen Startlagen kann eine lösbare Skizze die Iterationsgrenze erreichen. Exaktes B-Rep, STEP sowie ein vollständiger NURBS-Flächeneditor und allgemeine lokale Kantenrundungen stehen weiterhin aus.

Lokale Sicherungen mit getrennten Sitzungen und Wiederherstellung als Kopie sind integriert. Die 2-MiB-Grenze gilt auch für Sicherungen. ASCII-STL kann jetzt geschrieben werden. ASCII-PLY 1.0 unterstützt Geometrie ohne zusätzliche Farbfelder, Normalen oder andere Attribute. Der Export fasst alle Körper in einem Netz zusammen.

Formatierung, Check, Clippy, 48 automatisierte Tests und Release-Builds für Linux und Windows x64 wurden erfolgreich ausgeführt. Windows-Laufzeit, Installation auf einem Windows-System und physischer Druck stehen aus. Die folgenden Abschnitte enthalten auch Abnahmen älterer Versionen.

### Stand vom 21. November 2026

Plattenlayout mit Mustern und Referenzkanten, Import geschlossener Fremdnetze, Schrift- und SVG-Reliefs sowie parametrisches Gehäuse und Deckel sind integriert. Die Platzprüfung arbeitet mit achsparallelen Hüllboxen. Das Projektformat begrenzt die serialisierten Daten auf 2 MiB. Der 3MF-Geometrieimport deckt einfache Modellcontainer ab. Komplexe Bambu-Projekte verwenden den separaten Vorlagenablauf. Der SVG-Import deckt einen Teil der Pfadbefehle und Grundformen ab. Ältere R3DP-Projekte können geöffnet werden. Projekte mit neuen Konstruktionstypen benötigen beim erneuten Öffnen Version 0.6.

41 automatisierte Tests erfolgreich. Release für Linux und Windows x64 gebaut. Windows-Laufzeit, physische Passung und erneute Bambu-Studio-Abnahme der neuen Version stehen aus. Die nachfolgenden Slicer-Prüfungen stammen aus früheren Versionen.

### Offline-Engine

Eigener Rust-Kern für horizontale Konturen und wechselnde Linienfüllung mit Innenaussparungen. Export des vollständigen Modellstapels als SVG/ZIP. P1S-Druckdateien erstellt die separat installierte lokale Bambu-CLI. Offline-Abnahme unter Linux mit gesperrten IPv4-/IPv6-Sockets und Einzelfilamentprofil. Ein Mehrfilamentversuch mit der CLI 02.09.01.70 und einer aus Presets erzeugten Zweifilament-Vorlage endete mit einem Absturz des externen Prozesses. Die Ursache ist offen. Mehrfilament-Projektübergabe wurde in 0.4 geprüft. Das Mehrfilament-Slicing ist noch nicht abgenommen. Physischer Druck und Windows-Laufzeitprüfung stehen aus. Lokale Druckwarteschlange, Kamera und Bestandsverwaltung sind weiterhin offen.

### Lokale Druckvorbereitung

Vorlagenbasierte P1S-Übergabe, Objektfilamente, vier Prozessparameter, geometrische Schnittansicht und externer Slicerstart sind implementiert. Der Bambu-Pfad für Standardinstallationen wird gesucht. Weitere Installationsorte lassen sich manuell eintragen. Die funktionsbezogene Abgrenzung für Cloud, Gerätezugriff und Vorschau steht in [docs/SLICER_INTEGRATION.md](docs/SLICER_INTEGRATION.md).

Die Abnahme prüft das erneute Einlesen von zwei Teilen mit Bambu Studio 02.09.01.70. Eine physische AMS-Zuordnung und ein Druck waren nicht Teil der Abnahme. Zusätzliche Einschränkungen sind das Einzelplattenformat, P1S-Vorlagen mit rechteckiger Bettkontur sowie sitzungsbezogene Vorlagenwahl. Direkte Gerätekommunikation, integrierte Werkzeugwegvorschau und Cloud-Connector sind offen.

### Druckplatten-Werkzeuge

Direktes XY-Ziehen, Gruppen- und Rahmenauswahl, interne Modell-Zwischenablage, platzierte Kopien und automatische Plattenbelegung sind implementiert. Die Anordnung benutzt achsparallele Hüllboxen und behält die Ausrichtung bei. Konkave Teile können dadurch mehr Platz belegen als ihre tatsächliche Kontur. Die Projektgrenzen gelten auch für Serien.

### Abgeschlossen und überprüft

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
