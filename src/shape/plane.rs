use crate::Shape;
use crate::intersection::*;
use crate::material::Material;
use crate::matrix::*;
use crate::ray::Ray;
use crate::tuple::*;
use crate::EPSILON;

use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Self {
        Self { }
    }
}

impl Plane {
    pub fn local_intersect(shape: Shape, ray: &Ray) -> Intersections {
        if (ray.direction.y).abs() < EPSILON {
            return Intersections::new(vec![]);
        }

        let t = -ray.origin.y / ray.direction.y;
        return Intersections::new(vec![Intersection::new(t, shape)]);
    }

    pub fn local_normal_at(&self, _: Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::shape::plane::*;

    #[test]
    fn normal_of_plane_is_constant() {
        let p = Plane::new();
        let n1 = p.local_normal_at(point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(point(-5.0, 0.0, 150.0));

        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_ray_parallel() {
        let p = Shape::plane();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        assert_eq!(0, p.intersect(&r).len());
    }

    #[test]
    fn intersect_ray_coplanar() {
        let p = Shape::plane();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        assert_eq!(0, p.intersect(&r).len());
    }

    #[test]
    fn intersect_ray_above() {
        let p = Shape::plane();
        let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.intersect(&r);

        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(xs[0].obj.get_id(), p.get_id());
    }

    #[test]
    fn intersect_ray_below() {
        let p = Shape::plane();
        let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.intersect(&r);

        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(xs[0].obj.get_id(), p.get_id());
    }
}
