use egui_macroquad::egui::Rect;
use macroquad::{
    camera::{Camera, Projection},
    prelude::*,
};
use rust_3d_print_studio::{
    geometry,
    project::{Project, View},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SectionAxis {
    X,
    Y,
    Z,
}
impl SectionAxis {
    pub fn label(self) -> &'static str {
        match self {
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
        }
    }
    fn normal(self) -> ::glam::DVec3 {
        match self {
            Self::X => ::glam::DVec3::X,
            Self::Y => ::glam::DVec3::Y,
            Self::Z => ::glam::DVec3::Z,
        }
    }
}
pub struct Solid {
    model: geometry::Object,
    pub mesh: geometry::Mesh,
    gpu: Vec<Mesh>,
    pub bounds: (::glam::DVec3, ::glam::DVec3),
}
pub fn sync(cache: &mut Vec<Solid>, p: &Project) {
    cache.truncate(p.objects.len());
    for (i, o) in p.objects.iter().enumerate() {
        if cache.get(i).is_some_and(|s| s.model == *o) {
            continue;
        }
        let solid = Solid::new(o);
        if i == cache.len() {
            cache.push(solid)
        } else {
            cache[i] = solid
        }
    }
}
impl Solid {
    pub fn into_model(self) -> geometry::Object {
        self.model
    }
    pub fn new(o: &geometry::Object) -> Self {
        Self::from_mesh(o, o.mesh(), None)
    }

    pub fn section(&self, axis: SectionAxis, percent: u8) -> Result<(Self, f64), String> {
        let (lo, hi) = self.bounds;
        let normal = axis.normal();
        let offset = lo.lerp(hi, f64::from(percent) / 100.).dot(normal);
        let center = (lo + hi) * 0.5;
        let plane = center + normal * (offset - center.dot(normal));
        let mesh = rust_3d_print_studio::cad::clip(&self.mesh, plane, -normal)?;
        Ok((
            Self::from_mesh(&self.model, mesh, Some((normal, offset))),
            offset,
        ))
    }

    fn from_mesh(
        o: &geometry::Object,
        mesh: geometry::Mesh,
        cut: Option<(::glam::DVec3, f64)>,
    ) -> Self {
        let bounds = mesh.bounds();
        let mut vertices = Vec::with_capacity(mesh.faces.len() * 3);
        for face in &mesh.faces {
            let [a, b, c] = face.map(|j| mesh.vertices[j].as_vec3());
            let normal = (b - a).cross(c - a).normalize_or_zero();
            let shade = 0.66 + normal.dot(vec3(-0.35, -0.48, 0.8)).max(-0.4) * 0.32;
            let cap = cut.is_some_and(|(axis, offset)| {
                face.iter()
                    .all(|&i| (mesh.vertices[i].dot(axis) - offset).abs() < 1e-6)
            });
            let [r, g, b8] = if cap {
                [255, 186, 65]
            } else {
                o.color.map(|v| (v as f32 * shade) as u8)
            };
            for v in [a, b, c] {
                vertices.push(Vertex::new2(v, Vec2::ZERO, Color::from_rgba(r, g, b8, 255)));
            }
        }
        let gpu = vertices
            .chunks(3000)
            .map(|part| Mesh {
                indices: (0..part.len() as u16).collect(),
                vertices: part.to_vec(),
                texture: None,
            })
            .collect();
        Solid {
            model: o.clone(),
            mesh,
            gpu,
            bounds,
        }
    }

    pub fn edge(
        &self,
        index: usize,
    ) -> Result<rust_3d_print_studio::cad::references::EdgeRef, String> {
        rust_3d_print_studio::cad::references::EdgeRef::bind(&self.model.local_mesh(), index)
    }
    pub fn opening(
        &self,
        face: usize,
    ) -> Result<rust_3d_print_studio::cad::references::PlaneRef, String> {
        rust_3d_print_studio::cad::references::PlaneRef::bind(&self.model.local_mesh(), face)
    }
}
pub fn basis(v: &View) -> (Vec3, Vec3, Vec3) {
    let (s, c) = (v.yaw as f32).sin_cos();
    let (sp, cp) = (v.pitch as f32).sin_cos();
    let dir = vec3(cp * c, cp * s, sp);
    let right = vec3(-s, c, 0.);
    (dir, right, dir.cross(right))
}
pub fn camera(v: &View, rect: Rect, px: f32) -> Camera3D {
    let (dir, _, up) = basis(v);
    let target = v.target.as_vec3();
    Camera3D {
        position: target + dir * v.distance as f32,
        target,
        up,
        fovy: if v.perspective {
            0.78
        } else {
            v.distance as f32 * 0.80
        },
        aspect: Some(rect.width() / rect.height().max(1.)),
        projection: if v.perspective {
            Projection::Perspective
        } else {
            Projection::Orthographics
        },
        viewport: Some((
            (rect.left() * px) as i32,
            (screen_height() - rect.bottom() * px).round() as i32,
            (rect.width() * px).max(1.) as i32,
            (rect.height() * px).max(1.) as i32,
        )),
        z_near: 0.3,
        z_far: 20000.,
        ..Default::default()
    }
}
pub fn pick(cam: &Camera3D, x: f32, y: f32, cache: &[Solid]) -> Option<(usize, Vec3)> {
    pick_matrix(cam.matrix(), x, y, cache)
}
fn ray(matrix: Mat4, x: f32, y: f32) -> (Vec3, Vec3) {
    let inv = matrix.inverse();
    let origin = inv.project_point3(vec3(x, y, -1.));
    let dir = (inv.project_point3(vec3(x, y, 1.)) - origin).normalize();
    (origin, dir)
}
pub fn xy_at(cam: &Camera3D, x: f32, y: f32, z: f32) -> Option<::glam::DVec2> {
    let (o, d) = ray(cam.matrix(), x, y);
    if d.z.abs() < 0.015 {
        return None;
    }
    let t = (z - o.z) / d.z;
    (t >= 0.).then(|| (o + d * t).truncate().as_dvec2())
}
fn pick_matrix(matrix: Mat4, x: f32, y: f32, cache: &[Solid]) -> Option<(usize, Vec3)> {
    surface_matrix(matrix, x, y, cache).map(|(i, _, point)| (i, point))
}
pub fn surface(cam: &Camera3D, x: f32, y: f32, cache: &[Solid]) -> Option<(usize, usize, Vec3)> {
    surface_matrix(cam.matrix(), x, y, cache)
}
fn surface_matrix(matrix: Mat4, x: f32, y: f32, cache: &[Solid]) -> Option<(usize, usize, Vec3)> {
    let (origin, dir) = ray(matrix, x, y);
    let mut best = f32::INFINITY;
    let mut index = None;
    for (i, s) in cache.iter().enumerate() {
        for (face, f) in s.mesh.faces.iter().enumerate() {
            let [a, b, c] = f.map(|j| s.mesh.vertices[j].as_vec3());
            let e = b - a;
            let q = c - a;
            let cross = dir.cross(q);
            let det = e.dot(cross);
            if det.abs() < 1e-7 {
                continue;
            }
            let t = origin - a;
            let u = t.dot(cross) / det;
            let k = t.cross(e);
            let v = dir.dot(k) / det;
            let depth = q.dot(k) / det;
            if u >= 0. && v >= 0. && u + v <= 1. && depth > 0. && depth < best {
                best = depth;
                index = Some((i, face));
            }
        }
    }
    index.map(|(i, f)| (i, f, origin + dir * best))
}
fn panel(a: f32, b: f32, c: f32, d: f32, z: f32, color: Color) {
    let vertices = [vec3(a, b, z), vec3(c, b, z), vec3(c, d, z), vec3(a, d, z)]
        .map(|v| Vertex::new2(v, Vec2::ZERO, color))
        .to_vec();
    draw_mesh(&Mesh {
        vertices,
        indices: vec![0, 1, 2, 0, 2, 3],
        texture: None,
    });
}
pub fn draw(
    cam: &Camera3D,
    p: &Project,
    cache: &[Solid],
    selected: &[usize],
    wire: bool,
    dark: bool,
) {
    set_camera(cam);
    let [w, h] = p.printer.bed.map(|x| x as f32);
    panel(
        0.,
        0.,
        w,
        h,
        -0.3,
        if dark {
            Color::from_hex(0x354358)
        } else {
            Color::from_hex(0xc7d9db)
        },
    );
    for &[a, b, c, d] in &p.printer.excluded {
        panel(
            a as f32,
            b as f32,
            c as f32,
            d as f32,
            -0.16,
            Color::from_hex(0xae7659),
        );
    }
    let grid = if dark {
        Color::from_hex(0x596676)
    } else {
        Color::from_hex(0x9db6bb)
    };
    let step = p.view.grid as f32;
    for i in 0..=(w / step) as usize {
        let x = i as f32 * step;
        draw_line_3d(vec3(x, 0., -0.05), vec3(x, h, -0.05), grid);
    }
    for i in 0..=(h / step) as usize {
        let y = i as f32 * step;
        draw_line_3d(vec3(0., y, -0.05), vec3(w, y, -0.05), grid);
    }
    for (end, col) in [
        (vec3(w, 0., 0.), RED),
        (vec3(0., h, 0.), GREEN),
        (vec3(0., 0., 45.), SKYBLUE),
    ] {
        draw_line_3d(Vec3::ZERO, end, col);
    }
    draw_line_3d(vec3(w, 0., 0.), vec3(w, h, 0.), grid);
    draw_line_3d(vec3(0., h, 0.), vec3(w, h, 0.), grid);
    for (i, s) in cache.iter().enumerate() {
        if wire {
            for f in &s.mesh.faces {
                let [a, b, c] = f.map(|k| s.mesh.vertices[k].as_vec3());
                for (u, v) in [(a, b), (b, c), (c, a)] {
                    draw_line_3d(u, v, if selected.contains(&i) { GOLD } else { grid })
                }
            }
        } else {
            for batch in &s.gpu {
                draw_mesh(batch);
            }
        }
        if selected.contains(&i) {
            let (lo, hi) = s.bounds;
            let span = (hi - lo).as_vec3() + Vec3::splat(0.4);
            draw_cube_wires(((hi + lo) * 0.5).as_vec3(), span, Color::from_hex(0x62978c));
        }
    }
    set_default_camera();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_3d_print_studio::geometry::{Object, Shape};
    #[test]
    fn shell_section_retains_cavity_at_world_height() {
        let project = rust_3d_print_studio::files::load(std::path::Path::new(
            "examples/modelle/Seitlich-offene-Schale.r3dp",
        ))
        .unwrap();
        let mut object = project.objects[0].clone();
        object.position.z = 7.;
        object.rotation.z = 32.;
        let solid = Solid::new(&object);
        for percent in [25, 50, 75] {
            let (section, z) = solid.section(SectionAxis::Z, percent).unwrap();
            assert!((z - (7. + 0.2 * f64::from(percent))).abs() < 1e-7);
            assert!((section.bounds.1.z - z).abs() < 1e-6);
            let area: f64 = section
                .mesh
                .faces
                .iter()
                .map(|f| f.map(|i| section.mesh.vertices[i]))
                .filter(|tri| tri.iter().all(|v| (v.z - z).abs() < 1e-6))
                .map(|[a, b, c]| (b - a).cross(c - a).length() * 0.5)
                .sum();
            assert!((area - (40. * 30. - 38. * 26.)).abs() < 1e-3, "{area}");
        }
        assert_eq!(solid.into_model(), object);
    }
    #[test]
    fn vertical_sections_follow_plate_axes_after_rotation() {
        let project = rust_3d_print_studio::files::load(std::path::Path::new(
            "examples/modelle/Seitlich-offene-Schale.r3dp",
        ))
        .unwrap();
        let mut object = project.objects[0].clone();
        object.rotation.z = 90.;
        object.position = ::glam::DVec3::new(111., 89., 7.);
        let solid = Solid::new(&object);
        for (axis, area) in [
            (SectionAxis::X, 40. * 20. - 38. * 16.),
            (SectionAxis::Y, 30. * 20. - 26. * 16.),
        ] {
            let n = axis.normal();
            for percent in [25, 50, 75] {
                let (cut, offset) = solid.section(axis, percent).unwrap();
                let (lo, hi) = solid.bounds;
                let expected = lo.dot(n) + (hi - lo).dot(n) * f64::from(percent) / 100.;
                assert!((offset - expected).abs() < 1e-6);
                assert!((cut.bounds.0.dot(n) - lo.dot(n)).abs() < 1e-6);
                assert!((cut.bounds.1.dot(n) - offset).abs() < 1e-6);
                let actual: f64 = cut
                    .mesh
                    .faces
                    .iter()
                    .map(|f| f.map(|i| cut.mesh.vertices[i]))
                    .filter(|tri| tri.iter().all(|p| (p.dot(n) - offset).abs() < 1e-6))
                    .map(|[a, b, c]| (b - a).cross(c - a).length() * 0.5)
                    .sum();
                assert!((actual - area).abs() < 1e-3, "{actual} / {area}");
            }
        }
    }
    #[test]
    fn click_on_rotated_wedge_binds_local_slope_in_both_projections() {
        use rust_3d_print_studio::{
            cad::{references::Side, Feature},
            files::{self, Output},
        };
        let mut project =
            files::load(std::path::Path::new("examples/modelle/Keilschale.r3dp")).unwrap();
        let object = &mut project.objects[0];
        object.rotation.z = 90.;
        object.position = ::glam::DVec3::new(80., 100., 8.);
        let Some(Feature::Shell { source, .. }) = &object.feature else {
            panic!()
        };
        let mut outer = object.clone();
        outer.feature = Some((**source).clone());
        outer.rebuild().unwrap();
        let cache = [Solid::new(&outer)];
        let center = vec3(65., 120., 18.);
        let view = Mat4::look_at_rh(center + vec3(0., 60., 120.), center, Vec3::X);
        for projection in [
            Mat4::perspective_rh_gl(0.8, 1., 0.3, 500.),
            Mat4::orthographic_rh_gl(-35., 35., -35., 35., 0.3, 500.),
        ] {
            let (_, face, hit) = surface_matrix(projection * view, 0., 0., &cache).unwrap();
            assert!(hit.distance(center) < 0.01);
            let reference = cache[0].opening(face).unwrap();
            if let Some(Feature::Shell { side, .. }) = &mut object.feature {
                *side = Some(Side::Plane(reference));
            }
            object.rebuild().unwrap();
            let mesh = object.mesh();
            let volume = mesh
                .faces
                .iter()
                .map(|f| {
                    let [a, b, c] = f.map(|i| mesh.vertices[i]);
                    a.dot(b.cross(c)) / 6.
                })
                .sum::<f64>();
            assert!((volume.abs() - 4486.).abs() < 1e-3);
        }
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Gewählte Schräge.r3dp");
        files::save(&path, &project, Output::Project, false).unwrap();
        assert_eq!(
            files::load(&path).unwrap().objects[0].feature,
            project.objects[0].feature
        );
    }
    #[test]
    fn ray_selects_frontmost_object_in_both_projections() {
        let mut p = Project::default();
        for y in [80., 140.] {
            p.objects
                .push(Object::new(Shape::Box, ::glam::DVec3::new(128., y, 0.)));
        }
        let mut cache = vec![];
        sync(&mut cache, &p);
        let view = Mat4::look_at_rh(vec3(128., 0., 10.), vec3(128., 128., 10.), Vec3::Z);
        for projection in [
            Mat4::perspective_rh_gl(0.8, 1., 0.3, 20000.),
            Mat4::orthographic_rh_gl(-75., 75., -75., 75., 0.3, 20000.),
        ] {
            assert_eq!(
                pick_matrix(projection * view, 0., 0., &cache).map(|h| h.0),
                Some(0)
            );
            assert_eq!(pick_matrix(projection * view, 0.95, 0.95, &cache), None);
        }
    }
}
