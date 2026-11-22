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

#[test]
fn sloped_opening_roundtrip_and_volume() {
    let mut project =
        files::load(std::path::Path::new("examples/modelle/Keilschale.r3dp")).unwrap();
    let obj = &mut project.objects[0];
    obj.rebuild().unwrap();
    let mesh = obj.mesh();
    rust_3d_print_studio::cad::validate(&mesh).unwrap();
    let volume = mesh
        .faces
        .iter()
        .map(|f| {
            let [a, b, c] = f.map(|i| mesh.vertices[i]);
            a.dot(b.cross(c)) / 6.
        })
        .sum::<f64>()
        .abs();
    assert!(
        (volume - (12000. - 34. * 17. / 2. * 26.)).abs() < 1e-4,
        "{volume}"
    );
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Keilschale.r3dp");
    files::save(&file, &project, Output::Project, false).unwrap();
    assert_eq!(
        files::load(&file).unwrap().objects[0].feature,
        project.objects[0].feature
    );
}

#[test]
fn bound_plane_survives_reordering_and_resize_and_rejects_missing_face() {
    use rust_3d_print_studio::cad::references::PlaneRef;
    let project = files::load(std::path::Path::new("examples/modelle/Keilschale.r3dp")).unwrap();
    let Some(Feature::Shell { source, .. }) = &project.objects[0].feature else {
        panic!()
    };
    let mut mesh = source.build().unwrap();
    let slope = mesh
        .faces
        .iter()
        .position(|f| {
            let [a, b, c] = f.map(|i| mesh.vertices[i]);
            let n = (b - a).cross(c - a).normalize();
            n.x > 0.1 && n.z > 0.1
        })
        .unwrap();
    let reference = PlaneRef::bind(&mesh, slope).unwrap();
    assert!(PlaneRef::bind(&mesh, mesh.faces.len()).is_err());
    mesh.faces.reverse();
    for p in &mut mesh.vertices {
        *p *= DVec3::new(2., 1., 0.8);
    }
    let shell = |src| Feature::Shell {
        source: Box::new(src),
        open: true,
        thickness: 2.,
        side: Some(Side::Plane(reference)),
    };
    let result = shell(Feature::Polygon(mesh)).build().unwrap();
    let (lo, hi) = result.bounds();
    assert!((hi - lo).distance(DVec3::new(80., 30., 16.)) < 1e-5);
    assert!(shell(block(DVec3::splat(20.))).build().is_err());
}
