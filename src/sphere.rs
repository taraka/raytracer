use crate::ray::Ray;
use crate::tuple::Tuple;


pub struct Sphere {

}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f32> {
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0,0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        vec![t1, t2]
    }
}