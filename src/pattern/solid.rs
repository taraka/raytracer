use crate::color::Color;
use crate::tuple::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Solid {
    pub c: Color,
}

impl Solid {
    pub fn new(c: Color) -> Self {
        Self { c }
    }

    pub fn color_at(&self, p: &Tuple) -> Color {
        self.c
    }
}
