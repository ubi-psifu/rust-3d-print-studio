use glam::DVec2;
use rust_3d_print_studio::section::{line_fill, Contour};
fn ring(lo: f64, hi: f64) -> Vec<[DVec2; 2]> {
    let p = [[lo, lo], [hi, lo], [hi, hi], [lo, hi]].map(DVec2::from_array);
    (0..4).map(|i| [p[i], p[(i + 1) % 4]]).collect()
}
#[test]
fn fill_alternates_and_preserves_holes() {
    let mut lines = ring(0., 10.);
    lines.extend(ring(3., 7.));
    let c = [Contour {
        slot: 2,
        color: [0, 0, 0],
        lines,
    }];
    for vertical in [true, false] {
        let fill = line_fill(&c, 1., vertical).unwrap();
        let paths = &fill[0].lines;
        assert_eq!(paths.len(), 14);
        assert_eq!(fill[0].slot, 2);
        assert!((paths.iter().map(|[a, b]| a.distance(*b)).sum::<f64>() - 84.).abs() < 1e-9);
        assert!(paths
            .iter()
            .all(|[a, b]| if vertical { a.x == b.x } else { a.y == b.y }));
    }
}
#[test]
fn open_spans_and_invalid_pitch_are_reported() {
    let c = [Contour {
        slot: 1,
        color: [0; 3],
        lines: vec![[DVec2::ZERO, DVec2::Y * 3.]],
    }];
    assert!(line_fill(&c, 1., false).is_err());
    assert!(line_fill(&c, 0., true).is_err());
}
#[test]
fn independent_stack_matches_cube_height_and_preserves_existing_file() {
    use rust_3d_print_studio::{files, layers};
    use std::{fs::File, io::Read, path::Path};
    let p = files::load(Path::new("examples/modelle/Wuerfel.r3dp")).unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("Plan.zip");
    assert_eq!(layers::save(&p, 0.2, 15, &out).unwrap(), 100);
    assert!(layers::save(&p, 0.2, 15, &out).is_err());
    let mut zip = zip::ZipArchive::new(File::open(out).unwrap()).unwrap();
    assert_eq!(zip.len(), 101);
    let mut svg = String::new();
    zip.by_name("Schichten/00100.svg")
        .unwrap()
        .read_to_string(&mut svg)
        .unwrap();
    assert!(svg.contains("Z 19.9000 mm"));
    assert!(svg.contains("id=\"Fuellung\""));
    assert!(svg.contains(" d=\"M"));
}
