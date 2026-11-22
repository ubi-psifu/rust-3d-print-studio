use glam::DVec3;
use rust_3d_print_studio::{
    geometry::{Object, Shape},
    plate::{arrange, copy_offsets},
    project::Printer,
};

fn box_at(x: f64, y: f64, w: f64, h: f64) -> Object {
    let mut o = Object::new(Shape::Box, DVec3::new(x, y, 0.));
    o.size = DVec3::new(w, h, 12.);
    o
}
fn separated(a: (DVec3, DVec3), b: (DVec3, DVec3), gap: f64) -> bool {
    let g = gap - 1e-7;
    a.0.x >= b.1.x + g || a.0.y >= b.1.y + g || b.0.x >= a.1.x + g || b.0.y >= a.1.y + g
}
#[test]
fn fill_respects_edges_holes_spacing_and_capacity() {
    let p = Printer {
        bed: [100., 80.],
        excluded: vec![[0., 0., 20., 20.]],
        ..Default::default()
    };
    let group = [box_at(50., 40., 18., 18.)];
    let original = group[0].mesh().bounds();
    let shifts = copy_offsets(&p, &group, &group, 2., 128);
    assert!(shifts.len() >= 12);
    assert_eq!(copy_offsets(&p, &group, &group, 2., 2).len(), 2);
    let mut placed = vec![original];
    for d in shifts {
        let b = (original.0 + d.extend(0.), original.1 + d.extend(0.));
        assert!(p.warning(b.0, b.1).is_none());
        assert!(placed.iter().all(|a| separated(*a, b, 2.)));
        placed.push(b);
    }
}
#[test]
fn rotated_group_copies_preserve_relative_xy_and_z() {
    let p = Printer::default();
    let mut a = box_at(50., 80., 50., 14.);
    a.rotation.z = 38.;
    a.position.z = 8.;
    let group = [a, box_at(105., 80., 15., 25.)];
    let d = copy_offsets(&p, &group, &group, 4., 1)[0].extend(0.);
    let copies: Vec<_> = group
        .iter()
        .cloned()
        .map(|mut o| {
            o.position += d;
            o
        })
        .collect();
    assert_eq!(copies[0].position.z, 8.);
    assert!(
        ((copies[1].position - copies[0].position) - (group[1].position - group[0].position))
            .length()
            < 1e-8
    );
    for o in copies {
        let b = o.mesh().bounds();
        assert!(p.warning(b.0, b.1).is_none());
        assert!(group.iter().all(|o| separated(o.mesh().bounds(), b, 4.)));
    }
}
#[test]
fn arrange_drops_and_is_all_or_nothing() {
    let p = Printer {
        bed: [82., 40.],
        excluded: vec![],
        ..Default::default()
    };
    let mut objs = [box_at(20., 20., 40., 40.), box_at(20., 20., 40., 40.)];
    objs[0].position.z = 7.;
    let shifts = arrange(&p, &objs, 2.).unwrap();
    for (o, d) in objs.iter_mut().zip(shifts) {
        o.position += d;
        assert!(o.mesh().bounds().0.z.abs() < 1e-7);
    }
    assert!(separated(
        objs[0].mesh().bounds(),
        objs[1].mesh().bounds(),
        2.
    ));
    assert!(arrange(&p, &objs, 4.).is_none());
    assert!(copy_offsets(&p, &objs, &objs[..1], 2., 1).is_empty());
}
