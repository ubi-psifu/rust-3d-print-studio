# Entwicklungsstand 0.2.0

## Abgeschlossen und überprüft

Die Linux-Oberfläche folgt der bereitgestellten SVG-Gliederung mit Werkzeugspalte, Modellansicht, Objektinspektor und grünen Prüf- und Exportschaltflächen. Das Startmodell entspricht dem Bohrquader mit 80 × 80 × 40 mm und 28 mm Bohrung. Aktive Funktionen und Bedienfolgen stehen im USER_MANUAL.md.

Der Wandhaken ist als Generator, frei editierbarer Feature-Baum, binäre STL und Standard-3MF enthalten. Die allgemeine Konstruktion kombiniert Loft, variable Pfadextrusion, Rippenextrusion und boolesche Schritte. Umfang der erneuten Abnahme siehe docs/ABNAHME.md.

| Bereich | Implementiert | Ausstehend oder begrenzt |
|---|---|---|
| Skizzen | Kontrollpunkte grafisch und numerisch, Ebenen und räumliche Skizzen, Linienzüge, Kreis-/Ellipsenbögen, Bézier, B-Spline/NURBS, Hilfsgeometrie, fünf gerichtete Bedingungen | Allgemeiner bidirektionaler Constraint-Solver und interaktive Bemaßung direkt auf dem Hauptmodell fehlen. |
| Volumen | Offene/geschlossene Sweeps, Profilskalierung und Verdrehung, gerundete Spitze, Loft, Führung über Zwischenversätze, Extrusion, Rotation, Shell und Flächenverbund | Geschlossener Sweep hat konstante Skalierung und keine zusätzliche Verdrehung. Keine beliebigen unabhängigen Loft-Leitkurven. Shell ist ein Netzversatz mit optionaler ebener Öffnung oben. |
| Konstruktionshistorie | Editierbarer Feature-Baum, variable Maße mit Formeln, Neuaufbau mit letztem gültigem Stand, v1-Migration | Kein exakter B-Rep-Kern, keine dauerhafte topologische Referenzierung. Nummern von Kanten und Flächen können sich bei größerer Quelländerung verschieben. |
| Freeform | Dreiecksecken, Kanten-/Flächenversatz, Unterteilung, Glättung, 2×2×2-Gitter, Symmetrie, Verdrehen, Verjüngen, Biegen, Skalieren | Kein NURBS-Flächeneditor mit Trimmkurven. Selektive Flächenersetzung und beliebige Gitterauflösung fehlen. |
| Körperbearbeitung | Drei Netzboolesche, ebener Schnitt, gewölbter Werkzeugkörper per Differenz, konvexe Stützkanten mit Fase/variabler Rundung, toleranzbasierte Reparatur | Freie gekrümmte Schnittfläche ohne geschlossenen Werkzeugkörper und allgemeine lokale Fillets an konkaven Kanten fehlen. Kein automatisches Füllen beliebiger Löcher. |
| Haken | Acht Voreinstellungen, variable Profile und Spitzen, Lochmuster, Senkungen, Rippe, gerundete Platte, auflösbare freie Konstruktion | Öffnungsweite und tatsächliche Gesamttiefe ergeben sich aus Pfad und Profil. Es gibt noch keine inverse Maßsteuerung dieser beiden Größen. Kantenübergänge an der Rippe sind scharf. |
| Druckprüfung | P1S-Grenzen, Sperrfläche, Kanteninzidenz/-orientierung, entartete Flächen, numerische Gültigkeit, Selbstschnittprüfung, Überhangfläche | Dreiecke mit gemeinsamen Eckpunktindizes werden vom Selbstschnitttest ausgenommen. Allgemeine Mindestwandstärke, Kerbspannungen, Haftung und Werkstoffschrumpfung sind nicht simuliert. |
| Dateien/Plattform | R3DP v1 lesen und v2 schreiben, STL binär, OBJ und 3MF exportieren, Windows-EXE und Installer gebaut, Linux-Fenster geprüft | Fremdnetzimport, STEP, Autosicherung, Slicerstart, native Windows-/macOS-Abnahme und Lastprüfung offen. |

Grenzen pro Projekt sind 2 MiB, 128 Körper und 64 Undo-Schritte. Pro Feature-Baum gelten 128 Auswertungsknoten und 24 Verschachtelungsebenen. Die gespeicherten CAD-Parameter bleiben mit f64 erhalten. Tessellierte Kantenrundungen, Senkungen und Kreisquerschnitte sind segmentabhängig. Die begrenzte Selbstschnittprüfung kann bei sehr aufwendigen Netzen einen Prüfaufwandfehler melden. Neue Objekte und Boolesche können während der ersten Berechnung die Oberfläche kurz anhalten.
