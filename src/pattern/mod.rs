mod blended;
mod checkers;
mod gradient;
mod radialgradient;
mod ring;
mod stripe;

use crate::matrix::*;
use crate::pattern::blended::Blended;
use crate::pattern::checkers::Checkers;
use crate::pattern::gradient::Gradient;
use crate::pattern::radialgradient::RadialGradient;
use crate::pattern::ring::Ring;
use crate::pattern::stripe::Stripe;
use crate::shape::*;
use crate::Color;
use crate::Tuple;

#[derive(Debug, PartialEq, Clone)]
pub struct Pattern {
    pub pattern: Patterns,
    pub transform: Matrix4,
}

impl Pattern {
    pub fn new(pattern: Patterns) -> Self {
        Self {
            pattern,
            transform: Matrix4::identity(),
        }
    }

    pub fn blended(a: Pattern, b: Pattern) -> Self {
        Self::new(Patterns::Blended(Blended::new(a, b)))
    }

    pub fn checkers(a: Color, b: Color) -> Self {
        Self::new(Patterns::Checkers(Checkers::new(a, b)))
    }

    pub fn gradient(a: Color, b: Color) -> Self {
        Self::new(Patterns::Gradient(Gradient::new(a, b)))
    }

    pub fn radialgradient(a: Color, b: Color) -> Self {
        Self::new(Patterns::RadialGradient(RadialGradient::new(a, b)))
    }

    pub fn ring(a: Color, b: Color) -> Self {
        Self::new(Patterns::Ring(Ring::new(a, b)))
    }

    pub fn stripe(a: Color, b: Color) -> Self {
        Self::new(Patterns::Stripe(Stripe::new(a, b)))
    }

    pub fn solid(c: Color) -> Self {
        Self::new(Patterns::Solid(c))
    }

    pub fn color_at(&self, obj_point: &Tuple) -> Color {
        let t = self.transform.inverse() * *obj_point;
        match self.pattern.clone() {
            Patterns::Blended(p) => p.color_at(&t),
            Patterns::Checkers(p) => p.color_at(&t),
            Patterns::Gradient(p) => p.color_at(&t),
            Patterns::RadialGradient(p) => p.color_at(&t),
            Patterns::Ring(p) => p.color_at(&t),
            Patterns::Solid(c) => c,
            Patterns::Stripe(p) => p.color_at(&t),
        }
    }

    pub fn color_at_object(&self, obj: &Shape, p: &Tuple) -> Color {
        let obj_point = obj.transform.inverse() * *p;
        self.color_at(&obj_point)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Patterns {
    Blended(Blended),
    Checkers(Checkers),
    Gradient(Gradient),
    RadialGradient(RadialGradient),
    Ring(Ring),
    Solid(Color),
    Stripe(Stripe),
}
