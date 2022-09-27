use crate::Tuple;
use crate::Color;
use crate::pattern::stripe::StripePattern;

mod stripe;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pattern {
    pub pattern: Patterns
}

impl Pattern {
    pub fn stripe(a: Color, b: Color) -> Self {
        Self {
            pattern: Patterns::Stripe(StripePattern::new(a, b))
        }
    }

    pub fn color_at(&self, t: &Tuple) -> Color {
        match self.pattern {
            Patterns::Stripe(p) => p.color_at(t),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Patterns {
    Stripe(StripePattern)
}