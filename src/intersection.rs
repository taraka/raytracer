use crate::sphere::Sphere;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection {
    pub t: f64,
    pub obj: Sphere,
}

impl Intersection {
    pub fn new(t: f64, obj: Sphere) -> Self {
        Self { t, obj }
    }
}

pub struct Intersections {
    pub intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Self {
        Self { intersections }
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn hit(&self) -> Option<Intersection> {
        let mut candidates = self
            .intersections
            .iter()
            .filter(|i| i.t >= 0.0)
            .collect::<Vec<&Intersection>>();
        candidates.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        candidates.get(0).map(|x| x.to_owned().to_owned())
    }
}

impl ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::intersection::*;
    use crate::sphere::Sphere;

    #[test]
    fn interections() {
        let s = Sphere::new();
        let a = Intersection::new(1.0, s);
        let b = Intersection::new(2.0, s);
        let i = Intersections::new(vec![a, b]);

        assert_eq!(2, i.len());
        assert_eq!(a, i[0]);
    }

    #[test]
    fn hit_all_pos() {
        let s = Sphere::new();
        let a = Intersection::new(1.0, s);
        let b = Intersection::new(2.0, s);
        let i = Intersections::new(vec![a, b]);

        assert_eq!(a, i.hit().unwrap());
    }

    #[test]
    fn hit_neg_pos() {
        let s = Sphere::new();
        let a = Intersection::new(-1.0, s);
        let b = Intersection::new(2.0, s);
        let i = Intersections::new(vec![a, b]);

        assert_eq!(b, i.hit().unwrap());
    }

    #[test]
    fn hit_neg() {
        let s = Sphere::new();
        let a = Intersection::new(-1.0, s);
        let b = Intersection::new(-2.0, s);
        let i = Intersections::new(vec![a, b]);

        assert_eq!(None, i.hit());
    }

    #[test]
    fn hit_order() {
        let s = Sphere::new();
        let a = Intersection::new(5.0, s);
        let b = Intersection::new(7.0, s);
        let c = Intersection::new(-3.0, s);
        let d = Intersection::new(2.0, s);
        let i = Intersections::new(vec![a, b, c, d]);

        assert_eq!(d, i.hit().unwrap());
    }
}
