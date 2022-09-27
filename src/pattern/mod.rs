mod checkers;
mod gradient;
mod ring;
mod solid;
mod stripe;

use crate::matrix::*;
use crate::pattern::checkers::Checkers;
use crate::pattern::gradient::Gradient;
use crate::pattern::ring::Ring;
use crate::pattern::solid::Solid;
use crate::pattern::stripe::Stripe;
use crate::shape::*;
use crate::Color;
use crate::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
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

    pub fn checkers(a: Color, b: Color) -> Self {
        Self::new(Patterns::Checkers(Checkers::new(a, b)))
    }

    pub fn gradient(a: Color, b: Color) -> Self {
        Self::new(Patterns::Gradient(Gradient::new(a, b)))
    }

    pub fn ring(a: Color, b: Color) -> Self {
        Self::new(Patterns::Ring(Ring::new(a, b)))
    }

    pub fn stripe(a: Color, b: Color) -> Self {
        Self::new(Patterns::Stripe(Stripe::new(a, b)))
    }

    pub fn solid(c: Color) -> Self {
        Self::new(Patterns::Solid(Solid::new(c)))
    }

    pub fn color_at(&self, t: &Tuple) -> Color {
        match self.pattern {
            Patterns::Checkers(p) => p.color_at(t),
            Patterns::Gradient(p) => p.color_at(t),
            Patterns::Ring(p) => p.color_at(t),
            Patterns::Solid(p) => p.color_at(t),
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
    Checkers(Checkers),
    Gradient(Gradient),
    Ring(Ring),
    Solid(Solid),
    Stripe(Stripe),
}
