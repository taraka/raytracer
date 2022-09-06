
use crate::tuple::Tuple;
use crate::sphere::Sphere;

pub struct Ray {
    pub origin: Tuple,  
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::tuple::Tuple;
    use crate::sphere::Sphere;

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
        assert_eq!(4.0, xs[0]);
        assert_eq!(6.0, xs[1]);
    }

    #[test]
    fn ray_intersects_sphere_tangent() {
        let o = Tuple::point(0.0, 1.0, -5.0);
        let d = Tuple::vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let xs = s.intersect(r);


        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0]);
        assert_eq!(5.0, xs[1]);
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
        assert_eq!(-1.0, xs[0]);
        assert_eq!(1.0, xs[1]);
    }

    #[test]
    fn ray_behind_sphere() {
        let o = Tuple::point(0.0, 0.0, 5.0);
        let d = Tuple::vector(0.0, 0.0, 1.0);
        let r = Ray::new(o, d);

        let s = Sphere::new();

        let xs = s.intersect(r);


        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0]);
        assert_eq!(-4.0, xs[1]);
    }
}