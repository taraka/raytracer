use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;
use crate::EPSILON;
use std::cmp::Ordering;
use std::ops;

use crate::FP;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: FP,
    pub obj: Shape,
}

#[derive(Debug)]
pub struct Computations {
    pub t: FP,
    pub obj: Shape,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl Intersection {
    pub fn new(t: FP, obj: Shape) -> Self {
        Self { t, obj }
    }

    pub fn prepare_computations(&self, r: &Ray) -> Computations {
        let point = r.position(self.t);
        let eyev = -r.direction;
        let mut normalv = self.obj.normal_at(point);

        let inside = normalv.dot(&eyev) < 0.0;

        if inside {
            normalv = -normalv;
        }

        let over_point = point + (normalv * (EPSILON / 10000000.0) );

        Computations {
            t: self.t,
            obj: self.obj.clone(),
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl Eq for Intersection {}

#[derive(Debug, PartialEq)]
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
        candidates.get(0).map(|x| x.to_owned().clone())
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
    use crate::matrix::*;

    #[test]
    fn interections() {
        let s = Shape::sphere();
        let a = Intersection::new(1.0, s.clone());
        let b = Intersection::new(2.0, s);
        let i = Intersections::new(vec![a.clone(), b]);

        assert_eq!(2, i.len());
        assert_eq!(a, i[0]);
    }

    #[test]
    fn hit_all_pos() {
        let s = Shape::sphere();
        let a = Intersection::new(1.0, s.clone());
        let b = Intersection::new(2.0, s);
        let i = Intersections::new(vec![a.clone(), b]);

        assert_eq!(a, i.hit().unwrap());
    }

    #[test]
    fn hit_neg_pos() {
        let s = Shape::sphere();
        let a = Intersection::new(-1.0, s.clone());
        let b = Intersection::new(2.0, s);
        let i = Intersections::new(vec![a, b.clone()]);

        assert_eq!(b, i.hit().unwrap());
    }

    #[test]
    fn hit_neg() {
        let s = Shape::sphere();
        let a = Intersection::new(-1.0, s.clone());
        let b = Intersection::new(-2.0, s);
        let i = Intersections::new(vec![a, b]);

        assert_eq!(None, i.hit());
    }

    #[test]
    fn hit_order() {
        let s = Shape::sphere();
        let a = Intersection::new(5.0, s.clone());
        let b = Intersection::new(7.0, s.clone());
        let c = Intersection::new(-3.0, s.clone());
        let d = Intersection::new(2.0, s);
        let i = Intersections::new(vec![a, b, c, d.clone()]);

        assert_eq!(d, i.hit().unwrap());
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere();
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.obj.get_id(), i.obj.get_id());
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn precomputing_state_of_intersection_outside() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere();
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn precomputing_state_of_intersection_inside() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere();
        let i = Intersection::new(1.0, shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.inside, true);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_should_offset_the_point() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = Shape::sphere();
        shape.transform = translation(0.0, 0.0, 1.0);
        let i = Intersection::new(5.0, shape);
        let comps = i.prepare_computations(&r);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
