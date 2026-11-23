//! Analytischer STEP-AP214-Export für Vollzylinder, konzentrische Rohre und Bohrquader.
use crate::{
    cad::Feature,
    geometry::{Object, Shape},
    project::Project,
};
use glam::{DQuat, DVec3, EulerRot};
use std::io::{self, Write};

// UTC-Kalenderdatum für den ISO-10303-21-Dateikopf.
fn timestamp(seconds: u64) -> io::Result<String> {
    let mut days = seconds / 86400;
    for year in 1970..=9999 {
        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let year_days = if leap { 366 } else { 365 };
        if days >= year_days {
            days -= year_days;
            continue;
        }
        let months = [
            31,
            if leap { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];
        for (month, count) in months.into_iter().enumerate() {
            if days < count {
                return Ok(format!(
                    "{year:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                    month + 1,
                    days + 1,
                    seconds / 3600 % 24,
                    seconds / 60 % 60,
                    seconds % 60
                ));
            }
            days -= count;
        }
    }
    Err(io::Error::other(
        "Systemdatum außerhalb des STEP-Zeitbereichs",
    ))
}

fn xyz(v: DVec3) -> String {
    format!("({:.16E},{:.16E},{:.16E})", v.x, v.y, v.z)
}

pub(crate) fn write_solid(
    mut out: impl Write,
    project: &Project,
    included: Option<&[bool]>,
) -> io::Result<()> {
    if project.objects.is_empty() {
        return Err(io::Error::other(
            "Das Projekt enthält keine Körper für den STEP-Export.",
        ));
    }
    let date = timestamp(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_secs(),
    )?;
    writeln!(out, "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('CAD in mm'),'2;1');\nFILE_NAME('Platte.step','{date}',(''),(''),'Rust 3D Print Studio','Rust 3D Print Studio','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;")?;
    for (index, object) in project.objects.iter().enumerate() {
        if included.is_some_and(|mask| !mask[index]) {
            continue;
        }
        write_object(&mut out, object, index + 1).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Objekt {} »{}«: {e}", index + 1, object.name),
            )
        })?;
    }
    writeln!(out, "ENDSEC;\nEND-ISO-10303-21;")
}

// Die Formatvorlagen verwenden je Teil drei Stellen für die lokale Entitätsnummer.
fn write_object(mut out: impl Write, object: &Object, id: usize) -> io::Result<()> {
    let product = step_name(&object.name);
    let (shape, size, bore) = match &object.feature {
        Some(Feature::Primitive { shape, size, bore }) => (*shape, *size, *bore),
        None => (object.shape, object.size, object.bore),
        _ => {
            return Err(io::Error::other(
                "Bearbeitete Körper werden vom STEP-Export noch nicht unterstützt.",
            ))
        }
    };
    if !matches!(shape, Shape::Cylinder | Shape::Tube | Shape::BoredBox) {
        return Err(io::Error::other(
            "Für STEP einen Kreiszylinder, ein gerades Rohr oder einen Bohrquader verwenden.",
        ));
    }
    if shape != Shape::BoredBox && size.x != size.y {
        return Err(io::Error::other(
            "Für einen Kreiszylinder müssen X- und Y-Durchmesser übereinstimmen.",
        ));
    }
    let r = object.rotation * (std::f64::consts::PI / 180.);
    let rotation = DQuat::from_euler(EulerRot::ZYX, r.z, r.y, r.x);
    if shape == Shape::BoredBox {
        return write_bored_box(out, object.position, rotation, size, bore, id, &product);
    }
    let axis = rotation * DVec3::Z;
    let radial = rotation * DVec3::X;
    let radius = size.x * 0.5;
    let bottom = object.position;
    let top = bottom + size.z * axis;
    let hollow = shape == Shape::Tube;
    let interior = if hollow {
        if !(bore.is_finite() && bore > 0. && bore < size.x) {
            return Err(io::Error::other(
                "Die Bohrung muss kleiner als der Außendurchmesser und positiv sein.",
            ));
        }
        let inner = bore * 0.5;
        format!(
            include_str!("tube-interior.step.in"),
            id = id,
            radius = format_args!("{inner:.16E}"),
            top_seam = xyz(top + inner * radial),
            bottom_seam = xyz(bottom + inner * radial)
        )
    } else {
        String::new()
    };
    write!(
        out,
        include_str!("cylinder.step.in"),
        id = id,
        product = product,
        inner_face = if hollow {
            format!(",#{id}101")
        } else {
            String::new()
        },
        top_hole = if hollow {
            format!(",#{id}120")
        } else {
            String::new()
        },
        bottom_hole = if hollow {
            format!(",#{id}123")
        } else {
            String::new()
        },
        interior = interior,
        axis = xyz(axis),
        radial = xyz(radial),
        radius = format_args!("{radius:.16E}"),
        bottom = xyz(bottom),
        top = xyz(top),
        bottom_seam = xyz(bottom + radius * radial),
        top_seam = xyz(top + radius * radial)
    )
}

fn write_bored_box(
    mut out: impl Write,
    origin: DVec3,
    rotation: DQuat,
    size: DVec3,
    bore: f64,
    id: usize,
    product: &str,
) -> io::Result<()> {
    if !(bore.is_finite() && bore > 0. && bore < size.x.min(size.y)) {
        return Err(io::Error::other(
            "Der Bohrungsdurchmesser muss positiv und kleiner als beide Außenmaße sein.",
        ));
    }
    let half_x = size.x / 2.;
    let half_y = size.y / 2.;
    let radius = bore / 2.;
    let point = |x, y, z| xyz(origin + rotation * DVec3::new(x, y, z));
    write!(
        out,
        include_str!("bored-box.step.in"),
        id = id,
        product = product,
        radius = format_args!("{radius:.16E}"),
        bottom_center = point(0., 0., 0.),
        bottom_left_back = point(-half_x, half_y, 0.),
        bottom_left_front = point(-half_x, -half_y, 0.),
        bottom_right_back = point(half_x, half_y, 0.),
        bottom_right_front = point(half_x, -half_y, 0.),
        bottom_seam = point(radius, 0., 0.),
        top_center = point(0., 0., size.z),
        top_left_back = point(-half_x, half_y, size.z),
        top_left_front = point(-half_x, -half_y, size.z),
        top_right_back = point(half_x, half_y, size.z),
        top_right_front = point(half_x, -half_y, size.z),
        top_seam = point(radius, 0., size.z),
        x_axis = xyz(rotation * DVec3::X),
        y_axis = xyz(rotation * DVec3::Y),
        z_axis = xyz(rotation * DVec3::Z),
    )
}

fn step_name(name: &str) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::new();
    for ch in name.chars() {
        match ch {
            '\'' => encoded.push_str("''"),
            ' '..='~' if ch != '\\' => encoded.push(ch),
            _ if u32::from(ch) <= 0xffff => {
                write!(encoded, "\\X2\\{:04X}\\X0\\", u32::from(ch)).unwrap()
            }
            _ => write!(encoded, "\\X4\\{:08X}\\X0\\", u32::from(ch)).unwrap(),
        }
    }
    encoded
}
