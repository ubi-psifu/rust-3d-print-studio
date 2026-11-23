//! Analytischer STEP-AP214-Export eines einzelnen Kreiszylinders.
//! Die feste Topologie umfasst zwei Kreisränder, eine Mantelnaht und drei Flächen.
use crate::{cad::Feature, geometry::Shape, project::Project};
use glam::{DQuat, DVec3, EulerRot};
use std::io::{self, Write};

fn xyz(v: DVec3) -> String {
    format!("({:.16E},{:.16E},{:.16E})", v.x, v.y, v.z)
}

pub(crate) fn write_cylinder(mut out: impl Write, project: &Project) -> io::Result<()> {
    let [object] = project.objects.as_slice() else {
        return Err(io::Error::other(
            "Für den STEP-Export genau einen Zylinder im Projekt verwenden.",
        ));
    };
    let size = match &object.feature {
        Some(Feature::Primitive {
            shape: Shape::Cylinder,
            size,
            ..
        }) => *size,
        None if object.shape == Shape::Cylinder => object.size,
        _ => {
            return Err(io::Error::other(
                "STEP unterstützt derzeit einzelne unbearbeitete Kreiszylinder.",
            ))
        }
    };
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
    write!(
        out,
        include_str!("cylinder.step.in"),
        axis = xyz(axis),
        radial = xyz(radial),
        radius = format_args!("{radius:.16E}"),
        bottom = xyz(bottom),
        top = xyz(top),
        bottom_seam = xyz(bottom + radius * radial),
        top_seam = xyz(top + radius * radial)
    )
}
