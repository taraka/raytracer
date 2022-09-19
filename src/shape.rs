use crate::intersection::*;
use crate::material::*;
use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;

use uuid::Uuid;

pub trait Shape: std::fmt::Debug + ShapeClone {
    fn get_id(&self) -> Uuid;

    fn set_transform(&mut self, m: Matrix4);
    fn get_transform(&self) -> &Matrix4;
    fn get_mut_transform(&mut self) -> &mut Matrix4;

    fn set_material(&mut self, m: Material);
    fn get_material(&self) -> &Material;
    fn get_mut_material(&mut self) -> &mut Material;

    fn intersect(&self, r: &Ray) -> Intersections {
        let local_ray = r.transform(self.get_transform().inverse());
        self.local_intersect(&local_ray)
    }

    fn local_intersect(&self, r: &Ray) -> Intersections;

    fn normal_at(&self, p: Tuple) -> Tuple {
        let local_point = self.get_transform().inverse() * p;
        let local_normal = self.local_normal_at(local_point);
        let mut world_normal = self.get_transform().inverse().transpose() * local_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    fn local_normal_at(&self, p: Tuple) -> Tuple;
}

pub trait ShapeClone {
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl<T> ShapeClone for T
where
    T: 'static + Shape + Clone,
{
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
struct TestShape {
    transform: Matrix4,
    material: Material,
}

#[cfg(test)]
mod tests {
    use crate::shape::*;

    impl TestShape {
        fn new() -> Self {
            Self {
                transform: Matrix4::identity(),
                material: Material::new(),
            }
        }
    }

    impl Shape for TestShape {
        fn get_id(&self) -> Uuid {
            Uuid::new_v4()
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

        fn local_intersect(&self, _: &Ray) -> Intersections {
            Intersections::new(vec![])
        }

        fn local_normal_at(&self, _: Tuple) -> Tuple {
            vector(0.0, 1.0, 0.0)
        }
    }

    #[test]
    fn default_transform() {
        let s = TestShape::new();
        assert_eq!(s.get_transform(), &Matrix4::identity());
    }

    #[test]
    fn assign_transform() {
        let mut s = TestShape::new();
        s.set_transform(translation(2.0, 3.0, 4.0));
        assert_eq!(s.get_transform(), &translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn default_material() {
        let s = TestShape::new();
        assert_eq!(s.get_material(), &Material::new());
    }

    #[test]
    fn assign_material() {
        let mut s = TestShape::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.set_material(m);
        assert_eq!(s.get_material().ambient, 1.0);
    }
}
