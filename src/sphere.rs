use crate::intersection::*;
use crate::material::Material;
use crate::matrix::*;
use crate::ray::Ray;
use crate::tuple::*;
use crate::shape::*;

use uuid::Uuid;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    id: Uuid,
    pub transform: Matrix4,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrix4::identity(),
            material: Material::new(),
        }
    }
}

impl Shape for Sphere {
    fn set_transform(&mut self, m: Matrix4) {
        self.transform = m;
    }

    fn get_transform(&self) -> &Matrix4 {
        return &self.transform;
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    fn get_material(&self) -> &Material {
        return &self.material;
    }

    fn local_intersect(&self, ray: &Ray) -> Intersections {
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::new(vec![]);
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Intersections::new(vec![
            Intersection::new(t1, *self),
            Intersection::new(t2, *self),
        ])
    }

    fn local_normal_at(&self, object_point: Tuple) -> Tuple {
        object_point - point(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::sphere::*;
    use crate::FP;
    use std::f64::consts::PI;

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::new();
        assert_eq!(Matrix4::identity(), s.transform);
    }

    #[test]
    fn normal_at_point_on_x() {
        let s = Sphere::new();
        let n = s.normal_at(point(1.0, 0.0, 0.0));
        assert_eq!(vector(1.0, 0.0, 0.0), n);
    }

    #[test]
    fn normal_at_point_on_y() {
        let s = Sphere::new();
        let n = s.normal_at(point(0.0, 1.0, 0.0));
        assert_eq!(vector(0.0, 1.0, 0.0), n);
    }

    #[test]
    fn normal_at_point_on_z() {
        let s = Sphere::new();
        let n = s.normal_at(point(0.0, 0.0, 1.0));
        assert_eq!(vector(0.0, 0.0, 1.0), n);
    }

    #[test]
    fn normal_at_point_on_nonaxial() {
        let s = Sphere::new();
        let v = (3.0 as FP).sqrt() / 3.0;
        let n = s.normal_at(point(v, v, v));
        assert_eq!(vector(v, v, v), n);
    }

    #[test]
    fn normal_of_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(translation(0.0, 1.0, 0.0));

        let n = s.normal_at(point(0.0, 1.70711, -0.70711));
        assert_eq!(vector(0.0, 0.70711, -0.70711), n);
    }

    #[test]
    fn normal_of_transformed_sphere() {
        let mut s = Sphere::new();
        s.set_transform(scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0));

        let n = s.normal_at(point(
            0.0,
            (2.0 as FP).sqrt() / 2.0,
            -(2.0 as FP).sqrt() / 2.0,
        ));
        assert_eq!(vector(0.0, 0.97014, -0.24254), n);
    }

    #[test]
    fn assign_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;

        assert_eq!(m, s.material);
    }
}
