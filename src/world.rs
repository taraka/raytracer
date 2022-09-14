use crate::intersection::*;
use crate::light::PointLight;
use crate::matrix::*;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::*;
use crate::Color;

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

        s2.set_transform(scaling(0.5, 0.5, 0.5));

        Self {
            objects: vec![s1, s2],
            light: Some(PointLight::new(
                point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            )),
        }
    }

    pub fn intersect(&self, r: &Ray) -> Intersections {
        let mut v: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|o| o.intersect(r).intersections)
            .collect();
        v.sort();
        Intersections::new(v)
    }

    pub fn shade_hit(&self, comps: &Computations) -> Color {
        comps
            .obj
            .material
            .lighting(self.light.unwrap(), comps.point, comps.eyev, comps.normalv)
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

        assert_eq!(
            Some(PointLight::new(
                point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0)
            )),
            w.light
        );
        assert_eq!(2, w.objects.len());
    }

    #[test]
    fn intersect_world_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let w = World::default();

        let xs = w.intersect(&r);

        assert_eq!(4, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6.0, xs[3].t);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = w.objects[0];
        let i = Intersection::new(4.0, s);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(point(0.0, 0.25, 0.0), Color::white()));
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = w.objects[1];
        let i = Intersection::new(0.5, s);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }
}
