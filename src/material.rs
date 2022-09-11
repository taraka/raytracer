
use crate::color::Color;
use crate::FP;

pub struct Material {
    color: Color,
    ambient: FP,
    diffuse: FP,
    specular: FP,
    shininess: FP,
}

impl Material{
    fn new() -> Self {
        Self {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::material::Material;
    use crate::color::Color;

    #[test]
    fn default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::white());
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}