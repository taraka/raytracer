use crate::color::Color;
use crate::tuple::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Stripe {
    pub a: Color,
    pub b: Color,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, p: &Tuple) -> Color {
        if p.x.floor().abs() as isize % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::stripe::*;
    use crate::pattern::*;

    #[test]
    fn create_stripe_pattern() {
        let p = Stripe::new(Color::black(), Color::white());
        assert_eq!(p.a, Color::black());
        assert_eq!(p.b, Color::white());
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let p = Stripe::new(Color::white(), Color::black());
        assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let p = Stripe::new(Color::white(), Color::black());
        assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_altinates_in_x() {
        let p = Stripe::new(Color::white(), Color::black());
        assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.color_at(&point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(p.color_at(&point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.color_at(&point(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_pattern__with_object_transform() {
        let mut s = Shape::sphere();
        s.transform = scaling(2.0, 2.0, 2.0);
        let p = Pattern::stripe(Color::white(), Color::black());

        assert_eq!(Color::white(), p.color_at_object(&s, &point(1.5, 0.0, 0.0)));
    }

    #[test]
    fn stripe_pattern_transform() {
        let s = Shape::sphere();
        let mut p = Pattern::stripe(Color::white(), Color::black());
        p.transform = scaling(2.0, 2.0, 2.0);

        assert_eq!(Color::white(), p.color_at_object(&s, &point(1.5, 0.0, 0.0)));
    }

    #[test]
    fn stripe_pattern_and_obj_transform() {
        let mut s = Shape::sphere();
        let mut p = Pattern::stripe(Color::white(), Color::black());
        s.transform = scaling(2.0, 2.0, 2.0);
        p.transform = translation(0.5, 0.0, 0.0);

        assert_eq!(Color::white(), p.color_at_object(&s, &point(2.5, 0.0, 0.0)));
    }
}
