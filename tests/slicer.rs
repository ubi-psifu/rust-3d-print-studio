use glam::DVec3;
use rust_3d_print_studio::{
    files,
    geometry::{Object, Shape},
    project::Project,
    slicer::{export, Preparation},
};
use serde_json::json;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};
fn template(path: &Path, sliced: bool, area: &str) {
    let mut zip = ZipWriter::new(File::create(path).unwrap());
    let opt = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.start_file("3D/3dmodel.model", opt).unwrap();
    zip.write_all(
        br#"<model><metadata name="Application">BambuStudio-02.09.01.70</metadata></model>"#,
    )
    .unwrap();
    zip.start_file("Metadata/project_settings.config", opt)
        .unwrap();
    serde_json::to_writer(&mut zip,&json!({"printer_model":"Bambu Lab P1S","printer_settings_id":"P1S","print_settings_id":"Test","nozzle_diameter":["0.4"],"printable_area":["0x0","256x0",area,"0x256"],"filament_settings_id":["PLA Blau","PLA Rot"],"filament_colour":["#2040FF","#FF0000"],"inherits_group":["","","",""],"different_settings_to_system":["","","",""],"unbekannt":{"wert":42}})).unwrap();
    if sliced {
        zip.start_file("Metadata/plate_1.gcode", opt).unwrap();
    }
    zip.finish().unwrap();
}
fn plate() -> Project {
    let mut p = Project::default();
    for (x, y, slot, name) in [(40., 50., 1, "A & B"), (202., 191., 2, "Teil 2")] {
        let mut o = Object::new(Shape::Box, DVec3::new(x, y, 0.));
        o.filament_slot = slot;
        o.name = name.into();
        p.objects.push(o);
    }
    p
}
#[test]
fn preparation_preserves_global_profile_and_slot_mapping() {
    let temp = tempfile::tempdir().unwrap();
    let src = temp.path().join("Vorlage.3mf");
    let dst = temp.path().join("Platte.3mf");
    template(&src, false, "256x256");
    let mut prep = Preparation::load(&src).unwrap();
    assert_eq!(prep.slot_color(1), Some([255, 0, 0]));
    assert!(prep.override_process(0.8, 3, 20, true).is_err());
    prep.override_process(0.16, 4, 27, true).unwrap();
    export(&dst, &plate(), Some(&prep)).unwrap();
    assert!(export(&dst, &plate(), Some(&prep)).is_err());
    let mut zip = ZipArchive::new(File::open(&dst).unwrap()).unwrap();
    let data: serde_json::Value =
        serde_json::from_reader(zip.by_name("Metadata/project_settings.config").unwrap()).unwrap();
    assert_eq!(data["wall_loops"], "4");
    assert_eq!(data["unbekannt"]["wert"], 42);
    let mut xml = String::new();
    zip.by_name("Metadata/model_settings.config")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    assert!(xml.contains("A &amp; B"));
    assert!(xml.contains(r#"key="extruder" value="2""#));
    assert_eq!(xml.matches("<model_instance>").count(), 2);
}
#[test]
fn rejects_sliced_templates_incompatible_beds_and_missing_filaments() {
    let temp = tempfile::tempdir().unwrap();
    let src = temp.path().join("Vorlage.3mf");
    let dst = temp.path().join("Test.3mf");
    template(&src, true, "256x256");
    assert!(Preparation::load(&src).is_err());
    template(&src, false, "200x200");
    assert!(export(&dst, &plate(), Some(&Preparation::load(&src).unwrap())).is_err());
    template(&src, false, "256x256");
    let mut p = plate();
    p.objects[0].filament_slot = 3;
    assert!(export(&dst, &p, Some(&Preparation::load(&src).unwrap())).is_err());
    assert!(!dst.exists());
}
#[test]
fn project_round_trip_keeps_filament_slots() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("Projekt.r3dp");
    let p = plate();
    files::save(&path, &p, files::Output::Project, false).unwrap();
    assert_eq!(files::load(&path).unwrap().objects, p.objects);
}

#[test]
fn section_uses_world_coordinates_and_clears_above_the_plate() {
    let p = plate();
    let sections = rust_3d_print_studio::section::intersect(&p, 8.).unwrap();
    assert_eq!(sections.len(), 2);
    let length: f64 = sections[0].lines.iter().map(|[a, b]| a.distance(*b)).sum();
    assert!((length - 80.).abs() < 1e-8);
    assert!(sections[0]
        .lines
        .iter()
        .flatten()
        .all(|q| q.x >= 30. && q.x <= 50. && q.y >= 40. && q.y <= 60.));
    assert!(rust_3d_print_studio::section::intersect(&p, 21.)
        .unwrap()
        .is_empty());
}
