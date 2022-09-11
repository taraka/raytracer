use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::light::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let i = Color::new(1.0, 1.0, 1.0);
        let p = Tuple::point(0.0, 0.0, 0.0);
        let l = PointLight::new(p, i);

        assert_eq!(p, l.position);
        assert_eq!(i, l.intensity);
    }
}
