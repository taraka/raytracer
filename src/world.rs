use crate::Pattern;
use crate::intersection::*;
use crate::light::PointLight;
use crate::matrix::*;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::*;
use crate::Color;

pub struct World {
    pub objects: Vec<Shape>,
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
        let mut s1 = Shape::sphere();
        let mut s2 = Shape::sphere();

        s1.material.pattern = Pattern::solid(Color::new(0.8, 1.0, 0.6));
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        s2.transform = scaling(0.5, 0.5, 0.5);

        Self {
            objects: vec![s1, s2],
            light: Some(PointLight::new(
                point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            )),
        }
    }

    pub fn add(&mut self, s: Shape) {
        self.objects.push(s)
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
        comps.obj.material.lighting(
            &comps.obj,
            self.light.unwrap(),
            comps.over_point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed(&comps.over_point),
        )
    }

    pub fn color_at(&self, r: &Ray) -> Color {
        let xs = self.intersect(r);

        if let Some(hit) = xs.hit() {
            let comps = hit.prepare_computations(r);

            self.shade_hit(&comps)
        } else {
            Color::black()
        }
    }

    pub fn is_shadowed(&self, p: &Tuple) -> bool {
        let v = self.light.unwrap().position - *p;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(*p, direction);

        if let Some(hit) = self.intersect(&r).hit() {
            hit.t < distance
        } else {
            false
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
        let s = w.objects[0].clone();
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
        let s = w.objects[1].clone();
        let i = Intersection::new(0.5, s);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn shading_miss() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn shading_hit() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_hit_shadowed() {
        let mut w = World::new();
        w.light = Some(PointLight::new(point(0.0, 0.0, -10.0), Color::white()));

        let s1 = Shape::sphere();
        w.add(s1);

        let mut s2 = Shape::sphere();
        s2.transform = translation(0.0, 0.0, 10.0);
        w.add(s2.clone());

        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, s2);
        let comps = i.prepare_computations(&r);

        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn shading_behind() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);

        assert_eq!(c, w.objects[1].material.pattern.color_at(&point(0.0, 0.0, 0.0)));
    }

    #[test]
    fn shadow_nothing_collinear() {
        let w = World::default();
        let p = point(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn shadow_obj_between_light_and_point() {
        let w = World::default();
        let p = point(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&p));
    }

    #[test]
    fn shadow_obj_behind_light() {
        let w = World::default();
        let p = point(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn shadow_obj_behind_point() {
        let w = World::default();
        let p = point(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&p));
    }
}
