//! Analytischer STEP-AP214-Export für Vollzylinder und konzentrische Rohre.
use crate::{cad::Feature, geometry::Shape, project::Project};
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

pub(crate) fn write_cylinder(mut out: impl Write, project: &Project) -> io::Result<()> {
    let [object] = project.objects.as_slice() else {
        return Err(io::Error::other(
            "Für den STEP-Export genau einen Zylinder oder ein Rohr im Projekt verwenden.",
        ));
    };
    let (shape, size, bore) = match &object.feature {
        Some(Feature::Primitive { shape, size, bore }) => (*shape, *size, *bore),
        None => (object.shape, object.size, object.bore),
        _ => {
            return Err(io::Error::other(
                "Bearbeitete Körper werden vom STEP-Export noch nicht unterstützt.",
            ))
        }
    };
    if !matches!(shape, Shape::Cylinder | Shape::Tube) {
        return Err(io::Error::other(
            "Für STEP einen Kreiszylinder oder ein gerades Rohr verwenden.",
        ));
    }
    if size.x != size.y {
        return Err(io::Error::other(
            "Für einen Kreiszylinder müssen X- und Y-Durchmesser übereinstimmen.",
        ));
    }
    let r = object.rotation * (std::f64::consts::PI / 180.);
    let rotation = DQuat::from_euler(EulerRot::ZYX, r.z, r.y, r.x);
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
        timestamp = timestamp(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(io::Error::other)?
                .as_secs()
        )?,
        product = if hollow { "Rohr" } else { "Zylinder" },
        inner_face = if hollow { ",#101" } else { "" },
        top_hole = if hollow { ",#120" } else { "" },
        bottom_hole = if hollow { ",#123" } else { "" },
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
