use glam::{DVec2, DVec3};
use rust_3d_print_studio::{
    cad::{
        curve::Curve,
        dimensions::Binding,
        inspect,
        sketch::{Rule, RuleKind, Sketch},
        validate, Feature, Hook, Operation,
    },
    files::{load, save, Output},
    geometry::{Mesh, Object, Shape},
    project::{History, Project},
};
fn cube() -> Feature {
    Feature::Primitive {
        shape: Shape::Box,
        size: DVec3::splat(20.),
        bore: 5.,
    }
}
fn vol(m: &Mesh) -> f64 {
    m.faces
        .iter()
        .map(|f| {
            let [a, b, c] = f.map(|j| m.vertices[j]);
            a.dot(b.cross(c)) / 6.
        })
        .sum()
}
#[test]
fn booleans_have_correct_volumes() {
    for (op, v) in [
        (Operation::Union, 12000.),
        (Operation::Difference, 4000.),
        (Operation::Intersection, 4000.),
    ] {
        let f = Feature::Boolean {
            left: Box::new(cube()),
            right: Box::new(Feature::Transform {
                source: Box::new(cube()),
                translation: DVec3::X * 10.,
                rotation: DVec3::ZERO,
                scale: DVec3::ONE,
            }),
            operation: op,
        };
        assert!((vol(&f.build().unwrap()) - v).abs() < 1e-5);
    }
}
#[test]
fn shell_cut_and_deform() {
    let shell = Feature::Shell {
        source: Box::new(cube()),
        thickness: 2.,
        open: true,
    }
    .build()
    .unwrap();
    assert!((vol(&shell) - (8000. - 16. * 16. * 18.)).abs() < 1e-4);
    let cut = Feature::Cut {
        source: Box::new(cube()),
        point: DVec3::Z * 10.,
        normal: DVec3::new(1., 0., 1.),
    }
    .build()
    .unwrap();
    assert!((vol(&cut) - 4000.).abs() < 1e-6);
    let mut v = [DVec3::ZERO; 8];
    v[7] = DVec3::X * 2.;
    let m = Feature::Lattice {
        source: Box::new(cube()),
        offsets: v,
        symmetric: false,
    }
    .build()
    .unwrap();
    assert_eq!(m.bounds().1.x, 12.);
}
#[test]
fn constrained_profile_and_dimensions() {
    let mut s = Sketch::default();
    s.rules.push(Rule {
        kind: RuleKind::Distance,
        ids: [0, 1, 0, 0],
        value: 30.,
    });
    let p = s.sample(16).unwrap();
    assert!((p[0].distance(p[1]) - 30.).abs() < 1e-8);
    s.rules.push(Rule {
        kind: RuleKind::Distance,
        ids: [0, 1, 0, 0],
        value: 32.,
    });
    assert!(s.sample(16).is_err());
    let f = Feature::Driven {
        source: Box::new(cube()),
        variables: [("w".into(), 22.)].into(),
        bindings: vec![Binding {
            target: "/Primitive/size/0".into(),
            expression: "(w+3)*2".into(),
        }],
    };
    let (lo, hi) = f.build().unwrap().bounds();
    assert!((hi.x - lo.x - 50.).abs() < 1e-8);
}
#[test]
fn nurbs_and_concave_extrusion() {
    let c = Curve::Nurbs {
        degree: 2,
        points: vec![DVec3::X, DVec3::new(1., 1., 0.), DVec3::Y],
        weights: vec![1., 0.5f64.sqrt(), 1.],
        knots: vec![0., 0., 0., 1., 1., 1.],
    };
    assert!(c
        .sample(16)
        .unwrap()
        .iter()
        .all(|p| (p.length() - 1.).abs() < 1e-9));
    let f = Feature::Extrude {
        profile: [(0., 0.), (4., 0.), (4., 1.), (1., 1.), (1., 4.), (0., 4.)]
            .map(|(x, y)| DVec3::new(x, y, 0.))
            .to_vec(),
        direction: DVec3::Z * 3.,
        end_scale: 1.,
    };
    assert!((vol(&f.build().unwrap()) - 21.).abs() < 1e-8);
}
#[test]
fn closed_sweep_and_rotated_profile() {
    let profile = vec![
        DVec2::new(-1., -1.),
        DVec2::new(1., -1.),
        DVec2::new(1., 1.),
        DVec2::new(-1., 1.),
    ];
    let f = Feature::Sweep {
        path: Curve::Arc {
            center: DVec3::ZERO,
            u: DVec3::X * 15.,
            v: DVec3::Y * 15.,
            start: 0.,
            angle: 360.,
        },
        profile,
        steps: 32,
        up: DVec3::Z,
        end_scale: 1.,
        twist: 0.,
        round_tip: false,
        closed: true,
    };
    f.build().unwrap();
}
#[test]
fn self_intersections_detected() {
    let mut m = cube().build().unwrap();
    let offset = m.vertices.len();
    let b = cube().build().unwrap();
    m.vertices
        .extend(b.vertices.iter().map(|p| *p + DVec3::new(6., 5., 4.)));
    m.faces
        .extend(b.faces.iter().map(|face| face.map(|i| i + offset)));
    assert!(validate(&m).is_ok());
    assert!(inspect::intersections(&m).is_err());
}
#[test]
fn edited_hook_roundtrip_and_v1_migration() {
    let mut p = load(std::path::Path::new("examples/modelle/Wandhaken.r3dp")).unwrap();
    let old = p.objects[0].mesh();
    let mut history = History::default();
    history.record(p.clone());
    if let Some(Feature::Hook(h)) = &mut p.objects[0].feature {
        h.width = 36.;
    }
    p.objects[0].rebuild().unwrap();
    assert_eq!(p.objects[0].size.x, 36.);
    assert!(history.undo(&mut p));
    assert_eq!(old, p.objects[0].mesh());
    assert!(history.redo(&mut p));
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("h.r3dp");
    save(&path, &p, Output::Project, false).unwrap();
    let read = load(&path).unwrap();
    assert_eq!(read.objects[0].size.x, 36.);
    assert_eq!(
        load(std::path::Path::new("examples/modelle/Wuerfel.r3dp"))
            .unwrap()
            .version,
        2
    );
}
#[test]
fn invalid_hook_keeps_last_result() {
    let mut o = Object::new(Shape::Box, DVec3::ZERO);
    let mut h = Hook::default();
    o.feature = Some(Feature::Hook(h.clone()));
    o.rebuild().unwrap();
    let before = o.mesh();
    h.spacing = 120.;
    o.feature = Some(Feature::Hook(h));
    assert!(o.rebuild().is_err());
    assert_eq!(before, o.mesh());
}
#[test]
fn hook_exported_stl_is_closed_and_intersection_free() {
    let data = include_bytes!("../examples/modelle/Wandhaken.stl");
    let mut m = Mesh::default();
    let mut map = std::collections::HashMap::new();
    for r in data[84..].chunks_exact(50) {
        let mut f = [0; 3];
        for (i, vertex) in f.iter_mut().enumerate() {
            let coord: [u32; 3] = std::array::from_fn(|j| {
                let k = 12 + 12 * i + 4 * j;
                u32::from_le_bytes(r[k..k + 4].try_into().unwrap())
            });
            *vertex = *map.entry(coord).or_insert_with(|| {
                let p = DVec3::from_array(coord.map(|x| f32::from_bits(x) as f64));
                m.vertices.push(p);
                m.vertices.len() - 1
            });
        }
        m.faces.push(f);
    }
    validate(&m).unwrap();
    inspect::intersections(&m).unwrap();
    let (a, b) = m.bounds();
    assert!(((b - a) - DVec3::new(32., 78., 42.889664)).length() < 1e-4);
}
#[test]
fn smooth_loft_reaches_stations() {
    let rings = [0., 8., 16.]
        .map(|z| {
            [(0., 0.), (10., 0.), (10., 10.), (0., 10.)]
                .map(|(x, y)| DVec3::new(x, y, z))
                .to_vec()
        })
        .to_vec();
    let m = Feature::SmoothLoft {
        rings,
        steps: 4,
        guide: vec![DVec3::X, DVec3::ZERO],
    }
    .build()
    .unwrap();
    assert!((vol(&m) - 1600.).abs() < 1e-6);
}
#[test]
fn empty_scene_stays_valid() {
    assert!(Project::default().valid());
}
#[test]
fn hook_presets_rebuild() {
    for style in rust_3d_print_studio::cad::HookStyle::ALL {
        Feature::Hook(Hook::preset(style))
            .build()
            .unwrap_or_else(|e| panic!("{style:?}: {e}"));
    }
}
#[test]
fn convex_variable_edge_finish() {
    for steps in [1, 6] {
        let f = Feature::EdgeFinish {
            source: Box::new(cube()),
            edge: 0,
            r1: 1.,
            r2: 3.,
            steps,
        };
        let m = f.build().unwrap();
        assert!(vol(&m) < 7999. && vol(&m) > 7800.);
    }
}
#[test]
fn sews_surface_patches_and_repairs_duplicates() {
    use rust_3d_print_studio::cad::surfaces::*;
    let m = sew(&cube_patches(), 0.001).unwrap();
    assert!((vol(&m) - 8000.).abs() < 1e-8);
    let mut broken = m.clone();
    broken.faces.push(m.faces[0]);
    broken.faces[2].swap(1, 2);
    broken.faces.push([0, 0, 0]);
    assert_eq!(repair(&broken, 0.001).unwrap().faces.len(), 12);
}
#[test]
fn wall_hook_has_two_through_holes_and_two_countersinks() {
    let p = load(std::path::Path::new("examples/modelle/Wandhaken.r3dp")).unwrap();
    let mesh = p.objects[0].mesh();
    let edges = mesh.faces.len() * 3 / 2;
    assert_eq!(
        mesh.vertices.len() as isize - edges as isize + mesh.faces.len() as isize,
        -2
    );
    for cy in [100., 156.] {
        for (z, r) in [(3.75, 2.35), (6., 4.6)] {
            let pts = mesh
                .vertices
                .iter()
                .filter(|p| {
                    (p.z - z).abs() < 1e-6 && ((p.x - 128.).hypot(p.y - cy) - r).abs() < 1e-6
                })
                .count();
            assert!(pts >= 12, "Fehlender Bohr- oder Senkring bei {cy}, {z}");
        }
    }
}
#[test]
fn exported_hook_3mf_matches_the_verified_stl() {
    use std::io::Read;
    let data = include_bytes!("../examples/modelle/Wandhaken.3mf");
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(data)).unwrap();
    let mut xml = String::new();
    zip.by_name("3D/3dmodel.model")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    assert!(xml.contains("unit=\"millimeter\""));
    assert_eq!(xml.matches("<item ").count(), 1);
    fn attr<'a>(tag: &'a str, key: &str) -> &'a str {
        tag.split_once(&format!("{key}=\""))
            .unwrap()
            .1
            .split('"')
            .next()
            .unwrap()
    }
    let mut mesh = Mesh::default();
    for part in xml.split('<') {
        if part.starts_with("vertex ") {
            mesh.vertices.push(DVec3::from_array(
                ["x", "y", "z"].map(|s| attr(part, s).parse().unwrap()),
            ));
        } else if part.starts_with("triangle ") {
            mesh.faces
                .push(["v1", "v2", "v3"].map(|s| attr(part, s).parse().unwrap()));
        }
    }
    validate(&mesh).unwrap();
    inspect::intersections(&mesh).unwrap();
    let stl = include_bytes!("../examples/modelle/Wandhaken.stl");
    assert_eq!(mesh.faces.len() * 50 + 84, stl.len());
    for (face, rec) in mesh.faces.iter().zip(stl[84..].chunks_exact(50)) {
        for (slot, component) in face
            .iter()
            .flat_map(|i| mesh.vertices[*i].to_array())
            .enumerate()
        {
            let k = 12 + slot * 4;
            assert_eq!(
                component as f32,
                f32::from_le_bytes(rec[k..k + 4].try_into().unwrap())
            );
        }
    }
}
