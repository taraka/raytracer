use crate::intersection::*;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::Tuple;

use uuid::Uuid;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    id: Uuid,
    pub transform: Matrix4,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrix4::identity(),
        }
    }

    pub fn intersect(&self, r: Ray) -> Intersections {
        let ray = r.transform(self.transform.inverse());

        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
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

    pub fn set_transform(&mut self, m: Matrix4) {
        self.transform = m;
    }
}

#[cfg(test)]
mod tests {
    use crate::sphere::Matrix4;
    use crate::sphere::Sphere;

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::new();
        assert_eq!(Matrix4::identity(), s.transform);
    }
}
