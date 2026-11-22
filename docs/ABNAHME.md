# Abnahme 0.7.0

Die sechs Austausch- und Skizzentests sowie der neue Sicherungstest ergänzen die 41 vorherigen Tests. Alle 48 Tests, Formatprüfung, Workspace-Check, Clippy mit Warnungen als Fehler sowie die optimierten Builds für Linux und Windows x64 liefen erfolgreich. Das NSIS-Installationspaket wurde gebaut.

Im Linux-Fenster wurde die bemaßte Platte geöffnet und dupliziert. Nach dem Sicherungsintervall enthielt der Checkpoint zwei Körper und das Originalprojekt einen. Anschließend wurde der Prozess hart beendet. Beim Neustart ohne Projektargument bot das Studio die Sicherung an. Die wiederhergestellte Kopie wurde über den Speicherdialog als neue R3DP-Datei geschrieben. Sie enthielt beide Körper, das ursprüngliche Projekt weiterhin einen.

Die automatisierte Geometrieprüfung ergab 48 × 30 × 6 mm für die Beispielplatte und bestätigte identische Grenzen sowie Dreieckszahlen nach Export und erneutem Import als ASCII-STL und ASCII-PLY. Keine neue Bambu-Studio- oder Windows-Laufzeitabnahme und kein physischer Druck in dieser Version.

# Abnahme 0.6.0

Rust 1.88.0 unter Linux. Formatierung, Check aller Targets im Workspace, Clippy mit Warnungen als Fehler, 41 Tests sowie optimierte Linux- und Windows-x64-Builds liefen erfolgreich mit gesperrter Abhängigkeitsauflösung. Die neuen Fälle prüfen unter anderem eine Serie mit zwölf Körpern und Undo, Konflikte und ausgelassene Felder, numerische Abstände, Netzexport mit erneutem Import, Schrift mit Innenkonturen und die Gehäuse- und Kragenmaße.

Im nativen Linux-Fenster wurde ein 20-mm-Würfel als 4 × 3-Serie angeordnet. Nach dem Speichern enthielt das Projekt zwölf Körper. Einmal Strg+Z und erneutes Speichern ergaben wieder einen Körper. Das neue Gehäusebeispiel wurde im Test geladen, als 3MF exportiert und als zwei Objekte erneut importiert. Bambu-Studio-Import, Installation auf Windows und Druck der neuen Beispielteile stehen für 0.6 aus.

# Abnahme 0.3.0

Im nativen Linux-Fenster wurden zwei 20-mm-Quader per Umschalt-Klick ausgewählt und in der Draufsicht mit einem Mauszug um rund 20,34 mm in X und 10,17 mm in Y verschoben. Die gespeicherten Positionen hatten weiterhin 40 mm Abstand sowie unveränderte Z-Koordinaten. Strg+C/V erzeugte zwei neue Körper. Zwei Undo-Schritte entfernten die Kopien und setzten die Ausgangspositionen wieder ein. Eine Rahmenauswahl mit anschließendem Strg+D erzeugte erneut zwei Kopien.

Ein weiterer Fenstertest belegte die P1S-Platte mit 119 Würfeln von 20 mm Kantenlänge. Die gespeicherten Positionen hielten 3 mm Mindestabstand und die Sperrfläche ein. Ein Undo setzte die Serie auf den ursprünglichen Würfel zurück.

29 automatisierte Tests. Die neuen Fälle prüfen Abstände, Sperrflächen, gedrehte Hüllboxen, Gruppenversatz, Objektlimit beim Kopieren sowie vollständige oder verworfene Anordnungen. Die fünf Cargo-Prüfungen liefen auf Linux. Windows-x64-Programm und Installer wurden gebaut. Ein Start auf Windows bleibt offen.

# Abnahmeprotokoll · 0.2.0 · 20. November 2026

## J-Wandhaken

| Parameter | Abnahmewert |
|---|---:|
| Platte | 32 × 78 × 6 mm |
| Eckenradius / Rundung am Plattenrand | 5 / 1,2 mm |
| Schenkel / Biegeradius | 20 / 13 mm |
| Ausgangsdurchmesser / Endskalierung | 11 mm / 0,64 |
| Spitzenlänge / Spitzenwinkel | 10 mm / 15° |
| Bohrungen | 2, Achsabstand 56 mm |
| Durchgangsbohrung inklusive Zugabe | Ø 4,7 mm |
| 90°-Senkung inklusive Zugabe | Ø 9,2 mm |
| Verstärkungsrippe | 4,2 mm breit |

Die Platte liegt mit ihrer Rückseite bei Z = 0. Der Haken entsteht durch einen Sweep mit variablem Querschnitt mit gerundeter Spitze. Die Rippe und der Arm sind mit der durchbohrten Platte vereinigt.

**Bambu Studio 02.09.01.70** hat beide exportierten Dateien mit --info geprüft. Für beide meldete es 32 × 78 × 42,889664 mm, 2860 Dreiecke, **manifold = yes** und **number_of_parts = 1**. Die Volumina beider Importwege stimmen bis auf die Float-Rundung überein. Der CLI-Bericht gibt für 3MF lokale, zentrierte Koordinaten aus. Es wurde kein Druckauftrag ausgelöst.

Zusätzlich prüft ein Rust-Test die tatsächlich exportierte binäre STL nach dem Wiedereinlesen auf offene oder falsch orientierte Kanten, degenerierte Dreiecke und Selbstschnitt nicht benachbarter Flächen. Der 3MF-Test liest die Modellressource aus dem ZIP, prüft Einheit, Build-Objekt, Topologie und Selbstschnitt und vergleicht jedes Dreieck mit der STL im Rahmen der f32-Darstellung. Weitere Tests prüfen die beiden Senkringe, die Topologie eines Körpers mit zwei Durchgangslöchern, das Ändern der Plattenbreite, Undo/Redo und den Wiederaufbau aus R3DP.

## Anwendung und Builds

- Linux-Fenster mit 3D-Darstellung im virtuellen X11-Desktop geöffnet und kontrolliert. Plattenbreite im Inspektor von 32 auf 36 mm gezogen, übernommen und gespeichert. Die gespeicherte Datei enthielt 36 mm als Quellparameter und abgeleitete Breite. Nach Undo und erneutem Speichern betrug die Breite wieder 32 mm.
- cargo fmt --all --check, cargo check --workspace --all-targets --locked, cargo clippy --workspace --all-targets --locked -- -D warnings, cargo test --workspace --locked und cargo build --workspace --release --locked erfolgreich. Insgesamt 26 Tests.
- Zusätzlicher Release-Build für x86_64-pc-windows-gnu und NSIS-x64-Installer. Lauf auf einem Windows-Gerät und Installationsabnahme stehen aus.
- Reproduzieren mit cargo run --release --example wandhaken --locked im Projektordner. Dieser Befehl aktualisiert die vier benannten Beispieldateien in examples/modelle.

Die Hakenabnahme betrifft digitale Geometrie und Austauschformate. Material, Montagerichtung, Schrauben, Verankerung und Lastfall wurden mechanisch nicht geprüft.

# Abnahmeprotokoll · 0.1.0 · 20. November 2026

Ausgeführt auf Linux x86_64 mit Rust 1.88.0. Alle fünf Cargo-Prüfungen liefen mit Erfolg.

```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --release --locked
```

Neun Tests prüfen geschlossene und orientierte Körper einschließlich Bohrung, Transformation, Profilgrenzen, Sperrfläche, Undo/Redo-Verzweigung, Projekt-Roundtrip und sicheren Dateiersatz, beschädigte Eingaben, Exportstrukturen und Auswahl des vordersten Körpers bei beiden Projektionen.

Das Linux-Release öffnete das Beispiel Bohrquader in einem virtuellen X11-Fenster mit Mesa-Rendering. Oberfläche, Druckbett, Sperrfläche, Bohrung und Auswahlrahmen wurden im Fenster kontrolliert. Ein Klick auf Quader fügte 16 Dreiecke hinzu. Rechtsziehen und Mausrad änderten die Projektion im Bild.

## Bambu Studio

Bambu Studio 02.09.01.70 las jede der sechs mitgelieferten Austauschdateien mit **--info**. Für STL und 3MF kam jeweils derselbe Befund zurück.

| Beispiel | Größe X × Y × Z in mm | Dreiecke | Teile | Manifold |
|---|---|---:|---:|---|
| Wuerfel | 20 × 20 × 20 | 16 | 1 | yes |
| Bohrquader | 55 × 32 × 16 | 384 | 1 | yes |
| Profilgrenze | 20 × 20 × 251 | 16 | 1 | yes |

Das Volumen des Würfels beträgt laut Bambu Studio 8000 mm³, das des Bohrquaders 27358,043 mm³. Der Grenzfall demonstriert den Unterschied zwischen geschlossener Oberfläche und erlaubter Druckhöhe. Bei 3MF meldet --info zentrierte lokale Extremkoordinaten. Absolute Plattenplatzierung in der Bambu-Oberfläche, Slicing und physischer Druck wurden nicht abgenommen.

## Wiederholen

1. `cargo run --release --example abnahme --locked -- Abnahmemodelle` erzeugt die drei Modellvarianten in einem neuen Ordner und schützt bereits bestehende Verzeichnisse.
2. Ein R3DP-Beispiel im Studio öffnen, einen Parameter ändern, übernehmen und unter neuem Namen speichern. Datei erneut öffnen und Maß prüfen.
3. Bohrquader als STL und 3MF exportieren. Mit `bambu-studio --info Modell.3mf` und entsprechend für STL prüfen. In der grafischen Oberfläche zusätzlich Position und Druckorientierung kontrollieren.
4. Profilgrenze.r3dp öffnen. Es muss eine Bereichswarnung erscheinen. Höhe auf 250 mm setzen und übernehmen. Die Warnung verschwindet.

Das Sperrrechteck 0…18 mm in X und 0…28 mm in Y wurde mit dem P1S-0,4-mm-Profil (GM014) im getesteten Bambu-Studio-Paket verglichen. [Bambu-Studio-Release](https://github.com/bambulab/BambuStudio/releases/tag/v02.09.01.70)

## Slicerübergabe 0.4

Geprüft mit der offiziellen Bambu-Studio-CLI 02.09.01.70 unter Linux. Zwei Würfel, jeweils 20 mm Kantenlänge, wurden mit Mittelpunkten bei (40, 50, 10) und (202, 191, 10) mm auf einer Platte übernommen. Der erneute Projektexport enthielt beide Namen und Filamentnummern, die beiden gewählten Filamentprofile, 0,16 mm Schichthöhe, vier Wandlinien, 27 % Füllung und aktivierte Stützen. Die Prüfung betrifft das ungeslicte Projekt und erteilt keine Druckfreigabe.

Wiederholung mit Rust, Python 3, einem funktionierenden Bambu-Studio-Programm und einem eigenen P1S-Projekt mit 0,4-mm-Düse und zwei Filamenten:

```sh
python3 scripts/bambu_abnahme.py /pfad/zu/bambu-studio /pfad/zur/Vorlage.3mf
```

## 0.5 – Offline-Slicing

Unter Linux x86-64 blockierte ein für den Test vorgeschalteter Prozessfilter IPv4- und IPv6-Sockets für Anwendung und Kindprozesse. Die offizielle CLI 02.09.01.70 erzeugte mit lokaler P1S-Vorlage und einem Filament beim Würfel 100 Schichten (15 Minuten geschätzt), beim Wandhaken 214 Schichten (31 Minuten geschätzt). Für den Haken meldete der Slicer frei schwebende Bereiche. Beide Dateien dienten ausschließlich zur Softwareabnahme. Bei einer Zweifilament-Testvorlage beendete sich die externe Engine während der Verarbeitung ihrer Extruderkonfiguration mit einem Speicherfehler.

Die eigene Rust-Berechnung des Wandhakens ergab bei gleichmäßigen 0,20 mm und 15 % Linienfüllung 215 geometrische Schnitte. Sämtliche SVG-Dateien wurden als XML eingelesen. Die Schichteinteilung der Druckdatei richtet sich separat nach dem Herstellerprofil. Die Tests prüfen zusätzlich Innenaussparungen, Richtungswechsel, offene Schnittlinien und vollständige Stapel.

```sh
cargo run --locked --example schichtplan -- examples/modelle/Wandhaken.r3dp Plan.zip
cargo run --locked --example offline_slicen -- /pfad/bambu-studio examples/modelle/Wuerfel.r3dp /pfad/P1S.3mf Wuerfel.gcode.3mf
```

Der Öffnungs- und Speicherablauf für die Rust-Schichtplanung wurde außerdem im Linux-Fenster am Wandhaken bedient. Der GUI-Export enthielt die erwarteten 215 Schichten.
