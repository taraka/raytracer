use crate::matrix::Matrix4;
use crate::tuple::Tuple;
use crate::FP;

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: FP) -> Tuple {
        self.origin + (self.direction * t)
    }

    pub fn transform(&self, m: Matrix4) -> Self {
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::*;
    use crate::Sphere;

    #[test]
    fn create_ray() {
        let o = Tuple::point(1.0, 2.0, 3.0);
        let d = Tuple::vector(4.0, 5.0, 6.0);

        let r = Ray::new(o, d);
        assert_eq!(o, r.origin);
        assert_eq!(d, r.direction);
    }

    #[test]
    fn point_from_distance() {
        let o = Tuple::point(2.0, 3.0, 4.0);
        let d = Tuple::vector(1.0, 0.0, 0.0);
        let r = Ray::new(o, d);

        assert_eq!(Tuple::point(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(Tuple::point(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(Tuple::point(4.5, 3.0, 4.0), r.position(2.5));
    }

    #[test]
    fn ray_intersects_sphere_twice() {
        let o = Tuple::point(0.0, 0.0, -5.0);
        let d = Tuple::vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);
    }

    #[test]
    fn ray_intersects_sphere_tangent() {
        let o = Tuple::point(0.0, 1.0, -5.0);
        let d = Tuple::vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
    }

    #[test]
    fn ray_misses_sphere() {
        let o = Tuple::point(0.0, 2.0, -5.0);
        let d = Tuple::vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn ray_inside_sphere() {
        let o = Tuple::point(0.0, 0.0, 00.0);
        let d = Tuple::vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn ray_behind_sphere() {
        let o = Tuple::point(0.0, 0.0, 5.0);
        let d = Tuple::vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn translate_ray() {
        let o = Tuple::point(1.0, 2.0, 3.0);
        let d = Tuple::vector(0.0, 1.0, 0.0);
        let r = Ray::new(o, d);

        let m = Matrix4::translation(3.0, 4.0, 5.0);

        let r2 = r.transform(m);
        assert_eq!(Tuple::point(4.0, 6.0, 8.0), r2.origin);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), r2.direction);
    }

    #[test]
    fn scale_ray() {
        let o = Tuple::point(1.0, 2.0, 3.0);
        let d = Tuple::vector(0.0, 1.0, 0.0);
        let r = Ray::new(o, d);

        let m = Matrix4::scaling(2.0, 3.0, 4.0);

        let r2 = r.transform(m);
        assert_eq!(Tuple::point(2.0, 6.0, 12.0), r2.origin);
        assert_eq!(Tuple::vector(0.0, 3.0, 0.0), r2.direction);
    }

    #[test]
    fn intersect_ray_with_scaled_sphere() {
        let o = Tuple::point(0.0, 0.0, -5.0);
        let d = Tuple::vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);
        let mut s = Sphere::new();
        s.set_transform(Matrix4::scaling(2.0, 2.0, 2.0));

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersect_ray_with_translated_sphere() {
        let o = Tuple::point(0.0, 0.0, -5.0);
        let d = Tuple::vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);
        let mut s = Sphere::new();
        s.set_transform(Matrix4::translation(5.0, 0.0, 0.0));

        let xs = s.intersect(r);

        assert_eq!(0, xs.len());
    }
}
