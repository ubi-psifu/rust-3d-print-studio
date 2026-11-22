use super::{require, Feature, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Binding {
    pub target: String,
    pub expression: String,
}
pub fn drive(
    source: &Feature,
    vars: &BTreeMap<String, f64>,
    bindings: &[Binding],
) -> Result<Feature> {
    require(
        vars.len() <= 32 && bindings.len() <= 64,
        "Zu viele Variablen oder Beziehungen",
    )?;
    let mut data = serde_json::to_value(source).map_err(|e| e.to_string())?;
    for b in bindings {
        let slot = data
            .pointer_mut(&b.target)
            .ok_or("Zieldimension existiert nicht")?;
        require(
            slot.is_f64() && b.expression.len() <= 128,
            "Ziel muss ein Dezimalmaß sein; Formel maximal 128 Zeichen",
        )?;
        let text = b.expression.replace(' ', "");
        let mut parser = Parser {
            src: text.as_bytes(),
            vars,
            index: 0,
        };
        let result = parser.expr(0)?;
        require(
            parser.index == text.len() && result.is_finite(),
            "Ungültige Maßformel",
        )?;
        *slot = result.into();
    }
    serde_json::from_value(data).map_err(|_| "Abgeleitete Maße unzulässig".into())
}
struct Parser<'a> {
    src: &'a [u8],
    vars: &'a BTreeMap<String, f64>,
    index: usize,
}
impl Parser<'_> {
    fn peek(&self) -> u8 {
        self.src.get(self.index).copied().unwrap_or(0)
    }
    fn expr(&mut self, prec: u8) -> Result<f64> {
        let mut x = self.atom()?;
        loop {
            let op = self.peek();
            let p = match op {
                b'+' | b'-' => 1,
                b'*' | b'/' => 2,
                _ => 0,
            };
            if p <= prec {
                break;
            }
            self.index += 1;
            let y = self.expr(p)?;
            x = match op {
                b'+' => x + y,
                b'-' => x - y,
                b'*' => x * y,
                _ => x / y,
            };
        }
        Ok(x)
    }
    fn atom(&mut self) -> Result<f64> {
        let c = self.peek();
        match c {
            b'(' => {
                self.index += 1;
                let x = self.expr(0)?;
                require(self.peek() == b')', "Schließende Klammer fehlt")?;
                self.index += 1;
                Ok(x)
            }
            b'-' => {
                self.index += 1;
                Ok(-self.atom()?)
            }
            _ => {
                let start = self.index;
                if c.is_ascii_alphabetic() || c == b'_' {
                    while self.peek().is_ascii_alphanumeric() || self.peek() == b'_' {
                        self.index += 1;
                    }
                    let name = std::str::from_utf8(&self.src[start..self.index])
                        .map_err(|_| "Variablenname ungültig")?;
                    self.vars
                        .get(name)
                        .copied()
                        .ok_or_else(|| format!("Variable {name} fehlt"))
                } else {
                    while self.peek().is_ascii_digit() || self.peek() == b'.' {
                        self.index += 1;
                    }
                    std::str::from_utf8(&self.src[start..self.index])
                        .unwrap_or("")
                        .parse()
                        .map_err(|_| "Zahl oder Variable erwartet".into())
                }
            }
        }
    }
}
