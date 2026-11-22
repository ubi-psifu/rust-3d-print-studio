//! Kontextverweise und durchsuchbare Kapitel aus dem eingebetteten Bedienhandbuch.
use egui_macroquad::egui;
use rust_3d_print_studio::{cad::Feature, layout::Mode};

const MANUAL: &str = include_str!("../USER_MANUAL.md");
pub const TOOLS: [&str; 8] = [
    "Auswählen und verschieben",
    "Grundkörper erstellen",
    "Skizze anlegen",
    "Objekt drehen",
    "Objekt skalieren",
    "Durchgangsbohrung",
    "Körper schneiden",
    "Abstand messen",
];
pub const GUIDE: &str = "Hilfe im Studio";
pub const FILE: &str = "Speichern und exportieren";
pub const RECOVER: &str = "Ungespeicherte Arbeit zurückholen";
pub const CASE: &str = "Gehäuse, Schrift und Relief";
pub const PLATE: &str = "Serien ausrichten und kopieren";
pub const PROFILE: &str = "Grenzen prüfen";
pub const IMPORT: &str = "Netze importieren";
pub const PREP: [&str; 4] = [
    "Druckprofil wählen",
    "Filamente zuordnen",
    "Schichten ansehen",
    "An den lokalen Slicer übergeben",
];

pub fn layout(mode: Mode) -> &'static str {
    match mode {
        Mode::Grid => "Rechteckmuster",
        Mode::Circle => "Kreismuster",
        Mode::Copy => "Kopierschritt",
        Mode::Move => "Versatz",
        Mode::Align => "Ausrichten",
        Mode::Distribute => "Verteilen",
    }
}
pub fn feature(f: &Feature) -> &'static str {
    match f {
        Feature::Case(_) | Feature::Relief(_) => CASE,
        Feature::SketchExtrude { .. } => "Abstände und Winkel grafisch bemaßen",
        Feature::Hook(_) => "Wandhaken und Konstruktionsbaum",
        Feature::Sweep { .. } => "Sweep",
        Feature::Loft { .. } | Feature::SmoothLoft { .. } => "Loft",
        Feature::Revolve { .. } => "Rotation",
        Feature::Boolean { .. } => "Boolesche",
        Feature::Cut { .. } => "Kanten und Schnitt",
        Feature::EdgeFinish { .. } => "Kanten und Öffnungsflächen binden",
        Feature::Driven { .. } => "Maßbeziehungen",
        Feature::Transform { .. } => TOOLS[4],
        Feature::Primitive { .. } => TOOLS[1],
        Feature::Extrude { .. } => "Extrusion",
        Feature::Shell { .. } => "Kanten und Öffnungsflächen binden",
        Feature::Lattice { .. }
        | Feature::Sewn { .. }
        | Feature::Deform { .. }
        | Feature::Polygon(_) => "Direkte Netzbearbeitung und Freiform",
    }
}

// Kurzhilfen ergänzen die ausführlichen Kapitel an den jeweiligen Bedienelementen.
const HINTS:&[(&str,&str)]=&[
(TOOLS[0],"Klicke auf ein Teil und ziehe es auf der Platte. Halte Umschalt für weitere Teile gedrückt. Mit X oder Y beschränkst du die Bewegung. Für Kopien drücke Strg+D oder öffne die Plattenwerkzeuge."),
(TOOLS[1],"Klicke auf einen Grundkörper. Gib im Inspektor rechts seine Maße in Millimetern ein und klicke auf Parameter übernehmen."),
(TOOLS[2],"Wähle eine Skizzenkontur oder einen 3D-Skizzenpfad. Ziehe Kontrollpunkte in der Zeichenfläche oder gib ihre Koordinaten rechts ein. Für eine Extrusion müssen die Segmente einen geschlossenen Umriss bilden."),
(TOOLS[3],"Markiere ein Objekt. Gib die Drehwinkel um X, Y und Z in Grad ein und klicke rechts auf Parameter übernehmen. Prüfe danach die Lage auf dem Druckbett."),
(TOOLS[4],"Bei einem Grundkörper gibst du die neuen Maße in Millimetern ein. Bei einem parametrischen Körper wählst du Skalierung anfügen und bearbeitest die drei Faktoren im Transformationsknoten. Faktor 1 erhält die Länge der jeweiligen Achse. Anschließend rechts übernehmen."),
(TOOLS[5],"Bohrquader erstellen erzeugt einen Quader mit veränderlichem Lochdurchmesser. Für eine Bohrung in einem vorhandenen Teil platziere einen Zylinder durch das Teil. Wähle das Teil, dann Körper abziehen und den Zylinder als Werkzeugkörper."),
(TOOLS[6],"Markiere zuerst den Grundkörper. Öffne Körperschnitt, wähle einen zweiten Körper und entscheide zwischen Vereinigung, Differenz und Schnittmenge. Bei Differenz wird das Volumen des zweiten Körpers vom Grundkörper abgezogen."),
(TOOLS[7],"Klicke nacheinander zwei Punkte auf Modellflächen an. Du siehst die räumliche Entfernung in mm und die Differenzen entlang X, Y und Z. Aktiviere Ecken / Kantenmitten fangen für reproduzierbare Messpunkte."),
(IMPORT,"Wähle Datei → Netz importieren. Gib den Pfad zu STL, OBJ, ASCII-PLY oder einer einfachen Modell-3MF ein. Für einheitenlose Netze wähle mm, cm oder Zoll nach der Quelldatei. Mit Zusammen auf der Platte absetzen bleiben die Teile relativ zueinander angeordnet."),
("Rechteckmuster","Wähle ein Teil und stelle Spalten, Reihen und den freien Abstand zwischen den Kanten ein. Das Original zählt mit. Klicke einzelne Kopierfelder in der Vorschau an, um sie auszulassen. Übernehmen ist möglich, sobald die verbleibenden Felder konfliktfrei sind."),
("Kreismuster","Stelle Anzahl und Radius ein. Der Kreismittelpunkt liegt vom ausgewählten Teil aus um den Radius nach links versetzt. Mitdrehen dreht die Kopien mit ihrer Winkelposition."),
("Kopierschritt","Gib die Anzahl der Kopien und den Versatz je Schritt in X und Y ein. Kontrolliere die vorgeschlagenen Positionen in der Vorschau und klicke auf Übernehmen."),
("Versatz","Gib die Änderung in X und Y in Millimetern ein. Die Vorschau zeigt die verschobene Auswahl. Ein negatives Vorzeichen bewegt sie in die entgegengesetzte Achsenrichtung."),
("Ausrichten","Wähle mehrere Teile. Bestimme Achse, Referenzteil und Minimum, Mitte oder Maximum. Die übrigen Teile richten ihre entsprechende Hüllboxkante oder Mitte am Referenzteil aus."),
("Verteilen","Wähle mehrere Teile und stelle die Achse und den gewünschten freien Kantenabstand ein. Überprüfe die neuen Positionen in der Vorschau, bevor du sie übernimmst."),
("Extrusion","Bearbeite den geschlossenen Profilumriss und die Extrusionshöhe. Die Endskalierung bestimmt die Größe des gegenüberliegenden Profils. Übernimm die Parameter, um die Geometrie neu zu berechnen."),
("Aushöhlung","Wähle den Ausgangskörper und stelle die Wandstärke ein. Prüfe nach der Neuberechnung das Netz. Zu große Wandstärken können an engen oder stark gekrümmten Stellen eine gültige Hülle verhindern."),
(PREP[0],"Lade eine eigene ungeslicte P1S-Projektvorlage aus Bambu Studio. Sie liefert Drucker-, Prozess- und Filamentprofile für die lokale Übergabe."),
(PREP[1],"Weise jedem Objekt einen Filamentplatz aus der Vorlage zu. Die tatsächliche Zuordnung zu den AMS-Fächern erfolgt später im Slicer."),
(PREP[2],"Stelle die Schnitthöhe ein. Für eine geometrische Schichtvorschau aktiviere die eigene Rust-Schichtplanung und wähle Schichthöhe und Füllung."),
(PREP[3],"Für eine Modellübergabe speichere eine 3MF-Datei. Zum lokalen Slicen wähle eine P1S-Vorlage, die installierte Bambu-Studio-Anwendung und einen neuen Ausgabepfad."),
];
fn normalized(s: &str) -> String {
    s.to_lowercase()
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss")
}
struct Article {
    title: &'static str,
    body: String,
    index: String,
}
impl Article {
    fn new(title: &'static str, body: String) -> Self {
        Self {
            title,
            index: normalized(&format!("{title} {body}")),
            body,
        }
    }
}
fn catalog(manual: &'static str) -> Vec<Article> {
    let mut entries: Vec<_> = HINTS
        .iter()
        .map(|&(t, b)| Article::new(t, b.into()))
        .collect();
    for chapter in manual.split("\n## ").skip(1) {
        let Some((title, body)) = chapter.split_once('\n') else {
            continue;
        };
        let title = title.trim();
        let mut content = Vec::new();
        for line in body.lines() {
            if line.starts_with('|') {
                let cells: Vec<_> = line.trim_matches('|').split('|').map(str::trim).collect();
                if cells.len() < 2
                    || cells[0].starts_with("---")
                    || matches!(
                        cells[0],
                        "Aktion" | "Werkzeug" | "Gewünschte Aktion" | "Format"
                    )
                {
                    continue;
                }
                let desc = if cells.len() == 3 {
                    format!("Öffnen: {}. Schreiben: {}.", cells[1], cells[2])
                } else {
                    cells[1..].join(" · ")
                };
                if title == "Skizzen und allgemeine Körper" {
                    entries.push(Article::new(cells[0], desc.clone()));
                }
                content.push(format!("{} · {}", cells[0], desc));
            } else if !line.trim().is_empty() {
                content.push(line.to_owned());
            }
        }
        entries.push(Article::new(title, content.join("\n\n")));
    }
    entries.sort_by_key(|a| a.title);
    entries
}
pub struct Help {
    entries: Vec<Article>,
    selected: usize,
    origin: usize,
    query: String,
    pub open: bool,
}
impl Default for Help {
    fn default() -> Self {
        Self {
            entries: catalog(MANUAL),
            selected: 0,
            origin: 0,
            query: String::new(),
            open: false,
        }
    }
}
impl Help {
    fn results(&self, query: &str) -> Vec<usize> {
        let terms = normalized(query);
        let mut matches: Vec<_> = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| terms.split_whitespace().all(|t| e.index.contains(t)))
            .map(|(i, _)| i)
            .collect();
        matches.sort_by_key(|&i| !normalized(self.entries[i].title).contains(terms.trim()));
        matches
    }
    pub fn show_topic(&mut self, title: &str) {
        self.origin = self
            .entries
            .iter()
            .position(|e| e.title == title)
            .unwrap_or(0);
        self.selected = self.origin;
        self.query.clear();
        self.open = true;
    }
    pub fn draw(&mut self, ctx: &egui::Context) {
        egui::Modal::new("offline_help".into()).show(ctx, |ui| {
            ui.set_width((ctx.screen_rect().width() - 48.).clamp(150., 620.));
            ui.horizontal(|ui| {
                ui.heading("Hilfe");
                if ui.button("Schließen").clicked() {
                    self.open = false;
                }
            });
            ui.add(
                egui::TextEdit::singleline(&mut self.query)
                    .hint_text("Anleitung durchsuchen …")
                    .desired_width(f32::INFINITY),
            );
            if !self.query.trim().is_empty() {
                let found = self.results(&self.query);
                let Some(&first) = found.first() else {
                    ui.label("Keine Treffer. Versuche ein anderes Stichwort.");
                    if ui.small_button("Suche löschen").clicked() {
                        self.query.clear();
                        self.selected = self.origin;
                    }
                    return;
                };
                if !found.contains(&self.selected) {
                    self.selected = first;
                }
                egui::ScrollArea::vertical()
                    .id_salt("hilfe_treffer")
                    .max_height(ctx.screen_rect().height() * 0.16)
                    .show(ui, |ui| {
                        for i in found {
                            if ui
                                .selectable_label(self.selected == i, self.entries[i].title)
                                .clicked()
                            {
                                self.selected = i;
                            }
                        }
                    });
            } else {
                egui::ComboBox::from_id_salt("hilfe_kapitel")
                    .selected_text(self.entries[self.selected].title)
                    .show_ui(ui, |ui| {
                        for (i, e) in self.entries.iter().enumerate() {
                            ui.selectable_value(&mut self.selected, i, e.title);
                        }
                    });
            }
            if self.selected != self.origin && ui.small_button("Zurück zur Kontexthilfe").clicked()
            {
                self.selected = self.origin;
                self.query.clear();
            }
            ui.separator();
            let a = &self.entries[self.selected];
            ui.strong(a.title);
            egui::ScrollArea::vertical()
                .id_salt(("hilfe_text", self.selected))
                .max_height(ctx.screen_rect().height() * 0.39)
                .show(ui, |ui| {
                    for p in a.body.split("\n\n") {
                        ui.label(p.replace("**", "").replace('`', ""));
                        ui.add_space(2.);
                    }
                });
        });
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.open = false;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_context_resolves_to_an_offline_article() {
        let mut h = Help::default();
        let mut titles = std::collections::HashSet::new();
        for e in &h.entries {
            assert!(titles.insert(e.title), "Doppelter Titel: {}", e.title);
            assert!(!e.body.trim().is_empty());
        }
        let modes = [
            Mode::Grid,
            Mode::Circle,
            Mode::Copy,
            Mode::Move,
            Mode::Align,
            Mode::Distribute,
        ];
        for key in TOOLS
            .into_iter()
            .chain(PREP)
            .chain([
                GUIDE,
                FILE,
                RECOVER,
                CASE,
                PLATE,
                PROFILE,
                IMPORT,
                "Boolesche",
                "Kanten und Schnitt",
                "Aushöhlung",
                "Maßbeziehungen",
                "Direkte Netzbearbeitung und Freiform",
            ])
            .chain(modes.map(layout))
            .chain(
                crate::feature_ui::templates()
                    .iter()
                    .map(|(_, f)| feature(f)),
            )
        {
            h.show_topic(key);
            assert_eq!(h.entries[h.selected].title, key);
        }
    }
    #[test]
    fn windows_line_endings_preserve_cad_context_topics() {
        let entries = catalog("# Hilfe\r\n\r\n## Skizzen und allgemeine Körper\r\n| Boolesche | Körper kombinieren |\r\n");
        assert!(entries
            .iter()
            .any(|article| article.title == "Boolesche" && article.body == "Körper kombinieren"));
    }

    #[test]
    fn search_matches_body_umlauts_and_multiple_words() {
        let mut h = Help::default();
        assert!(h
            .results("KOERPER VERSCHIEBEN")
            .iter()
            .any(|&i| h.entries[i].title == "Druckplatte belegen"));
        assert!(h.results("völligunbekanntesstichwort").is_empty());
        assert_eq!(h.entries[h.results("Kreismuster")[0]].title, "Kreismuster");
        h.query = "alter Filter".into();
        h.show_topic("Abstand messen");
        assert!(h.open && h.query.is_empty());
        assert_eq!(h.origin, h.selected);
    }
}
