
use crate::color::Color;
use crate::tuple::Tuple;

pub struct Light {
    intensity: Color,
    position: Tuple,
}

impl Light{
    fn new(intensity: Color, position: Tuple) -> Self {
        Self {
            intensity,
            position,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::tuple::Tuple;
    use crate::light::Light;

    #[test]
    fn point_light_has_position_and_intensity() {
        let i = Color::new(1.0, 1.0, 1.0);
        let p = Tuple::point(0.0, 0.0, 0.0);
        let l = Light::new(i, p);

        assert_eq!(p, l.position);
        assert_eq!(i, l.intensity);
    }
}