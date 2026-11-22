//! SVG-Teilmenge für geschlossene Reliefkonturen.
use crate::cad::accessories::{Art, Relief};
use glam::DVec2;
use quick_xml::{events::Event, Reader};
use std::collections::BTreeMap;
type R<T> = Result<T, String>;
fn tokens(s: &str) -> R<Vec<&str>> {
    let b = s.as_bytes();
    let mut out = vec![];
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_whitespace() || b[i] == b',' {
            i += 1;
            continue;
        }
        let start = i;
        if b[i].is_ascii_alphabetic() {
            i += 1
        } else {
            if matches!(b[i], b'+' | b'-') {
                i += 1
            }
            let mut digits = 0;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
                digits += 1
            }
            if b.get(i) == Some(&b'.') {
                i += 1;
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                    digits += 1
                }
            }
            if digits == 0 {
                return Err("Ungültiges Pfadzeichen".into());
            }
            if matches!(b.get(i), Some(b'e' | b'E')) {
                i += 1;
                if matches!(b.get(i), Some(b'+' | b'-')) {
                    i += 1
                }
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1
                }
            }
        }
        out.push(&s[start..i]);
        if out.len() > 24000 {
            return Err("Pfad zu komplex".into());
        }
    }
    Ok(out)
}
fn num(s: &str) -> R<f64> {
    s.parse::<f64>()
        .ok()
        .filter(|n| n.is_finite() && n.abs() < 1e7)
        .ok_or("Ungültige SVG-Zahl".into())
}
fn path(s: &str) -> R<Vec<Vec<DVec2>>> {
    let t = tokens(s)?;
    let mut i = 0;
    let mut cmd = b' ';
    let mut rings = vec![];
    let mut ring = vec![];
    let mut pos = DVec2::ZERO;
    while i < t.len() {
        if t[i].as_bytes()[0].is_ascii_alphabetic() {
            cmd = t[i].as_bytes()[0];
            i += 1
        }
        let old = pos;
        let offset = if cmd.is_ascii_lowercase() {
            old
        } else {
            DVec2::ZERO
        };
        let mut n = || {
            let v = num(t.get(i).copied().unwrap_or(""));
            i += 1;
            v
        };
        let mut point = || -> R<DVec2> { Ok(DVec2::new(n()?, n()?) + offset) };
        match cmd.to_ascii_uppercase() {
            b'M' => {
                if !ring.is_empty() {
                    return Err("Offene Kontur".into());
                }
                pos = point()?;
                ring.push(pos);
                cmd = if cmd == b'M' { b'L' } else { b'l' };
            }
            b'Z' => {
                if ring.first() == ring.last() {
                    ring.pop();
                }
                if ring.len() < 3 {
                    return Err("Leere Kontur".into());
                }
                pos = ring[0];
                rings.push(std::mem::take(&mut ring));
                cmd = b' ';
            }
            _ if ring.is_empty() => return Err("Pfad muss mit M beginnen".into()),
            b'L' => {
                pos = point()?;
                ring.push(pos);
            }
            b'H' => {
                pos.x = n()? + offset.x;
                ring.push(pos)
            }
            b'V' => {
                pos.y = n()? + offset.y;
                ring.push(pos)
            }
            b'Q' => {
                let b = point()?;
                pos = point()?;
                for j in 1..=8 {
                    let t = j as f64 / 8.;
                    ring.push(old.lerp(b, t).lerp(b.lerp(pos, t), t));
                }
            }
            b'C' => {
                let b = point()?;
                let c = point()?;
                pos = point()?;
                for j in 1..=10 {
                    let t = j as f64 / 10.;
                    ring.push(
                        old.lerp(b, t)
                            .lerp(b.lerp(c, t), t)
                            .lerp(b.lerp(c, t).lerp(c.lerp(pos, t), t), t),
                    );
                }
            }
            _ => return Err("Unterstützte Pfadbefehle: M L H V Q C Z".into()),
        }
        if ring.len() > 257 || rings.len() > 64 {
            return Err("SVG-Pfad zu komplex".into());
        }
    }
    if !ring.is_empty() {
        return Err("SVG-Pfad mit Z schließen".into());
    }
    Ok(rings)
}
pub fn load(data: &[u8]) -> R<Relief> {
    let mut reader = Reader::from_reader(data);
    let mut groups = vec![];
    let mut root = false;
    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(e) | Event::Empty(e) => {
                let mut a = BTreeMap::new();
                for item in e.attributes() {
                    let item = item.map_err(|e| e.to_string())?;
                    let k = String::from_utf8_lossy(item.key.as_ref()).into_owned();
                    let v = item
                        .decoded_and_normalized_value(
                            quick_xml::XmlVersion::Implicit1_0,
                            reader.decoder(),
                        )
                        .map_err(|e| e.to_string())?
                        .into_owned();
                    if matches!(
                        k.as_str(),
                        "transform"
                            | "style"
                            | "class"
                            | "mask"
                            | "filter"
                            | "clip-path"
                            | "opacity"
                            | "display"
                            | "visibility"
                    ) || k == "stroke" && v != "none"
                        || k == "fill" && (v == "none" || v.starts_with("url("))
                    {
                        return Err("Darstellungseffekte zuerst in gefüllte Pfade umwandeln".into());
                    }
                    a.insert(k, v);
                }
                let get = |k: &str| a.get(k).map(String::as_str);
                let n = |k: &str, d: &str| num(get(k).unwrap_or(d));
                let tag = e.local_name();
                if !root && tag.as_ref() != b"svg" {
                    return Err("SVG-Wurzel fehlt".into());
                }
                let g = match tag.as_ref() {
                    b"svg" if !root => {
                        root = true;
                        continue;
                    }
                    b"g" | b"title" | b"desc" => continue,
                    b"path" => path(get("d").unwrap_or(""))?,
                    b"polygon" => {
                        let t = tokens(get("points").unwrap_or(""))?;
                        if t.len() % 2 != 0 {
                            return Err("Unvollständiges Punktepaar".into());
                        }
                        vec![t
                            .chunks_exact(2)
                            .map(|a| Ok(DVec2::new(num(a[0])?, num(a[1])?)))
                            .collect::<R<Vec<_>>>()?]
                    }
                    b"rect" => {
                        if get("rx").is_some() || get("ry").is_some() {
                            return Err("Gerundetes Rechteck in Pfad umwandeln".into());
                        }
                        let lo = DVec2::new(n("x", "0")?, n("y", "0")?);
                        let size = DVec2::new(n("width", "")?, n("height", "")?);
                        if size.min_element() <= 0. {
                            return Err("Ungültiges Rechteck".into());
                        }
                        let hi = lo + size;
                        vec![vec![lo, DVec2::new(hi.x, lo.y), hi, DVec2::new(lo.x, hi.y)]]
                    }
                    b"circle" | b"ellipse" => {
                        let c = DVec2::new(n("cx", "0")?, n("cy", "0")?);
                        let (x, y) = if tag.as_ref() == b"circle" {
                            let r = n("r", "")?;
                            (r, r)
                        } else {
                            (n("rx", "")?, n("ry", "")?)
                        };
                        if x.min(y) <= 0. {
                            return Err("Ungültige Ellipse".into());
                        }
                        vec![crate::cad::ellipse(x, y, 48)
                            .into_iter()
                            .map(|p| p + c)
                            .collect()]
                    }
                    _ => return Err("SVG enthält nicht unterstützte Elemente".into()),
                };
                if g.is_empty()
                    || g.iter().any(|r| !(3..=256).contains(&r.len()))
                    || groups.len() >= 64
                {
                    return Err("SVG-Konturgrenze überschritten".into());
                }
                if g.len() > 1 && get("fill-rule") != Some("evenodd") {
                    return Err("Mehrfachkonturen benötigen fill-rule=evenodd am Pfad".into());
                }
                groups.push(g);
            }
            Event::DocType(_) => return Err("DTD in SVG unzulässig".into()),
            Event::Eof => break,
            _ => (),
        }
    }
    if groups.is_empty() {
        return Err("SVG ohne geschlossene Formen".into());
    }
    Ok(Relief {
        source: Art::Logo(groups),
        size: 35.,
        hole: false,
        ..Default::default()
    })
}
