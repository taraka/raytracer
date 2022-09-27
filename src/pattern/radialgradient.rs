use crate::color::Color;
use crate::tuple::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RadialGradient {
    pub a: Color,
    pub b: Color,
}

impl RadialGradient {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, p: &Tuple) -> Color {
        let d = (p.x * p.x + p.z * p.z).sqrt();
        let distance = self.b - self.a;
        let fraction = d - d.floor();
        self.a + distance * fraction
    }
}
