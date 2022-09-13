use crate::EPSILON;
use crate::FP;
use std::cmp::PartialEq;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: FP,
    pub y: FP,
    pub z: FP,
    pub w: FP,
}

impl Tuple {
    pub fn new(x: FP, y: FP, z: FP, w: FP) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: FP, y: FP, z: FP) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn vector(x: FP, y: FP, z: FP) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> FP {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();

        Self::new(self.x / mag, self.y / mag, self.z / mag, self.w / mag)
    }

    pub fn dot(self, rhs: &Tuple) -> FP {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn get(&self, i: usize) -> FP {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("Can't fetch that value from a tuple"),
        }
    }

    pub fn set(&mut self, i: usize, v: FP) {
        match i {
            0 => self.x = v,
            1 => self.y = v,
            2 => self.z = v,
            3 => self.w = v,
            _ => panic!("Can't fetch that value from a tuple"),
        }
    }

    pub fn cross(&self, rhs: &Tuple) -> Self {
        Self::vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn reflect(self, n: &Tuple) -> Self {
        self - (*n * 2.0) * self.dot(n)
    }
}

impl PartialEq<Tuple> for Tuple {
    fn eq(&self, rhs: &Tuple) -> bool {
        (self.x - rhs.x).abs() < EPSILON
            && (self.y - rhs.y).abs() < EPSILON
            && (self.z - rhs.z).abs() < EPSILON
            && (self.w - rhs.w).abs() < EPSILON
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl ops::Mul<FP> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: FP) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl ops::Div<FP> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: FP) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

pub fn point(x: FP, y: FP, z: FP) -> Tuple {
    Tuple::point(x, y, z)
}

pub fn vector(x: FP, y: FP, z: FP) -> Tuple {
    Tuple::vector(x, y, z)
}

#[cfg(test)]
mod tests {
    use crate::tuple::*;
    use crate::FP;

    #[test]
    fn new_tuple_point() {
        let point = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(4.3, point.x);
        assert_eq!(-4.2, point.y);
        assert_eq!(3.1, point.z);
        assert_eq!(1.0, point.w);

        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    #[test]
    fn new_tuple_vector() {
        let vector = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(4.3, vector.x);
        assert_eq!(-4.2, vector.y);
        assert_eq!(3.1, vector.z);
        assert_eq!(0.0, vector.w);

        assert!(!vector.is_point());
        assert!(vector.is_vector());
    }

    #[test]
    fn new_point() {
        assert_eq!(point(4.0, -4.0, 3.0), Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn new_vector() {
        assert_eq!(vector(4.0, -4.0, 3.0), Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn add_vector_to_point() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtract_point_from_point() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vector_from_vector() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_zero_from_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);

        assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_tuple() {
        assert_eq!(-vector(-1.0, 2.0, 3.0), vector(1.0, -2.0, -3.0));
    }

    #[test]
    fn multiply_by_scalar() {
        assert_eq!(
            Tuple::new(1.0, -2.0, 3.0, -4.0) * 3.5,
            Tuple::new(3.5, -7.0, 10.5, -14.0)
        );
    }

    #[test]
    fn multiply_by_fraction() {
        assert_eq!(
            Tuple::new(1.0, -2.0, 3.0, -4.0) * 0.5,
            Tuple::new(0.5, -1.0, 1.5, -2.0)
        );
    }

    #[test]
    fn divide_by_scalar() {
        assert_eq!(
            Tuple::new(1.0, -2.0, 3.0, -4.0) / 2.0,
            Tuple::new(0.5, -1.0, 1.5, -2.0)
        );
    }

    #[test]
    fn magnitude_of_vector() {
        assert_eq!(vector(1.0, 0.0, 0.0).magnitude(), 1.0);

        assert_eq!(vector(0.0, 1.0, 0.0).magnitude(), 1.0);

        assert_eq!(vector(0.0, 0.0, 1.0).magnitude(), 1.0);

        assert_eq!(vector(1.0, 2.0, 3.0).magnitude(), (14.0 as FP).sqrt());

        assert_eq!(vector(-1.0, -2.0, -3.0).magnitude(), (14.0 as FP).sqrt());
    }

    #[test]
    fn normalizing_vector() {
        assert_eq!(vector(4.0, 0.0, 0.0).normalize(), vector(1.0, 0.0, 0.0));

        assert_eq!(
            vector(1.0, 2.0, 3.0).normalize(),
            vector(0.26726124, 0.5345225, 0.8017837)
        );
    }

    #[test]
    fn magnitude_of_normal_vector() {
        assert!((vector(1.0, 2.0, 3.0).normalize().magnitude() - 1.0).abs() < FP::EPSILON);
    }

    #[test]
    fn tuple_dot_product() {
        assert_eq!(vector(1.0, 2.0, 3.0).dot(&vector(2.0, 3.0, 4.0)), 20.0);
    }

    #[test]
    fn vector_cross_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);

        assert_eq!(a.cross(&b), vector(-1.0, 2.0, -1.0));

        assert_eq!(b.cross(&a), vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_vector_at_45() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);

        assert_eq!(v.reflect(&n), vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_vector_from_slant() {
        let v = vector(0.0, -1.0, 0.0);
        let a = (2.0 as FP).sqrt() / 2.0;
        let n = vector(a, a, 0.0);

        assert_eq!(v.reflect(&n), vector(1.0, 0.0, 0.0));
    }
}
