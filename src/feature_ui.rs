use super::ui::vector;
use egui_macroquad::egui::{self, Ui};
use glam::{DVec2, DVec3};
use rust_3d_print_studio::{
    cad::{curve::Curve, ellipse, Feature, Hook, Operation},
    geometry::Mesh,
};
pub fn templates() -> Vec<(&'static str, Feature)> {
    let rect = vec![
        DVec3::new(-12., -8., 0.),
        DVec3::new(12., -8., 0.),
        DVec3::new(12., 8., 0.),
        DVec3::new(-12., 8., 0.),
    ];
    let b = vec![
        DVec3::ZERO,
        DVec3::new(0., 0., 22.),
        DVec3::new(0., 25., 30.),
        DVec3::new(0., 32., 16.),
    ];
    let sweep = |path| Feature::Sweep {
        path,
        profile: ellipse(4., 4., 24),
        steps: 20,
        up: DVec3::X,
        end_scale: 0.8,
        twist: 0.,
        round_tip: true,
        closed: false,
    };
    vec![
        (
            "Skizzenkontur",
            Feature::SketchExtrude {
                sketch: Default::default(),
                height: 18.,
                end_scale: 1.,
                steps: 24,
            },
        ),
        (
            "3D-Skizzenpfad",
            sweep(Curve::Sketch(rust_3d_print_studio::cad::sketch::Sketch {
                points: vec![DVec3::ZERO, DVec3::Z * 18., DVec3::new(0., 12., 30.)],
                planar: false,
                segments: vec![
                    rust_3d_print_studio::cad::sketch::Segment {
                        kind: rust_3d_print_studio::cad::sketch::SegmentKind::Line,
                        ids: [0, 1, 0, 0],
                        construction: false,
                    },
                    rust_3d_print_studio::cad::sketch::Segment {
                        kind: rust_3d_print_studio::cad::sketch::SegmentKind::Line,
                        ids: [1, 2, 0, 0],
                        construction: false,
                    },
                ],
                ..Default::default()
            })),
        ),
        (
            "Flächenverbund",
            Feature::Sewn {
                patches: rust_3d_print_studio::cad::surfaces::cube_patches(),
                tolerance: 0.001,
            },
        ),
        ("J-Wandhaken", Feature::Hook(Hook::default())),
        ("Bézier-Sweep", sweep(Curve::Bezier(b.clone()))),
        (
            "Polylinien-Sweep",
            sweep(Curve::Polyline(vec![b[0], b[1], b[2]])),
        ),
        (
            "B-Spline / NURBS",
            sweep(Curve::Nurbs {
                points: b,
                weights: vec![1.; 4],
                degree: 3,
                knots: vec![0., 0., 0., 0., 1., 1., 1., 1.],
            }),
        ),
        (
            "Ellipsenbogen",
            sweep(Curve::Arc {
                center: DVec3::Z * 16.,
                u: DVec3::Y * 22.,
                v: DVec3::Z * 16.,
                start: -90.,
                angle: 170.,
            }),
        ),
        (
            "Extrusion",
            Feature::Extrude {
                profile: rect.clone(),
                direction: DVec3::Z * 24.,
                end_scale: 0.8,
            },
        ),
        (
            "Loft",
            Feature::Loft {
                rings: vec![
                    rect.clone(),
                    rect.iter()
                        .map(|p| *p * 0.6 + DVec3::new(3., 3., 12.))
                        .collect(),
                    rect.iter()
                        .map(|p| *p * 0.9 + DVec3::new(8., 2., 26.))
                        .collect(),
                ],
            },
        ),
        (
            "Rotationskörper",
            Feature::Revolve {
                profile: vec![
                    DVec3::new(8., 0., 0.),
                    DVec3::new(15., 0., 0.),
                    DVec3::new(13., 0., 20.),
                    DVec3::new(8., 0., 22.),
                ],
                origin: DVec3::ZERO,
                axis: DVec3::Z,
                angle: 360.,
                steps: 36,
            },
        ),
    ]
}
fn num(ui: &mut Ui, label: &str, value: &mut f64) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value).speed(0.1).max_decimals(3));
    });
}
fn point(ui: &mut Ui, label: &str, p: &mut DVec3) {
    vector(ui, label, p, -2000.0..=2000.);
}
fn integer(ui: &mut Ui, label: &str, value: &mut usize, min: usize, max: usize) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value).range(min..=max));
    });
}
fn points(ui: &mut Ui, pts: &mut Vec<DVec3>, allow_len: bool) {
    edit_points(ui, pts, allow_len);
}
fn edit_points(ui: &mut Ui, pts: &mut Vec<DVec3>, allow_len: bool) -> Option<usize> {
    if pts.is_empty() {
        return None;
    }
    let id = ui.id().with("skizzenzustand");
    let (mut index, mut plane, mut extent) = ui
        .data(|d| d.get_temp::<(usize, usize, f32)>(id))
        .unwrap_or_else(|| {
            let span = pts
                .iter()
                .copied()
                .fold(DVec3::splat(f64::NEG_INFINITY), DVec3::max)
                - pts
                    .iter()
                    .copied()
                    .fold(DVec3::splat(f64::INFINITY), DVec3::min);
            (
                0,
                if span.z <= span.y.min(span.x) {
                    0
                } else if span.y <= span.x {
                    1
                } else {
                    2
                },
                55.,
            )
        });
    index = index.min(pts.len() - 1);
    ui.horizontal(|ui| {
        for (i, name) in ["XY", "XZ", "YZ"].iter().enumerate() {
            ui.selectable_value(&mut plane, i, *name);
        }
    });
    ui.add(egui::Slider::new(&mut extent, 5.0..=200.).text("Ausschnitt ±mm"));
    let (rect, res) = ui.allocate_exact_size(
        egui::vec2(ui.available_width().min(260.), 145.),
        egui::Sense::click_and_drag(),
    );
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 4., egui::Color32::from_rgb(25, 33, 45));
    let (a, b) = [(0, 1), (0, 2), (1, 2)][plane];
    let center = rect.center();
    let scale = rect.height() * 0.46 / extent;
    let screen = |p: DVec3| center + egui::vec2(p[a] as f32, -p[b] as f32) * scale;
    let grid = egui::Stroke::new(1., egui::Color32::from_gray(75));
    painter.hline(rect.x_range(), center.y, grid);
    painter.vline(center.x, rect.y_range(), grid);
    for p in pts.windows(2) {
        painter.line_segment(
            [screen(p[0]), screen(p[1])],
            egui::Stroke::new(1., egui::Color32::LIGHT_BLUE),
        );
    }
    for (i, p) in pts.iter().enumerate() {
        painter.circle_filled(
            screen(*p),
            if i == index { 4.5 } else { 3. },
            if i == index {
                egui::Color32::GOLD
            } else {
                egui::Color32::WHITE
            },
        );
    }
    if let Some(pos) = res.interact_pointer_pos() {
        if res.clicked() || res.drag_started() {
            let at = if res.drag_started() {
                ui.input(|i| i.pointer.press_origin()).unwrap_or(pos)
            } else {
                pos
            };
            if let Some((k, _)) = pts.iter().enumerate().min_by(|(_, p), (_, q)| {
                screen(**p)
                    .distance(at)
                    .total_cmp(&screen(**q).distance(at))
            }) {
                index = k;
            }
        }
        if res.dragged() {
            pts[index][a] = f64::from((pos.x - center.x) / scale);
            pts[index][b] = f64::from((center.y - pos.y) / scale);
        }
    }
    integer(ui, "Punkt (ab 0)", &mut index, 0, pts.len() - 1);
    point(ui, "Kontrollpunkt · mm", &mut pts[index]);
    if allow_len {
        ui.horizontal(|ui| {
            if pts.len() < 64 && ui.button("+ Punkt").clicked() {
                let p = pts[index];
                pts.insert(index + 1, p + DVec3::new(2., 2., 0.));
                index += 1;
            }
            if pts.len() > 2 && ui.button("Entfernen").clicked() {
                pts.remove(index);
                index = index.min(pts.len() - 1);
            }
        });
    }
    ui.data_mut(|d| d.insert_temp(id, (index, plane, extent)));
    Some(index)
}
pub fn edit(ui: &mut Ui, f: &mut Feature) {
    match f {
        Feature::Sewn { patches, tolerance } => {
            num(ui, "Nahttoleranz", tolerance);
            for (i, patch) in patches.iter_mut().enumerate() {
                ui.collapsing(format!("Fläche {}", i + 1), |ui| {
                    for (j, row) in patch.iter_mut().enumerate() {
                        ui.collapsing(format!("Reihe {}", j + 1), |ui| points(ui, row, true));
                    }
                    if ui.button("Fläche verlängern").clicked() && patch.len() >= 2 {
                        let n = patch.len();
                        patch.push(
                            patch[n - 1]
                                .iter()
                                .zip(&patch[n - 2])
                                .map(|(a, b)| 2. * *a - *b)
                                .collect(),
                        );
                    }
                });
            }
        }
        Feature::Case(c) => {
            vector(ui, "Innenmaße", &mut c.inside, 4.0..=500.);
            for (label, v) in [
                ("Wandstärke", &mut c.wall),
                ("Spiel je Seite", &mut c.play),
                ("Einstecktiefe", &mut c.lip),
            ] {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.add(egui::DragValue::new(v).speed(0.1).suffix(" mm"));
                });
            }
        }
        Feature::Relief(r) => {
            if let rust_3d_print_studio::cad::accessories::Art::Text { value, mono } = &mut r.source
            {
                ui.text_edit_singleline(value);
                ui.checkbox(mono, "Schrift mit fester Breite");
            }
            for (label, v) in [
                ("Schrifthöhe / Logobreite", &mut r.size),
                ("Rand", &mut r.border),
                ("Plattenstärke", &mut r.base),
                ("Reliefhöhe / Tiefe", &mut r.depth),
            ] {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.add(egui::DragValue::new(v).speed(0.1).suffix(" mm"));
                });
            }
            ui.checkbox(&mut r.engraved, "Gravur");
            ui.checkbox(&mut r.hole, "Öse mit 3,2 mm Bohrung");
        }
        Feature::Hook(h) => {
            egui::ComboBox::from_label("Hakentyp")
                .selected_text(h.style.label())
                .show_ui(ui, |ui| {
                    for style in rust_3d_print_studio::cad::HookStyle::ALL {
                        if ui
                            .selectable_label(h.style == style, style.label())
                            .clicked()
                        {
                            *h = Hook::preset(style);
                        }
                    }
                });
            ui.checkbox(&mut h.flat_tip, "Flache Spitze");
            ui.checkbox(&mut h.rectangular, "Rechteckprofil");

            egui::CollapsingHeader::new("Befestigungsplatte")
                .default_open(true)
                .show(ui, |ui| {
                    num(ui, "Höhe", &mut h.height);
                    num(ui, "Breite", &mut h.width);
                    num(ui, "Dicke", &mut h.thickness);
                    num(ui, "Eckradius", &mut h.corner);
                    num(ui, "Kantenradius", &mut h.fillet);
                });
            ui.collapsing("Pfadskizze und Querschnitt", |ui| {
                num(ui, "Schenkellänge", &mut h.shank);
                num(ui, "Biegeradius", &mut h.radius);
                num(ui, "Material-Ø", &mut h.diameter);
                num(ui, "Ovalität", &mut h.oval);
                num(ui, "Spitzenlänge", &mut h.tip_length);
                num(ui, "Spitzenskalierung", &mut h.tip_scale);
                num(ui, "Öffnungswinkel °", &mut h.tip_angle);
                integer(ui, "Tessellierung", &mut h.steps, 8, 64);
            });
            ui.collapsing("Senkbohrungen und Rippe", |ui| {
                integer(ui, "Lochanzahl", &mut h.holes, 0, 4);
                num(ui, "Mittenabstand", &mut h.spacing);
                num(ui, "Bohrungs-Ø", &mut h.drill);
                num(ui, "Senkungs-Ø", &mut h.sink);
                num(ui, "Durchmesserzugabe", &mut h.clearance);
                num(ui, "Rippenbreite", &mut h.rib);
            });
            if ui.button("In einzelne Features auflösen").clicked() {
                match h.features() {
                    Ok(tree) => *f = tree,
                    Err(e) => {
                        ui.label(e);
                    }
                }
            }
        }
        Feature::Sweep {
            path,
            profile,
            steps,
            up,
            end_scale,
            twist,
            round_tip,
            closed,
        } => {
            ui.collapsing("Pfadskizze", |ui| match path {
                Curve::Sketch(s) => sketch_ui(ui, s),
                Curve::Polyline(p) => points(ui, p, true),
                Curve::Bezier(p) => {
                    points(ui, p, false);
                    if p.len() < 62 && ui.button("Béziersegment anfügen").clicked() {
                        let a = p[p.len() - 1];
                        let dir = (a - p[p.len() - 2]).normalize_or_zero() * 5.;
                        p.extend([a + dir, a + dir * 2., a + dir * 3.]);
                    }
                }
                Curve::Nurbs {
                    points: p,
                    weights,
                    degree,
                    knots,
                } => {
                    points(ui, p, false);
                    integer(ui, "Grad", degree, 1, 5);
                    for (i, w) in weights.iter_mut().enumerate() {
                        num(ui, &format!("Gewicht {i}"), w);
                    }
                    ui.collapsing("Knoten", |ui| {
                        for v in knots {
                            num(ui, "Knoten", v)
                        }
                    });
                }
                Curve::Arc {
                    center,
                    u,
                    v,
                    start,
                    angle,
                } => {
                    point(ui, "Mittelpunkt", center);
                    point(ui, "Halbachse U", u);
                    point(ui, "Halbachse V", v);
                    num(ui, "Startwinkel", start);
                    num(ui, "Bogenwinkel", angle);
                }
            });
            ui.collapsing("Querschnitt", |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Rund").clicked() {
                        *profile = ellipse(4., 4., 24)
                    }
                    if ui.button("Elliptisch").clicked() {
                        *profile = ellipse(6., 3., 24)
                    }
                    if ui.button("Rechteck").clicked() {
                        *profile = vec![
                            DVec2::new(-4., -3.),
                            DVec2::new(4., -3.),
                            DVec2::new(4., 3.),
                            DVec2::new(-4., 3.),
                        ]
                    }
                });
                let mut pp: Vec<_> = profile.iter().map(|v| v.extend(0.)).collect();
                points(ui, &mut pp, true);
                *profile = pp.iter().map(|p| p.truncate()).collect();
            });
            point(ui, "Profilachse", up);
            num(ui, "Endskalierung", end_scale);
            num(ui, "Verdrehung °", twist);
            integer(ui, "Auflösung", steps, 4, 96);
            ui.checkbox(round_tip, "Gerundete Spitze");
            ui.checkbox(closed, "Pfad schließen");
        }
        Feature::SketchExtrude {
            sketch,
            height,
            end_scale,
            steps,
        } => {
            sketch_ui(ui, sketch);
            num(ui, "Höhe", height);
            num(ui, "Endskalierung", end_scale);
            integer(ui, "Auflösung", steps, 4, 96);
        }
        Feature::Shell {
            source,
            thickness,
            open,
            side,
        } => {
            num(ui, "Wandstärke", thickness);
            ui.checkbox(open, "Öffnungsfläche verwenden");
            if *open {
                use rust_3d_print_studio::cad::references::Side;
                let name = Side::ALL
                    .iter()
                    .find(|(s, _)| Some(*s) == *side)
                    .map_or("Oben (Altprojekt)", |(_, s)| *s);
                egui::ComboBox::from_id_salt(ui.id().with("seite"))
                    .selected_text(name)
                    .show_ui(ui, |ui| {
                        for (s, label) in Side::ALL {
                            ui.selectable_value(side, Some(s), label);
                        }
                    });
            }
            child(ui, "Außenkörper", source);
        }
        Feature::Cut {
            source,
            point: p,
            normal,
        } => {
            point(ui, "Ebenenpunkt", p);
            point(ui, "Normale", normal);
            child(ui, "Quelle", source);
        }
        Feature::Lattice {
            source,
            offsets,
            symmetric,
        } => {
            ui.checkbox(symmetric, "X-Symmetrie");
            for (i, v) in offsets.iter_mut().enumerate() {
                if !*symmetric || i % 2 == 0 {
                    ui.collapsing(format!("Gitterpunkt {}", i + 1), |ui| {
                        point(ui, "Versatz", v)
                    });
                }
            }
            child(ui, "Quelle", source);
        }
        Feature::Driven {
            source,
            variables,
            bindings,
        } => {
            let mut rename = None;
            for (k, v) in variables.iter_mut() {
                ui.horizontal(|ui| {
                    let id = ui.id().with(k);
                    let mut name = ui
                        .data(|d| d.get_temp::<String>(id))
                        .unwrap_or_else(|| k.clone());
                    if ui.text_edit_singleline(&mut name).lost_focus() && name != *k {
                        rename = Some((k.clone(), name.clone()));
                    }
                    ui.data_mut(|d| d.insert_temp(id, name));
                    ui.add(egui::DragValue::new(v));
                });
            }
            if let Some((old, new)) = rename {
                if !new.is_empty() && !variables.contains_key(&new) {
                    if let Some(v) = variables.remove(&old) {
                        variables.insert(new, v);
                    }
                }
            }
            if ui.button("+ Variable").clicked() {
                variables.insert(format!("mass{}", variables.len() + 1), 10.);
            }
            for b in bindings.iter_mut() {
                ui.label("Zielpfad");
                ui.text_edit_singleline(&mut b.target);
                ui.label("Formel");
                ui.text_edit_singleline(&mut b.expression);
            }
            if ui.button("+ Maßbeziehung").clicked() {
                bindings.push(rust_3d_print_studio::cad::dimensions::Binding {
                    target: "/Hook/width".into(),
                    expression: "mass1 * 2".into(),
                });
            }
            ui.collapsing("Parameterpfade ansehen", |ui| {
                if let Ok(t) = serde_json::to_string_pretty(source) {
                    ui.monospace(t.chars().take(4000).collect::<String>());
                }
            });
            child(ui, "Quelle", source);
        }
        Feature::SmoothLoft {
            rings,
            steps,
            guide,
        } => {
            integer(ui, "Zwischenschritte", steps, 2, 24);
            for (i, ring) in rings.iter_mut().enumerate() {
                ui.collapsing(format!("Profil {}", i + 1), |ui| points(ui, ring, true));
            }
            for (i, p) in guide.iter_mut().enumerate() {
                point(ui, &format!("Führung im Intervall {}", i + 1), p);
            }
        }
        Feature::Loft { rings } => {
            if ui.button("Glatter Loft mit Führung").clicked() {
                *f = Feature::SmoothLoft {
                    guide: vec![DVec3::ZERO; rings.len() - 1],
                    steps: 8,
                    rings: rings.clone(),
                };
                return;
            }

            for (i, ring) in rings.iter_mut().enumerate() {
                ui.collapsing(format!("Profil {}", i + 1), |ui| points(ui, ring, true));
            }
            if rings.len() < 32 && ui.button("Profil ergänzen").clicked() {
                if let Some(r) = rings.last() {
                    rings.push(r.iter().map(|p| *p + DVec3::Z * 8.).collect());
                }
            }
        }
        Feature::Extrude {
            profile,
            direction,
            end_scale,
        } => {
            ui.collapsing("Geschlossene Kontur", |ui| points(ui, profile, true));
            point(ui, "Extrusionsvektor", direction);
            num(ui, "Endskalierung", end_scale);
        }
        Feature::Revolve {
            profile,
            origin,
            axis,
            angle,
            steps,
        } => {
            ui.collapsing("Profil", |ui| points(ui, profile, true));
            point(ui, "Achsenpunkt", origin);
            point(ui, "Achsenrichtung", axis);
            num(ui, "Winkel °", angle);
            integer(ui, "Auflösung", steps, 4, 96);
        }
        Feature::Primitive { shape, size, bore } => {
            point(ui, "Abmessungen", size);
            if shape.hollow() {
                num(ui, "Bohrung Ø", bore);
            }
        }
        Feature::Boolean {
            left,
            right,
            operation,
        } => {
            ui.horizontal_wrapped(|ui| {
                for (op, s) in [
                    (Operation::Union, "Union"),
                    (Operation::Difference, "Differenz"),
                    (Operation::Intersection, "Schnittmenge"),
                ] {
                    ui.selectable_value(operation, op, s);
                }
            });
            child(ui, "Grundkörper", left);
            child(ui, "Werkzeugkörper", right);
        }
        Feature::Transform {
            source,
            translation,
            rotation,
            scale,
        } => {
            point(ui, "Verschiebung", translation);
            point(ui, "Rotation °", rotation);
            point(ui, "Skalierung", scale);
            child(ui, "Quelle", source);
        }
        Feature::Deform {
            source,
            twist,
            taper,
            bend,
        } => {
            num(ui, "Verdrehung °", twist);
            num(ui, "Endskalierung", taper);
            num(ui, "Biegung °", bend);
            child(ui, "Quelle", source);
        }
        Feature::EdgeFinish {
            source,
            edge,
            reference,
            r1,
            r2,
            steps,
        } => {
            let id = ui.id().with("kantenbezug");
            let mut candidate = ui.data(|d| d.get_temp::<usize>(id)).unwrap_or(*edge);
            integer(ui, "Kantennummer", &mut candidate, 0, 50000);
            ui.data_mut(|d| d.insert_temp(id, candidate));
            ui.label(if reference.is_some() {
                "Geometrischer Bezug gespeichert"
            } else {
                "Bezug über Kantennummer"
            });
            if ui.button("Kante geometrisch binden").clicked() {
                let result = source.build().and_then(|m| {
                    rust_3d_print_studio::cad::references::EdgeRef::bind(&m, candidate)
                });
                let message = match result {
                    Ok(r) => {
                        *reference = Some(r);
                        *edge = candidate;
                        "Kante gebunden. Parameter übernehmen.".into()
                    }
                    Err(e) => e,
                };
                ui.data_mut(|d| d.insert_temp(id.with("meldung"), message));
            }
            if reference.is_none() {
                *edge = candidate;
            }
            if let Some(msg) = ui.data(|d| d.get_temp::<String>(id.with("meldung"))) {
                ui.label(msg);
            }
            num(ui, "Anfangsmaß", r1);
            num(ui, "Endmaß", r2);
            integer(ui, "Facetten (1 = Fase)", steps, 1, 16);
            child(ui, "Quelle", source);
        }
        Feature::Polygon(m) => polygon(ui, m),
    }
}
fn child(ui: &mut Ui, label: &str, f: &mut Feature) {
    ui.collapsing(format!("{label} · {}", f.name()), |ui| edit(ui, f));
}
fn polygon(ui: &mut Ui, m: &mut Mesh) {
    if m.faces.is_empty() {
        return;
    }
    let id = ui.id().with("polygon");
    let mut ix = ui
        .data(|d| d.get_temp(id))
        .unwrap_or(0usize)
        .min(m.faces.len() - 1);
    integer(ui, "Fläche", &mut ix, 0, m.faces.len() - 1);
    ui.data_mut(|d| d.insert_temp(id, ix));
    let face = m.faces[ix];
    for (i, j) in face.into_iter().enumerate() {
        point(ui, &format!("Eckpunkt {}", i + 1), &mut m.vertices[j]);
    }
    let shift_id = id.with("delta");
    let mut delta = ui.data(|d| d.get_temp(shift_id)).unwrap_or(DVec3::ZERO);
    point(ui, "Versatz", &mut delta);
    ui.data_mut(|d| d.insert_temp(shift_id, delta));
    for (n, label) in [(2, "Kante 1–2 verschieben"), (3, "Fläche verschieben")] {
        if ui.button(label).clicked() {
            for &j in &face[..n] {
                m.vertices[j] += delta;
            }
        }
    }
    if ui.button("Netzreparatur vorbereiten").clicked() {
        let result = rust_3d_print_studio::cad::surfaces::repair(m, 0.0001);
        let status = match result {
            Ok(repaired) => {
                let msg = format!(
                    "Vorschau: {} → {} Punkte, {} → {} Dreiecke",
                    m.vertices.len(),
                    repaired.vertices.len(),
                    m.faces.len(),
                    repaired.faces.len()
                );
                *m = repaired;
                msg
            }
            Err(e) => e,
        };
        ui.data_mut(|d| d.insert_temp(id.with("bericht"), status));
    }
    if let Some(msg) = ui.data(|d| d.get_temp::<String>(id.with("bericht"))) {
        ui.label(msg);
    }
    if ui.button("Oberfläche glätten").clicked() {
        let mut sum = vec![DVec3::ZERO; m.vertices.len()];
        let mut count = vec![0; m.vertices.len()];
        for &[a, b, c] in &m.faces {
            for (i, j, k) in [(a, b, c), (b, a, c), (c, a, b)] {
                sum[i] += m.vertices[j] + m.vertices[k];
                count[i] += 2;
            }
        }
        for (i, v) in m.vertices.iter_mut().enumerate() {
            if count[i] > 0 {
                *v = v.lerp(sum[i] / f64::from(count[i]), 0.2);
            }
        }
    }
    if m.faces.len() < 12000 && ui.button("Dreiecke unterteilen").clicked() {
        let mut edges = std::collections::HashMap::new();
        let old = std::mem::take(&mut m.faces);
        for [a, b, c] in old {
            let mut mid = |i: usize, j: usize| {
                *edges.entry((i.min(j), i.max(j))).or_insert_with(|| {
                    let v = (m.vertices[i] + m.vertices[j]) * 0.5;
                    m.vertices.push(v);
                    m.vertices.len() - 1
                })
            };
            let (d, e, f) = (mid(a, b), mid(b, c), mid(c, a));
            m.faces.extend([[a, d, f], [d, b, e], [f, e, c], [d, e, f]]);
        }
    }
}

fn sketch_ui(ui: &mut Ui, s: &mut rust_3d_print_studio::cad::sketch::Sketch) {
    use rust_3d_print_studio::cad::sketch::*;
    ui.collapsing("Arbeitsebene", |ui| {
        point(ui, "Ursprung", &mut s.origin);
        point(ui, "U-Achse", &mut s.u);
        point(ui, "V-Achse", &mut s.v);
        ui.checkbox(&mut s.planar, "Auf Ebene binden");
    });
    ui.collapsing("Grafische Bemaßung", |ui| crate::dimension_ui::show(ui, s));
    if let Some(i) = edit_points(ui, &mut s.points, false) {
        let id = ui.id().with("punktmeldung");
        ui.horizontal_wrapped(|ui| {
            let action = if ui.small_button("+ Punkt").clicked() {
                Some(s.insert_point_after(i))
            } else if ui.small_button("Punkt entfernen").clicked() {
                Some(s.remove_unused_point(i))
            } else {
                None
            };
            if let Some(result) = action {
                ui.data_mut(|d| d.insert_temp(id, result.err().unwrap_or_default()));
            }
        });
        if let Some(msg) = ui
            .data(|d| d.get_temp::<String>(id))
            .filter(|s| !s.is_empty())
        {
            ui.label(msg);
        }
    }
    let max = s.points.len().saturating_sub(1);
    ui.collapsing("Kontur und Hilfslinien", |ui| {
        let mut radius_segment = None;
        let mut remove = None;
        for (i, seg) in s.segments.iter_mut().enumerate() {
            ui.push_id(i, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for (kind, label) in [
                        (SegmentKind::Line, "Linie"),
                        (SegmentKind::Arc, "Bogen"),
                        (SegmentKind::Bezier, "Bézier"),
                    ] {
                        ui.selectable_value(&mut seg.kind, kind, label);
                    }
                });
                let count = match seg.kind {
                    SegmentKind::Line => 2,
                    SegmentKind::Arc => 3,
                    SegmentKind::Bezier => 4,
                };
                ui.horizontal_wrapped(|ui| {
                    for v in &mut seg.ids[..count] {
                        ui.add(egui::DragValue::new(v).range(0..=max));
                    }
                });
                if seg.kind == SegmentKind::Arc && ui.small_button("Radius bemaßen").clicked() {
                    radius_segment = Some(i);
                }
                ui.checkbox(&mut seg.construction, "Hilfsgeometrie");
                if ui.small_button("Segment entfernen").clicked() {
                    remove = Some(i);
                }
            });
        }
        let id = ui.id().with("radiusmeldung");
        if let Some(i) = radius_segment {
            let message = s.dimension_arc(i).map_or_else(
                |e| e,
                |_| "Radius unter Bemaßung und Bedingungen bearbeiten.".to_string(),
            );
            ui.data_mut(|d| d.insert_temp(id, message));
        }
        if let Some(message) = ui.data(|d| d.get_temp::<String>(id)) {
            ui.small(message);
        }
        if let Some(i) = remove {
            s.segments.remove(i);
        }
        if ui.button("+ Segment").clicked() {
            s.segments.push(Segment {
                kind: SegmentKind::Line,
                ids: [0, 1, 0, 0],
                construction: false,
            });
        }
    });
    ui.collapsing("Bemaßung und Bedingungen", |ui| {
        ui.checkbox(&mut s.bidirectional, "Punkte gemeinsam lösen");
        ui.small(
            "Punktnummern beginnen bei 0. Fixierte Punkte behalten ihre eingegebenen Koordinaten.",
        );

        let mut remove = None;
        for (i, r) in s.rules.iter_mut().enumerate() {
            ui.push_id(i, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for (kind, label) in [
                        (RuleKind::Distance, "Abstand"),
                        (RuleKind::Angle, "Innenwinkel"),
                        (RuleKind::Fixed, "Fixieren"),
                        (RuleKind::Horizontal, "Horizontal"),
                        (RuleKind::Vertical, "Vertikal"),
                        (RuleKind::Perpendicular, "Senkrecht"),
                        (RuleKind::EqualLength, "Gleich lang"),
                        (RuleKind::Parallel, "Parallel"),
                        (RuleKind::Tangent, "Tangente"),
                        (RuleKind::Mirror, "Spiegelung"),
                        (RuleKind::Coincident, "Koinzident"),
                    ] {
                        if ui.selectable_value(&mut r.kind, kind, label).clicked()
                            && matches!(
                                kind,
                                RuleKind::Fixed
                                    | RuleKind::Horizontal
                                    | RuleKind::Vertical
                                    | RuleKind::Perpendicular
                                    | RuleKind::Angle
                                    | RuleKind::EqualLength
                            )
                        {
                            s.bidirectional = true;
                        }
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    let count = r.point_count();
                    for (label, v) in ["A", "B", "C", "D"].into_iter().zip(&mut r.ids[..count]) {
                        if r.kind == RuleKind::Tangent && label == "C" {
                            continue;
                        }
                        ui.label(label);
                        ui.add(egui::DragValue::new(v).range(0..=max));
                    }
                });
                if r.kind == RuleKind::Distance {
                    num(ui, "Maß", &mut r.value);
                } else if r.kind == RuleKind::Radius {
                    num(ui, "Bogenradius (mm)", &mut r.value);
                } else if r.kind == RuleKind::Angle {
                    num(ui, "Innenwinkel (°) · Scheitel B", &mut r.value);
                }
                if ui.small_button("Bedingung entfernen").clicked() {
                    remove = Some(i);
                }
            });
        }
        if let Some(i) = remove {
            s.rules.remove(i);
        }
        if ui.button("+ Bedingung").clicked() {
            s.rules.push(Rule {
                kind: RuleKind::Distance,
                ids: [0, 1, 0, 0],
                value: 28.,
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_sketch_entry_produces_a_valid_solid() {
        for (label, feature) in templates() {
            feature.build().unwrap_or_else(|e| panic!("{label}: {e}"));
        }
    }
}
