use crate::color::Color;
use crate::tuple::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Gradient {
    pub a: Color,
    pub b: Color,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, p: &Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = p.x - p.x.floor();
        self.a + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::gradient::*;
    use crate::pattern::*;

    #[test]
    fn gradient_interpolates_between_colours() {
        let p = Pattern::gradient(Color::white(), Color::black());
        assert_eq!(Color::new(1.0, 1.0, 1.0), p.color_at(&point(0.0, 0.0, 0.0)));
        assert_eq!(
            Color::new(0.75, 0.75, 0.75),
            p.color_at(&point(0.25, 0.0, 0.0))
        );
        assert_eq!(Color::new(0.5, 0.5, 0.5), p.color_at(&point(0.5, 0.0, 0.0)));
        assert_eq!(
            Color::new(0.25, 0.25, 0.25),
            p.color_at(&point(0.75, 0.0, 0.0))
        );
    }
}
