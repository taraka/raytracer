use crate::color::Color;
use crate::pattern::*;
use crate::tuple::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Blended {
    pub a: Box<Pattern>,
    pub b: Box<Pattern>,
}

impl Blended {
    pub fn new(a: Pattern, b: Pattern) -> Self {
        Self { 
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    pub fn color_at(&self, p: &Tuple) -> Color {
        let a = self.a.color_at(p);
        let b = self.b.color_at(p);
        (a + b) * 0.5
    }
}
