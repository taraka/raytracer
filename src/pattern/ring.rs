use crate::color::Color;
use crate::tuple::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ring {
    pub a: Color,
    pub b: Color,
}

impl Ring {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, p: &Tuple) -> Color {
        if (p.x * p.x + p.z * p.z).sqrt() as isize % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::ring::*;
    use crate::pattern::*;

    #[test]
    fn gradient_interpolates_between_colours() {
        let p = Pattern::ring(Color::white(), Color::black());
        assert_eq!(Color::white(), p.color_at(&point(0.0, 0.0, 0.0)));
        assert_eq!(Color::black(), p.color_at(&point(1.0, 0.0, 0.0)));
        assert_eq!(Color::black(), p.color_at(&point(0.0, 0.0, 1.0)));
        assert_eq!(Color::black(), p.color_at(&point(0.708, 0.0, 0.708)));
    }
}
