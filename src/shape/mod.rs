mod plane;
mod sphere;

use crate::intersection::*;
use crate::material::*;
use crate::matrix::*;
use crate::ray::*;
use crate::shape::plane::Plane;
use crate::shape::sphere::Sphere;
use crate::tuple::*;
use crate::EPSILON;

use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Shape {
    id: Uuid,
    pub shape: Shapes,
    pub transform: Matrix4,
    pub material: Material,
}

impl Shape {
    pub fn new(shape: Shapes) -> Self {
        Self {
            id: Uuid::new_v4(),
            shape: shape,
            transform: Matrix4::identity(),
            material: Material::new(),
        }
    }

    pub fn sphere() -> Self {
        Self::new(Shapes::Sphere(Sphere::new()))
    }

    pub fn plane() -> Self {
        Self::new(Shapes::Plane(Plane::new()))
    }

    pub fn test() -> Self {
        Self::new(Shapes::Test(TestShape::new()))
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn intersect(&self, r: &Ray) -> Intersections {
        let local_ray = r.transform(self.transform.inverse());

        match self.shape {
            Shapes::Sphere(s) => Sphere::local_intersect(self.clone(), &local_ray),
            Shapes::Plane(s) => Plane::local_intersect(self.clone(), &local_ray),
            Shapes::Test(s) => TestShape::local_intersect(self.clone(), &local_ray),
        }
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let local_point = self.transform.inverse() * p;

        let local_normal = match self.shape {
            Shapes::Sphere(s) => s.local_normal_at(local_point),
            Shapes::Plane(s) => s.local_normal_at(local_point),
            Shapes::Test(s) => s.local_normal_at(local_point),
        };

        let mut world_normal = self.transform.inverse().transpose() * local_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shapes {
    Sphere(Sphere),
    Plane(Plane),
    Test(TestShape),
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct TestShape {}

impl TestShape {
    pub fn new() -> Self {
        Self {}
    }

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
    use crate::shape::*;

    #[test]
    fn default_transform() {
        let s = Shape::test();
        assert_eq!(s.transform, Matrix4::identity());
    }

    #[test]
    fn assign_transform() {
        let mut s = Shape::test();
        s.transform = translation(2.0, 3.0, 4.0);
        assert_eq!(s.transform, translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn default_material() {
        let s = Shape::test();
        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn assign_material() {
        let mut s = Shape::test();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material.ambient, 1.0);
    }
}
