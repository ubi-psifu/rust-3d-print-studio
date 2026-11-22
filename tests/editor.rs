use glam::{DVec2, DVec3};
use rust_3d_print_studio::{
    cad::{
        accessories::{Art, Case, Relief},
        Feature,
    },
    files::{self, Output},
    geometry::{Object, Shape},
    layout::{Mode, Options, Scene},
    project::{History, Project},
};
fn plate() -> Project {
    let mut p = Project::default();
    p.objects
        .push(Object::new(Shape::Box, DVec3::new(60., 60., 0.)));
    p
}
#[test]
fn array_and_one_undo() {
    let mut p = plate();
    let mut hist = History::default();
    let original = p.clone();
    let scene = Scene::new(&p.objects, &[0]);
    let mut s = Options::default();
    let plan = scene.plan(&p, &s);
    assert!(plan.allowed());
    hist.record(p.clone());
    plan.apply(&mut p).unwrap();
    assert_eq!(p.objects.len(), 12);
    assert_eq!(p.objects[11].position, DVec3::new(132., 108., 0.));
    hist.undo(&mut p);
    assert_eq!(p, original);
    p.objects
        .push(Object::new(Shape::Box, DVec3::new(84., 60., 0.)));
    let scene = Scene::new(&p.objects, &[0]);
    assert!(!scene.plan(&p, &s).allowed());
    s.skipped.insert(1);
    assert!(scene.plan(&p, &s).allowed());
}
#[test]
fn edges_and_repeat() {
    let mut p = plate();
    let mut b = Object::new(Shape::Box, DVec3::new(110., 100., 0.));
    b.size.x = 32.;
    p.objects.push(b);
    let mut s = Options {
        mode: Mode::Align,
        reference: 1,
        edge: 0,
        ..Default::default()
    };
    Scene::new(&p.objects, &[0, 1])
        .plan(&p, &s)
        .apply(&mut p)
        .unwrap();
    assert_eq!(p.objects[0].position.x, 104.);
    s.mode = Mode::Distribute;
    Scene::new(&p.objects, &[0, 1])
        .plan(&p, &s)
        .apply(&mut p)
        .unwrap();
    let (a, b) = (p.objects[0].mesh().bounds(), p.objects[1].mesh().bounds());
    assert!(((a.0.x - b.1.x).max(b.0.x - a.1.x) - 4.).abs() < 1e-7);
    s.mode = Mode::Copy;
    s.step = DVec2::new(0., 24.);
    s.count = 2;
    Scene::new(&p.objects, &[1])
        .plan(&p, &s)
        .apply(&mut p)
        .unwrap();
    assert_eq!(p.objects.len(), 4);
}
#[test]
fn sign_b8_and_svg() {
    for engraved in [true, false] {
        let sign = Relief {
            source: Art::Text {
                value: "B8".into(),
                mono: false,
            },
            engraved,
            ..Default::default()
        };
        let m = Feature::Relief(sign).build().unwrap();
        let (a, b) = m.bounds();
        assert!(a.z.abs() < 1e-6);
        assert!((b.z - if engraved { 2.4 } else { 3.2 }).abs() < 1e-6);
        let mut logo = rust_3d_print_studio::svg::load(
            br#"<svg><path fill-rule="evenodd" d="M0 0H30V20H0Z M8 5h14v10H8Z"/></svg>"#,
        )
        .unwrap();
        logo.engraved = engraved;
        assert!(Feature::Relief(logo).build().is_ok());
    }
    assert!(rust_3d_print_studio::svg::load(br#"<svg><path d="M0 0L1 1"/></svg>"#).is_err());
}
#[test]
fn import_formats() {
    let mut p = plate();
    p.objects
        .push(Object::new(Shape::Cylinder, DVec3::new(108., 115., 4.)));
    let tmp = tempfile::tempdir().unwrap();
    for f in [Output::Stl, Output::Obj, Output::ThreeMf] {
        let path = tmp.path().join(format!("Platte.{}", f.ext()));
        files::save(&path, &p, f, false).unwrap();
        let objects = rust_3d_print_studio::import::load(&path, 1.).unwrap();
        assert_eq!(objects.len(), if f == Output::Stl { 1 } else { 2 });
        let hi = objects
            .iter()
            .map(|o| o.mesh().bounds().1)
            .reduce(DVec3::max)
            .unwrap();
        assert!((hi - DVec3::new(118., 125., 24.)).length() < 1e-4);
    }
}
#[test]
fn case_and_lid_geometry() {
    let project = files::load(std::path::Path::new(
        "examples/modelle/Gehaeuse-mit-Deckel.r3dp",
    ))
    .expect("Mitgeliefertes Gehäuseprojekt laden");
    assert_eq!(project.objects.len(), 2);
    let target = tempfile::tempdir().unwrap();
    let file = target.path().join("Gehaeuse.3mf");
    files::save(&file, &project, Output::ThreeMf, false).unwrap();
    assert_eq!(
        rust_3d_print_studio::import::load(&file, 1.).unwrap().len(),
        2
    );
    let c = Case::default();
    let housing = Feature::Case(c.clone()).build().unwrap();
    let (a, b) = housing.bounds();
    assert!((b - a - DVec3::new(56., 38., 20.)).length() < 1e-6);
    let lid = Feature::Case(Case { lid: true, ..c }).build().unwrap();
    let (la, lb) = lid.bounds();
    assert!((lb - la - DVec3::new(56., 38., 5.)).length() < 1e-6);
    let top: Vec<_> = lid
        .vertices
        .iter()
        .filter(|v| (v.z - 5.).abs() < 1e-7)
        .collect();
    assert!((top.iter().map(|v| v.x).fold(f64::NEG_INFINITY, f64::max) - 25.75).abs() < 1e-6);
}
