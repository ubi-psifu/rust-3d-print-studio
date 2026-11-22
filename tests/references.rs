use glam::DVec3;
use rust_3d_print_studio::{
    cad::{
        references::{EdgeRef, Side},
        Feature,
    },
    files::{self, Output},
    geometry::{Object, Shape},
    project::Project,
};
fn block(size: DVec3) -> Feature {
    Feature::Primitive {
        shape: Shape::Box,
        size,
        bore: 4.,
    }
}
#[test]
fn edge_survives_vertex_reordering_and_unequal_scaling() {
    let source = block(DVec3::splat(20.)).build().unwrap();
    let bound = EdgeRef::bind(&source, 0).unwrap();
    let size = DVec3::new(36., 24., 18.);
    let mut mesh = block(size).build().unwrap();
    let finish = |src: Feature| Feature::EdgeFinish {
        source: Box::new(src),
        edge: 99999,
        reference: Some(bound.clone()),
        r1: 1.,
        r2: 3.,
        steps: 1,
    };
    let expected = finish(Feature::Polygon(mesh.clone())).build().unwrap();
    mesh.vertices.reverse();
    let n = mesh.vertices.len();
    for f in &mut mesh.faces {
        *f = f.map(|i| n - 1 - i);
    }
    mesh.faces.reverse();
    let actual = finish(Feature::Polygon(mesh)).build().unwrap();
    for p in expected.vertices {
        assert!(actual.vertices.iter().any(|q| p.distance(*q) < 1e-6));
    }
}
#[test]
fn unresolved_edge_leaves_previous_geometry() {
    let source = block(DVec3::splat(20.));
    let bound = EdgeRef::bind(&source.build().unwrap(), 0).unwrap();
    let mut o = Object::new(Shape::Box, DVec3::ZERO);
    o.feature = Some(Feature::EdgeFinish {
        source: Box::new(source),
        edge: 0,
        reference: Some(bound),
        r1: 1.,
        r2: 2.,
        steps: 1,
    });
    o.rebuild().unwrap();
    let before = o.mesh();
    if let Some(Feature::EdgeFinish { source, .. }) = &mut o.feature {
        **source = Feature::Primitive {
            shape: Shape::Cone,
            size: DVec3::splat(20.),
            bore: 4.,
        };
    }
    assert!(o.rebuild().is_err());
    assert_eq!(o.mesh(), before);
}
#[test]
fn planar_opening_follows_selected_side() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Schale.r3dp");
    for (side, _) in Side::ALL {
        let mut o = Object::new(Shape::Box, DVec3::ZERO);
        o.feature = Some(Feature::Shell {
            source: Box::new(block(DVec3::new(40., 30., 20.))),
            thickness: 2.,
            open: true,
            side: Some(side),
        });
        o.rebuild().unwrap();
        let p = Project {
            objects: vec![o],
            ..Default::default()
        };
        files::save(&file, &p, Output::Project, true).unwrap();
        let m = files::load(&file).unwrap().objects[0].mesh();
        let volume = m
            .faces
            .iter()
            .map(|f| {
                let [a, b, c] = f.map(|i| m.vertices[i]);
                a.dot(b.cross(c)) / 6.
            })
            .sum::<f64>()
            .abs();
        let cavity = match side {
            Side::Left | Side::Right => 38. * 26. * 16.,
            Side::Front | Side::Back => 36. * 28. * 16.,
            _ => 36. * 26. * 18.,
        };
        assert!((volume - (24000. - cavity)).abs() < 1e-4);
    }
}
