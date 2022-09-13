use crate::Color;
use crate::sphere::Sphere;
use crate::light::PointLight;
use crate::tuple::*;
use crate::matrix::Matrix4;

pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Option<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            light: None,
        }
    }

    pub fn default() -> Self {
        let mut s1 = Sphere::new();
        let mut s2 = Sphere::new();

        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        s2.set_transform(Matrix4::scaling(0.5, 0.5, 0.5));

        Self {
            objects: vec![s1, s2],
            light: Some(PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::world::*;

    #[test]
    fn a_whole_new_world() {
        let w = World::new();
        assert_eq!(None, w.light);
        assert_eq!(0, w.objects.len());
    }

    #[test]
    fn default_world() {
        let w = World::default();

        assert_eq!(Some(PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0))), w.light);
        assert_eq!(2, w.objects.len());
    }

}