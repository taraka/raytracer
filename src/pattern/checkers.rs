use crate::color::Color;
use crate::tuple::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Checkers {
    pub a: Color,
    pub b: Color,
}

impl Checkers {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, p: &Tuple) -> Color {
        if (p.x.floor() + p.y.floor() + p.z.floor()).round() as isize % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::checkers::*;
    use crate::pattern::*;

    #[test]
    fn checkers_repeat_in_x() {
        let p = Pattern::checkers(Color::white(), Color::black());
        assert_eq!(Color::white(), p.color_at(&point(0.0, 0.0, 0.0)));
        assert_eq!(Color::white(), p.color_at(&point(0.99, 0.0, 0.0)));
        assert_eq!(Color::black(), p.color_at(&point(1.01, 0.0, 0.0)));
    }

    #[test]
    fn checkers_repeat_in_y() {
        let p = Pattern::checkers(Color::white(), Color::black());
        assert_eq!(Color::white(), p.color_at(&point(0.0, 0.0, 0.0)));
        assert_eq!(Color::white(), p.color_at(&point(0.0, 0.99, 0.0)));
        assert_eq!(Color::black(), p.color_at(&point(0.0, 1.01, 0.0)));
    }

    #[test]
    fn checkers_repeat_in_z() {
        let p = Pattern::checkers(Color::white(), Color::black());
        assert_eq!(Color::white(), p.color_at(&point(0.0, 0.0, 0.0)));
        assert_eq!(Color::white(), p.color_at(&point(0.0, 0.0, 0.99)));
        assert_eq!(Color::black(), p.color_at(&point(0.0, 0.0, 1.01)));
    }
}
