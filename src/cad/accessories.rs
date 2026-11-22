use super::{boolean, loft, orient, require, Mesh, Operation, Result};
use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};
use ttf_parser::{Face, OutlineBuilder};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Case {
    pub inside: DVec3,
    pub wall: f64,
    pub play: f64,
    pub lip: f64,
    pub lid: bool,
}
impl Default for Case {
    fn default() -> Self {
        Self {
            inside: DVec3::new(52., 34., 18.),
            wall: 2.,
            play: 0.25,
            lip: 3.,
            lid: false,
        }
    }
}
fn rectangle(size: DVec2) -> Vec<DVec2> {
    let h = size / 2.;
    vec![-h, DVec2::new(h.x, -h.y), h, DVec2::new(-h.x, h.y)]
}
fn prism(r: &[DVec2], low: f64, high: f64) -> Result<Mesh> {
    let mut m = loft(
        &[low, high].map(|z| r.iter().map(|p| p.extend(z)).collect()),
        false,
    )?;
    orient(&mut m);
    Ok(m)
}
impl Case {
    pub fn build(&self) -> Result<Mesh> {
        require(
            self.inside.is_finite()
                && self.inside.min_element() >= 4.
                && self.inside.max_element() <= 500.
                && (0.4..=20.).contains(&self.wall)
                && (0.02..=3.).contains(&self.play)
                && (0.5..=self.inside.z).contains(&self.lip),
            "Innenmaß, Wandstärke, Deckelspiel oder Einstecktiefe ungültig",
        )?;
        let size = self.inside.truncate();
        let w = self.wall;
        let outer = rectangle(size + DVec2::splat(2. * w));
        if !self.lid {
            return boolean(
                &prism(&outer, 0., self.inside.z + w)?,
                &prism(&rectangle(size), w, self.inside.z + w + 1.)?,
                Operation::Difference,
            );
        }
        let outside = size - DVec2::splat(2. * self.play);
        let inside = outside - DVec2::splat(2. * w);
        require(
            inside.min_element() > 0.5,
            "Innenmaß für den Deckelkragen zu klein",
        )?;
        let plate = prism(&outer, 0., w)?;
        let rim = boolean(
            &prism(&rectangle(outside), w - 0.1, w + self.lip)?,
            &prism(&rectangle(inside), w - 0.2, w + self.lip + 0.1)?,
            Operation::Difference,
        )?;
        boolean(&plate, &rim, Operation::Union)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Relief {
    pub source: Art,
    pub size: f64,
    pub border: f64,
    pub base: f64,
    pub depth: f64,
    pub engraved: bool,
    pub hole: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Art {
    Text { value: String, mono: bool },
    Logo(Vec<Vec<Vec<DVec2>>>),
}
impl Default for Relief {
    fn default() -> Self {
        Self {
            source: Art::Text {
                value: "CLIP".into(),
                mono: false,
            },
            size: 10.,
            border: 2.5,
            base: 2.4,
            depth: 0.8,
            engraved: false,
            hole: true,
        }
    }
}
#[derive(Default)]
struct Outline {
    rings: Vec<Vec<DVec2>>,
    last: DVec2,
}
impl Outline {
    fn push(&mut self, p: DVec2) {
        if let Some(r) = self.rings.last_mut() {
            if r.last() != Some(&p) {
                r.push(p);
            }
        }
        self.last = p;
    }
}
fn xy(x: f32, y: f32) -> DVec2 {
    DVec2::new(x.into(), y.into())
}
impl OutlineBuilder for Outline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.rings.push(vec![]);
        self.push(xy(x, y));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.push(xy(x, y));
    }
    fn quad_to(&mut self, x: f32, y: f32, u: f32, v: f32) {
        let (a, b, c) = (self.last, xy(x, y), xy(u, v));
        for i in 1..=4 {
            let t = i as f64 / 4.;
            self.push(a.lerp(b, t).lerp(b.lerp(c, t), t));
        }
    }
    fn curve_to(&mut self, x: f32, y: f32, u: f32, v: f32, w: f32, z: f32) {
        let (a, b, c, d) = (self.last, xy(x, y), xy(u, v), xy(w, z));
        for i in 1..=6 {
            let t = i as f64 / 6.;
            self.push(
                a.lerp(b, t)
                    .lerp(b.lerp(c, t), t)
                    .lerp(b.lerp(c, t).lerp(c.lerp(d, t), t), t),
            );
        }
    }
    fn close(&mut self) {
        if let Some(r) = self.rings.last_mut() {
            if r.len() > 1 && r.first() == r.last() {
                r.pop();
            }
        }
    }
}
fn contains(r: &[DVec2], p: DVec2) -> bool {
    let mut inside = false;
    for (a, b) in r.iter().zip(r.iter().cycle().skip(1)) {
        if (a.y > p.y) != (b.y > p.y) && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x {
            inside = !inside
        }
    }
    inside
}
impl Relief {
    pub fn build(&self) -> Result<Mesh> {
        require(
            (1.0..=240.).contains(&self.size)
                && (0.5..=20.).contains(&self.border)
                && (0.4..=20.).contains(&self.base)
                && (0.1..=10.).contains(&self.depth)
                && (!self.engraved || self.depth < self.base),
            "Ungültige Reliefmaße",
        )?;
        let mut groups = match &self.source {
            Art::Logo(g) => g.clone(),
            Art::Text { value, mono } => {
                require(
                    !value.trim().is_empty() && value.chars().count() <= 32,
                    "1 bis 32 Zeichen eingeben",
                )?;
                let font = Face::parse(
                    if *mono {
                        epaint_default_fonts::HACK_REGULAR
                    } else {
                        epaint_default_fonts::UBUNTU_LIGHT
                    },
                    0,
                )
                .map_err(|_| "Schrift fehlt")?;
                let mut groups = vec![];
                let mut x = 0.;
                for c in value.chars() {
                    let id = font
                        .glyph_index(c)
                        .ok_or_else(|| format!("Zeichen fehlt: {c}"))?;
                    let mut o = Outline::default();
                    if font.outline_glyph(id, &mut o).is_some() {
                        for p in o.rings.iter_mut().flatten() {
                            p.x += x;
                        }
                        groups.push(o.rings)
                    }
                    x += f64::from(font.glyph_hor_advance(id).unwrap_or(0));
                }
                groups
            }
        };
        require(
            !groups.is_empty()
                && groups.len() <= 64
                && groups.iter().all(|g| {
                    !g.is_empty()
                        && g.len() <= 64
                        && g.iter().all(|r| {
                            (3..=256).contains(&r.len()) && r.iter().all(|p| p.is_finite())
                        })
                }),
            "Leere oder zu komplexe Kontur",
        )?;
        let pts = groups.iter().flatten().flatten().copied();
        let lo = pts.clone().reduce(DVec2::min).unwrap();
        let hi = pts.reduce(DVec2::max).unwrap();
        let span = hi - lo;
        require(span.min_element() > 1e-7, "Ungültige Konturgröße")?;
        let logo = matches!(self.source, Art::Logo(_));
        let k = self.size / if logo { span.x } else { span.y };
        let eye = if self.hole { 6. } else { 0. };
        for p in groups.iter_mut().flatten().flatten() {
            *p = (*p - (lo + hi) / 2.) * DVec2::new(k, if logo { -k } else { k })
                + DVec2::X * eye * 0.5;
        }
        let sz = span * k + DVec2::splat(self.border * 2.) + DVec2::X * eye;
        let mut body = prism(&rectangle(sz), 0., self.base)?;
        if self.hole {
            let ring: Vec<_> = super::ellipse(1.6, 1.6, 32)
                .iter()
                .map(|v| *v + DVec2::X * (3. - sz.x / 2.))
                .collect();
            body = boolean(
                &body,
                &prism(&ring, -0.1, self.base + 0.1)?,
                Operation::Difference,
            )?;
        }
        let (low, high, op) = if self.engraved {
            (
                self.base - self.depth,
                self.base + 0.1,
                Operation::Difference,
            )
        } else {
            (self.base - 0.1, self.base + self.depth, Operation::Union)
        };
        for g in groups {
            let depths: Vec<_> = g
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    g.iter()
                        .enumerate()
                        .filter(|(j, q)| *j != i && contains(q, r[0]))
                        .count()
                })
                .collect();
            for (i, r) in g.iter().enumerate() {
                if depths[i] % 2 != 0 {
                    continue;
                }
                let mut piece = prism(r, low, high)?;
                for (j, h) in g.iter().enumerate() {
                    if depths[j] == depths[i] + 1 && contains(r, h[0]) {
                        piece = boolean(
                            &piece,
                            &prism(h, low - 0.1, high + 0.1)?,
                            Operation::Difference,
                        )?;
                    }
                }
                body = boolean(&body, &piece, op)?;
            }
        }
        Ok(body)
    }
}
