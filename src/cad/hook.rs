use super::{ellipse, require, Curve, Feature, Operation, Result};
use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};
use std::f64::consts::FRAC_PI_2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum HookStyle {
    #[default]
    Wall,
    J,
    S,
    Ceiling,
    Coat,
    Double,
    Cable,
    Clip,
}
impl HookStyle {
    pub const ALL: [Self; 8] = [
        Self::Wall,
        Self::J,
        Self::S,
        Self::Ceiling,
        Self::Coat,
        Self::Double,
        Self::Cable,
        Self::Clip,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Wall => "J-Wandhaken",
            Self::J => "J-Haken",
            Self::S => "S-Haken",
            Self::Ceiling => "Deckenhaken",
            Self::Coat => "Kleiderhaken",
            Self::Double => "Doppelhaken",
            Self::Cable => "Kabelhaken",
            Self::Clip => "Einhängehaken",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hook {
    #[serde(default)]
    pub style: HookStyle,
    #[serde(default)]
    pub flat_tip: bool,
    #[serde(default)]
    pub rectangular: bool,
    pub height: f64,
    pub width: f64,
    pub thickness: f64,
    pub radius: f64,
    pub shank: f64,
    pub diameter: f64,
    pub tip_length: f64,
    pub tip_scale: f64,
    pub tip_angle: f64,
    pub oval: f64,
    pub corner: f64,
    pub fillet: f64,
    pub holes: usize,
    pub spacing: f64,
    pub drill: f64,
    pub sink: f64,
    pub clearance: f64,
    pub rib: f64,
    pub steps: usize,
}
impl Default for Hook {
    fn default() -> Self {
        Self {
            style: HookStyle::Wall,
            flat_tip: false,
            rectangular: false,
            height: 78.,
            width: 32.,
            thickness: 6.,
            radius: 13.,
            shank: 20.,
            diameter: 11.,
            tip_length: 10.,
            tip_scale: 0.64,
            tip_angle: 15.,
            oval: 1.,
            corner: 5.,
            fillet: 1.2,
            holes: 2,
            spacing: 56.,
            drill: 4.5,
            sink: 9.,
            clearance: 0.2,
            rib: 4.2,
            steps: 24,
        }
    }
}
impl Hook {
    pub fn preset(style: HookStyle) -> Self {
        let mut h = Self {
            style,
            ..Self::default()
        };
        match style {
            HookStyle::Wall => {}
            HookStyle::J | HookStyle::S | HookStyle::Clip => {
                h.holes = 0;
                h.rib = 0.;
                h.diameter = 7.;
                h.shank = 30.;
                h.flat_tip = style == HookStyle::Clip;
                h.rectangular = h.flat_tip;
            }
            HookStyle::Double => {
                h.width = 48.;
                h.rib = 0.;
            }
            HookStyle::Ceiling => {
                h.tip_angle = 58.;
                h.tip_length = 18.;
                h.shank = 28.;
            }
            HookStyle::Coat => {
                h.radius = 19.;
                h.tip_length = 19.;
                h.oval = 1.45;
            }
            HookStyle::Cable => {
                h.radius = 10.;
                h.shank = 10.;
                h.tip_angle = 50.;
                h.tip_length = 14.;
                h.diameter = 7.;
                h.rib = 2.5;
            }
        }
        h
    }
    fn section(&self) -> Vec<DVec2> {
        let x = self.diameter * self.oval / 2.;
        let y = self.diameter / 2.;
        if self.rectangular {
            vec![
                DVec2::new(-x, -y),
                DVec2::new(x, -y),
                DVec2::new(x, y),
                DVec2::new(-x, y),
            ]
        } else {
            ellipse(x, y, self.steps)
        }
    }
    fn free_arm(&self) -> Feature {
        let r = self.radius;
        let l = self.shank;
        let z = self.diameter * self.oval / 2.;
        let point = |x, y| DVec3::new(x, y, z);
        let mut pts = vec![];
        if self.style == HookStyle::S {
            pts.push(point(-2. * r, l - self.tip_length));
            for i in 0..=self.steps / 2 {
                let a = std::f64::consts::PI * (1. - i as f64 / (self.steps / 2) as f64);
                pts.push(point(-r + r * a.cos(), l + r * a.sin()));
            }
        } else {
            pts.push(point(0., l));
        }
        if self.style == HookStyle::Clip {
            pts.extend([
                point(0., 0.),
                point(2. * r, 0.),
                point(2. * r, self.tip_length),
            ]);
        } else {
            for i in 0..=self.steps / 2 {
                let a = std::f64::consts::PI * (1. + i as f64 / (self.steps / 2) as f64);
                pts.push(point(r + r * a.cos(), r * a.sin()));
            }
            let a = self.tip_angle.to_radians();
            pts.push(point(
                2. * r - self.tip_length * a.sin(),
                self.tip_length * a.cos(),
            ));
        }
        Feature::Sweep {
            path: Curve::Polyline(pts),
            profile: self.section(),
            steps: self.steps,
            up: DVec3::Z,
            end_scale: self.tip_scale,
            twist: 0.,
            round_tip: !self.flat_tip,
            closed: false,
        }
    }
    pub fn features(&self) -> Result<Feature> {
        let h = self;
        let [w, l, t, r, d] = [h.width, h.height, h.thickness, h.radius, h.diameter];
        require(
            (12.0..=180.).contains(&w)
                && (30.0..=200.).contains(&l)
                && (2.0..=25.).contains(&t)
                && (2.0..=25.).contains(&d)
                && (0.35..=1.8).contains(&h.oval)
                && d * h.oval < w - 2. * h.fillet,
            "Platte oder Materialquerschnitt unzulässig",
        )?;
        require(
            (8..=64).contains(&h.steps)
                && r > d / 2. + 0.5
                && r < 80.
                && (4.0..=90.).contains(&h.shank)
                && (1.0..=60.).contains(&h.tip_length)
                && (0.25..=1.2).contains(&h.tip_scale)
                && (-30.0..=60.).contains(&h.tip_angle),
            "Biegeradius, Schenkel oder Spitze prüfen",
        )?;
        require(
            h.fillet > 0.05 && h.fillet < t / 2. && h.corner > h.fillet && h.corner < w.min(l) / 2.,
            "Außenradien kollidieren mit Plattenkanten",
        )?;
        let drill = h.drill + h.clearance;
        let sink = h.sink + h.clearance;
        require(
            h.holes <= 4
                && (0.0..=1.).contains(&h.clearance)
                && drill >= 1.
                && sink >= drill
                && (sink - drill) / 2. < t - 0.25
                && sink < w - 2. * h.fillet,
            "Senkung passt nicht in die Platte",
        )?;
        require(
            h.holes < 2
                || (h.spacing > sink + 1.
                    && h.spacing * (h.holes - 1) as f64 + sink + 2. * h.fillet < l),
            "Lochabstand oder Anzahl passt nicht zur Plattenhöhe",
        )?;
        require(
            (0.0..=d * 0.85).contains(&h.rib),
            "Rippenbreite muss kleiner als der Armquerschnitt sein",
        )?;
        if matches!(h.style, HookStyle::J | HookStyle::S | HookStyle::Clip) {
            return Ok(self.free_arm());
        }
        let n = h.steps;
        let mut rings = vec![];
        for top in [false, true] {
            for k in 0..=4 {
                let a = if top {
                    k as f64 / 4. * FRAC_PI_2
                } else {
                    (4 - k) as f64 / 4. * FRAC_PI_2
                };
                let inset = h.fillet * (1. - a.cos());
                let z = if top {
                    t - h.fillet + h.fillet * a.sin()
                } else {
                    h.fillet - h.fillet * a.sin()
                };
                let mut ring = vec![];
                for q in 0..4 {
                    let sx = if q == 0 || q == 3 { 1. } else { -1. };
                    let sy = if q < 2 { 1. } else { -1. };
                    let c = DVec2::new(sx * (w / 2. - h.corner), sy * (l / 2. - h.corner));
                    for i in 0..=n / 4 {
                        let ang = (q as f64 + i as f64 / (n / 4) as f64) * FRAC_PI_2;
                        let p = c + DVec2::new(ang.cos(), ang.sin()) * (h.corner - inset);
                        ring.push(p.extend(z));
                    }
                }
                rings.push(ring);
            }
        }
        let mut plate = Feature::Loft { rings };
        let y = -l * 0.15;
        for i in 0..h.holes {
            let cy = (i as f64 - (h.holes as f64 - 1.) / 2.) * h.spacing;
            require(
                (cy - y).abs() > sink / 2. + d / 2.
                    && (h.rib == 0. || cy > y || cy + sink / 2. < y - d),
                "Bohrung schneidet Hakenfuß oder Rippe",
            )?;
            let mut levels = vec![(-1., drill / 2.), (t - (sink - drill) / 2., drill / 2.)];
            if sink > drill {
                levels.push((t, sink / 2.));
            }
            levels.push((t + 1., sink / 2.));
            let rings = levels
                .into_iter()
                .map(|(z, rad)| {
                    ellipse(rad, rad, n)
                        .into_iter()
                        .map(|p| DVec3::new(p.x, p.y + cy, z))
                        .collect()
                })
                .collect();
            plate = combine(plate, Feature::Loft { rings }, Operation::Difference);
        }
        let pt = |dy, z| DVec3::new(0., y + dy, z);
        let start = t * 0.45;
        let elbow = t + h.shank;
        let s = h.tip_length;
        let a = h.tip_angle.to_radians();
        let end = pt(r + s * a.cos(), elbow + r - s * a.sin());
        let tip_dir = DVec3::new(0., a.cos(), -a.sin());
        let path = Curve::Bezier(vec![
            pt(0., start),
            pt(0., start + (elbow - start) / 3.),
            pt(0., elbow - (elbow - start) / 3.),
            pt(0., elbow),
            pt(0., elbow + r * 0.55228475),
            pt(r - r * 0.55228475, elbow + r),
            pt(r, elbow + r),
            pt(r + s / 3., elbow + r),
            end - tip_dir * s / 3.,
            end,
        ]);
        let sweep = Feature::Sweep {
            path,
            profile: h.section(),
            steps: n / 2,
            up: DVec3::X,
            end_scale: h.tip_scale,
            twist: 0.,
            round_tip: !h.flat_tip,
            closed: false,
        };
        let mut solid = if h.style == HookStyle::Double {
            let offset = w * 0.24;
            require(
                offset + d * h.oval / 2. < w / 2. - h.fillet,
                "Doppelarme außerhalb der Platte",
            )?;
            let arm = |x| Feature::Transform {
                source: Box::new(sweep.clone()),
                translation: DVec3::X * x,
                rotation: DVec3::ZERO,
                scale: DVec3::ONE,
            };
            combine(
                combine(plate, arm(-offset), Operation::Union),
                arm(offset),
                Operation::Union,
            )
        } else {
            combine(plate, sweep, Operation::Union)
        };
        if h.rib > 0. {
            let x = -h.rib / 2.;
            let pts = vec![
                DVec3::new(x, y - d, t * 0.6),
                DVec3::new(x, y, t * 0.6),
                DVec3::new(x, y, elbow * 0.93),
                DVec3::new(x, y - d * 0.6, t + d * 0.45),
            ];
            solid = combine(
                solid,
                Feature::Extrude {
                    profile: pts,
                    direction: DVec3::X * h.rib,
                    end_scale: 1.,
                },
                Operation::Union,
            );
        }
        Ok(solid)
    }
}
pub fn combine(left: Feature, right: Feature, operation: Operation) -> Feature {
    Feature::Boolean {
        operation,
        left: Box::new(left),
        right: Box::new(right),
    }
}
