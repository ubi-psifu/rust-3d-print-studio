# Stand 0.11

Die grafische Winkelbemaßung ergänzt die Abstandsmaße. Nächster Skizzenschritt ist ein treibendes Radiusmaß für Kreisbögen.

# Stand 0.10

Geometrische Kantenbezüge und ebene Öffnungen auf sechs Außenseiten sind verfügbar. Für weitergehende Änderungen der Topologie bleiben Feature-Identitäten im präzisen CAD-Kern und der STEP-Austausch offen.

# Stand 0.9

Die grafischen Abstandsmaße im Skizzeninspektor und die Referenzerhaltung für Skizzenpunkte sind umgesetzt. Als nächster Schritt folgen dauerhafte Bezüge für ausgewählte Modellkanten und Flächen nach einer Neuberechnung. Winkel- und Radiusbemaßung sowie der präzise CAD-Kern mit STEP-Austausch stehen aus.

# Stand 0.7

Der nächste CAD-Schritt betrifft grafische Maßbezüge und stabilere Feature-Referenzen. Der exakte B-Rep-Kern und der STEP-Austausch bleiben offen. Die aktuellen Skizzenbedingungen, Autosicherung und Austauschformate sind in DEVELOPMENT_STATUS.md beschrieben.

# Stand 0.6

Die Plattenwerkzeuge, der grundlegende Netzimport und erste Bauteilgeneratoren sind verfügbar. Nächste Abnahmen betreffen die Windows-Bedienung, größere Importmodelle und die physische Passung des Gehäuses.

# Offline-Engine ab 0.5

Der eigene Schichtkern berechnet Konturen und Linienfüllung ohne Zusatzprogramm. Nächste Schritte für einen eigenständigen Maschinenslicer sind robuste Konturversätze für mehrere Wände, Boden- und Deckflächen, Stützgeometrie und eine mechanisch validierte Bahnplanung. Parallel bleibt die lokale Bambu-CLI der verfügbare Weg für P1S-Druckdateien.

# Prioritäten ab 0.2

Der Konstruktionsablauf hat Vorrang vor Slicing. Der erste integrierte Stand umfasst editierbare Skizzen, Sweep/Loft, Boolesche und den geprüften Wandhaken. Es folgen die Stabilisierung komplexer Featureketten, eine topologisch stabile Auswahl von Kanten und Flächen sowie exakte B-Rep-Flächen und STEP-Roundtrips. Danach folgen Netzimport und umfangreichere Druckanalysen. Ein eigener Slicer hat weiterhin nachrangige Priorität.

# Entwicklungsplan
1. Grundsystem v0.1: Eigenständiges Rust-Fenster mit egui, GPU-Tiefenpuffer, kalibriertem Druckbett und bedienbarer Kamera. Prüfen durch Start und Ansichtswechsel.
2. Modellierung: Parametrische Körper in f64, Auswahl, numerische Bearbeitung und Historie. Geometrie und Rückgängig-Verhalten testen.
3. Lokaler Arbeitsablauf: Versioniertes Projekt, sicherer Dateiersatz und Netzexport. Maße und Anzahl der Flächen unabhängig prüfen.
4. CAD-Ausbau: Skizzen, Zwangsbedingungen und B-Rep-Kern evaluieren. STEP erst nach belastbarem Roundtrip freigeben.
5. Druckvorbereitung: Profilgrenzen, Sperrflächen und anschließend Netzanalyse mit Reparaturvorschau.
6. Slicer: Modellübergabe gegen Bambu Studio prüfen. Druckparameter bleiben bis zur bestätigten Übernahme Projektmetadaten.
7. Freigabe: Plattformtests, Regressionen, Installationspaket und reproduzierbarer Release. Jede Stufe erhält einen dokumentierten Prüfstand.
