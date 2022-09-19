
use crate::matrix::*;
use crate::tuple::*;
use crate::material::*;
use crate::ray::*;
use crate::intersection::*;

pub trait Shape {
    fn set_transform(&mut self, m: Matrix4);
    fn get_transform(&self) -> &Matrix4;

    fn set_material(&mut self, m: Material);
    fn get_material(&self) -> &Material;

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



struct TestShape {
    transform: Matrix4,
    material: Material,
}

impl TestShape {
    fn new() -> Self {
        Self {
            transform: Matrix4::identity(),
            material: Material::new(),
        }
    }
}

impl Shape for TestShape {
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

    fn local_intersect(&self, _: &Ray) -> Intersections {
        Intersections::new(vec![])
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
    }
}


#[cfg(test)]
mod tests {
    use crate::shape::*;

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