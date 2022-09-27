use crate::Tuple;
use crate::Color;
use crate::matrix::*;
use crate::shape::*;
use crate::pattern::stripe::StripePattern;

mod stripe;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pattern {
    pub pattern: Patterns,
    pub transform: Matrix4,
}

impl Pattern {
    pub fn stripe(a: Color, b: Color) -> Self {
        Self {
            pattern: Patterns::Stripe(StripePattern::new(a, b)),
            transform: Matrix4::identity(),
        }
    }

    pub fn color_at(&self, t: &Tuple) -> Color {
        match self.pattern {
            Patterns::Stripe(p) => p.color_at(t),
        }
    }

    pub fn color_at_object(&self, obj: &Shape, p: &Tuple) -> Color {
        let obj_point = obj.transform.inverse() * *p;
        let pattern_point = self.transform.inverse() * obj_point;
        self.color_at(&pattern_point)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Patterns {
    Stripe(StripePattern)
}