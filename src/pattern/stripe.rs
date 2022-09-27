use crate::color::Color;
use crate::tuple::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
        }
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

    #[test]
    fn create_stripe_pattern() {
        let p = StripePattern::new(Color::black(), Color::white());
        assert_eq!(p.a, Color::black());
        assert_eq!(p.b, Color::white());
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let p = StripePattern::new(Color::white(), Color::black());
        assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 2.0, 0.0)), Color::white());   
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let p = StripePattern::new(Color::white(), Color::black());
        assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(p.color_at(&point(0.0, 0.0, 2.0)), Color::white());   
    }

    #[test]
    fn stripe_pattern_altinates_in_x() {
        let p = StripePattern::new(Color::white(), Color::black());
        assert_eq!(p.color_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(p.color_at(&point(1.0, 0.0, 0.0)), Color::black());   
        assert_eq!(p.color_at(&point(-0.1, 0.0, 0.0)), Color::black());   
        assert_eq!(p.color_at(&point(-1.0, 0.0, 0.0)), Color::black()); 
        assert_eq!(p.color_at(&point(-1.1, 0.0, 0.0)), Color::white()); 
    }

    // #[test]
    // fn stripe_pattern__with_object_transform() {
    //     let p = StripePattern::new(Color::white(), Color::black());
    // }
}