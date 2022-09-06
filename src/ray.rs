
use crate::tuple::Tuple;

pub struct Ray {
    origin: Tuple,  
    direction: Tuple,
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
    }
}