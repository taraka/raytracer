use crate::intersection::*;
use crate::material::Material;
use crate::matrix::*;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;
use crate::EPSILON;

use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    id: Uuid,
    pub transform: Matrix4,
    pub material: Material,
}

impl Plane {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            id: Uuid::new_v4(),
            transform: Matrix4::identity(),
            material: Material::new(),
        })
    }
}

impl Shape for Plane {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn set_transform(&mut self, m: Matrix4) {
        self.transform = m;
    }

    fn get_transform(&self) -> &Matrix4 {
        return &self.transform;
    }

    fn get_mut_transform(&mut self) -> &mut Matrix4 {
        return &mut self.transform;
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    fn get_material(&self) -> &Material {
        return &self.material;
    }

    fn get_mut_material(&mut self) -> &mut Material {
        return &mut self.material;
    }

    fn local_intersect(&self, ray: &Ray) -> Intersections {
        if (ray.direction.y).abs() < EPSILON {
            return Intersections::new(vec![]);
        }

        let t = -ray.origin.y / ray.direction.y;
        return Intersections::new(vec![Intersection::new(t, Box::new(self.clone()))]);
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::plane::*;

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
        let p = Plane::new();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        assert_eq!(0, p.local_intersect(&r).len());
    }

    #[test]
    fn intersect_ray_coplanar() {
        let p = Plane::new();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        assert_eq!(0, p.local_intersect(&r).len());
    }

    #[test]
    fn intersect_ray_above() {
        let p = Plane::new();
        let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.local_intersect(&r);

        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(xs[0].obj.get_id(), p.get_id());
    }

    #[test]
    fn intersect_ray_below() {
        let p = Plane::new();
        let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.local_intersect(&r);

        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(xs[0].obj.get_id(), p.get_id());
    }
}
