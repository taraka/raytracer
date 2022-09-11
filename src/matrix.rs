use std::fmt::Debug;
use std::ops;

use crate::Tuple;
use crate::FP;
use crate::EPSILON;

pub type Matrix2 = Matrix<2>;
pub type Matrix3 = Matrix<3>;
pub type Matrix4 = Matrix<4>;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const S: usize> {
    data: [[FP; S]; S],
}

impl<const S: usize> Matrix<{ S }> {
    pub fn new(data: [[FP; S]; S]) -> Self {
        Self { data }
    }

    #[inline]
    pub fn get(&self, r: usize, c: usize) -> FP {
        self.data[r][c]
    }

    #[inline]
    pub fn set(&mut self, r: usize, c: usize, v: FP) {
        self.data[r][c] = v
    }

    pub fn transpose(&self) -> Self {
        let mut out = self.clone();

        for r in 0..S {
            for c in 0..S {
                out.set(c, r, self.get(r, c));
            }
        }

        out
    }
}

impl<const S: usize> ops::Index<usize> for Matrix<S> {
    type Output = [FP; S];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const S: usize> ops::IndexMut<usize> for Matrix<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const S: usize> From<[[FP; S]; S]> for Matrix<S> {
    fn from(data: [[FP; S]; S]) -> Self {
        Self { data }
    }
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(x: FP, y: FP, z: FP) -> Self {
        let mut out = Self::identity();

        out.set(0, 3, x);
        out.set(1, 3, y);
        out.set(2, 3, z);

        out
    }

    pub fn scaling(x: FP, y: FP, z: FP) -> Self {
        let mut out = Self::identity();

        out.set(0, 0, x);
        out.set(1, 1, y);
        out.set(2, 2, z);

        out
    }

    pub fn rotation_x(r: FP) -> Self {
        let mut out = Self::identity();

        out.set(1, 1, r.cos());
        out.set(1, 2, -r.sin());
        out.set(2, 1, r.sin());
        out.set(2, 2, r.cos());

        out
    }

    pub fn rotation_y(r: FP) -> Self {
        let mut out = Self::identity();

        out.set(0, 0, r.cos());
        out.set(0, 2, r.sin());
        out.set(2, 0, -r.sin());
        out.set(2, 2, r.cos());

        out
    }

    pub fn rotation_z(r: FP) -> Self {
        let mut out = Self::identity();

        out.set(0, 0, r.cos());
        out.set(0, 1, -r.sin());
        out.set(1, 0, r.sin());
        out.set(1, 1, r.cos());

        out
    }

    pub fn shearing(xy: FP, xz: FP, yx: FP, yz: FP, zx: FP, zy: FP) -> Self {
        let mut out = Self::identity();

        out.set(0, 1, xy);
        out.set(0, 2, xz);
        out.set(1, 0, yx);
        out.set(1, 2, yz);
        out.set(2, 0, zx);
        out.set(2, 1, zy);

        out
    }

    fn submatrix(&self, skip_r: usize, skip_c: usize) -> Matrix3 {
        let mut out = Matrix3::identity();
        for r in 0..4 {
            for c in 0..4 {
                if r == skip_r || c == skip_c {
                    continue;
                }

                let new_r = if r >= skip_r { r - 1 } else { r };
                let new_c = if c >= skip_c { c - 1 } else { c };
                out.set(new_r, new_c, self.get(r, c));
            }
        }
        out
    }

    fn minor(&self, r: usize, c: usize) -> FP {
        self.submatrix(r, c).determinant()
    }

    fn cofactor(&self, r: usize, c: usize) -> FP {
        let minor = self.minor(r, c);

        return if (r + c) % 2 == 0 { minor } else { -minor };
    }

    fn determinant(&self) -> FP {
        (0..4).map(|c| self.get(0, c) * self.cofactor(0, c)).sum()
    }

    fn is_invertable(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix4 {
        if !self.is_invertable() {
            panic!("Matrix is not invertable");
        }

        let d = self.determinant();
        let mut out = Matrix4::identity();

        for r in 0..4 {
            for c in 0..4 {
                out.set(c, r, self.cofactor(r, c) / d);
            }
        }

        out
    }
}

impl Matrix3 {
    fn identity() -> Self {
        Self::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }

    fn submatrix(&self, skip_r: usize, skip_c: usize) -> Matrix2 {
        let mut out = Matrix2::identity();
        for r in 0..3 {
            for c in 0..3 {
                if r == skip_r || c == skip_c {
                    continue;
                }

                let new_r = if r >= skip_r { r - 1 } else { r };
                let new_c = if c >= skip_c { c - 1 } else { c };
                out.set(new_r, new_c, self.get(r, c));
            }
        }
        out
    }

    fn minor(&self, r: usize, c: usize) -> FP {
        self.submatrix(r, c).determinant()
    }

    fn cofactor(&self, r: usize, c: usize) -> FP {
        let minor = self.minor(r, c);

        return if (r + c) % 2 == 0 { minor } else { -minor };
    }

    fn determinant(&self) -> FP {
        (0..3).map(|c| self.get(0, c) * self.cofactor(0, c)).sum()
    }
}

impl Matrix2 {
    fn identity() -> Self {
        Self::new([[1.0, 0.0], [0.0, 1.0]])
    }

    fn determinant(&self) -> FP {
        self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
    }
}

impl<const S: usize> PartialEq<Matrix<S>> for Matrix<S> {
    fn eq(&self, rhs: &Matrix<S>) -> bool {
        (0..S)
            .all(|r| (0..r).all(|c| (self.data[r][c] - rhs.data[r][c]).abs() < EPSILON))
    }
}

impl<const S: usize> ops::Mul<Matrix<S>> for Matrix<S> {
    type Output = Matrix<S>;

    fn mul(self, rhs: Matrix<S>) -> Matrix<S> {
        let mut out = self.clone();
        for r in 0..S {
            for c in 0..S {
                out.set(r, c, (0..S).map(|i| self.get(r, i) * rhs.get(i, c)).sum());
            }
        }
        out
    }
}

impl<const S: usize> ops::Mul<Tuple> for Matrix<S> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        let mut out = rhs.clone();
        for r in 0..S {
            out.set(r, (0..S).map(|i| self.get(r, i) * rhs.get(i)).sum());
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::matrix::Matrix2;
    use crate::matrix::Matrix3;
    use crate::matrix::Matrix4;
    use crate::Tuple;

    #[test]
    fn basic_4x4() {
        let m = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(1.0, m.get(0, 0));
        assert_eq!(4.0, m.get(0, 3));
        assert_eq!(5.5, m.get(1, 0));
        assert_eq!(7.5, m.get(1, 2));
        assert_eq!(11.0, m.get(2, 2));
        assert_eq!(13.5, m.get(3, 0));
        assert_eq!(15.5, m.get(3, 2));
    }

    #[test]
    fn basic_3x3() {
        let m = Matrix3::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_eq!(-3.0, m.get(0, 0));
        assert_eq!(-2.0, m.get(1, 1));
        assert_eq!(1.0, m.get(2, 2));
    }

    #[test]
    fn basic_2x2() {
        let m = Matrix2::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(-3.0, m.get(0, 0));
        assert_eq!(5.0, m.get(0, 1));
        assert_eq!(1.0, m.get(1, 0));
        assert_eq!(-2.0, m.get(1, 1));
    }

    #[test]
    fn matrix_equality() {
        let m1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_ne() {
        let m1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix4::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert!(m1 != m2);
    }

    #[test]
    fn matrix_multiply() {
        let m1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        assert_eq!(
            Matrix4::new([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0]
            ]),
            m1 * m2
        );
    }

    #[test]
    fn matrix_multiply_with_tuple() {
        let m = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let t = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(Tuple::new(18.0, 24.0, 33.0, 1.0), m * t);
    }

    #[test]
    fn matrix_multiply_with_identity() {
        let m = Matrix4::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_eq!(m, m * Matrix4::identity());
    }

    #[test]
    fn matrix_transpose() {
        let m = Matrix4::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        assert_eq!(
            Matrix4::new([
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            ]),
            m.transpose()
        );
    }

    #[test]
    fn matrix_transpose_identity() {
        assert_eq!(Matrix4::identity().transpose(), Matrix4::identity());
        assert_eq!(Matrix3::identity().transpose(), Matrix3::identity());
        assert_eq!(Matrix2::identity().transpose(), Matrix2::identity());
    }

    #[test]
    fn matrix2_determinant() {
        assert_eq!(17.0, Matrix2::new([[1.0, 5.0], [-3.0, 2.0]]).determinant());
    }

    #[test]
    fn submatrix4() {
        let m = Matrix4::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        assert_eq!(
            Matrix3::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]),
            m.submatrix(2, 1)
        );
    }

    #[test]
    fn submatrix3() {
        let m = Matrix3::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        assert_eq!(Matrix2::new([[-3.0, 2.0], [0.0, 6.0]]), m.submatrix(0, 2));
    }

    #[test]
    fn minor_matrix3() {
        let a = Matrix3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0);

        assert_eq!(25.0, b.determinant());
        assert_eq!(25.0, a.minor(1, 0));
    }

    #[test]
    fn matrix3_cofactor() {
        let a = Matrix3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_eq!(-12.0, a.minor(0, 0));
        assert_eq!(-12.0, a.cofactor(0, 0));
        assert_eq!(25.0, a.minor(1, 0));
        assert_eq!(-25.0, a.cofactor(1, 0));
    }

    #[test]
    fn matrix3_determinent() {
        let a = Matrix3::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_eq!(56.0, a.cofactor(0, 0));
        assert_eq!(12.0, a.cofactor(0, 1));
        assert_eq!(-46.0, a.cofactor(0, 2));
        assert_eq!(-196.0, a.determinant());
    }

    #[test]
    fn matrix4_determinent() {
        let a = Matrix4::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_eq!(690.0, a.cofactor(0, 0));
        assert_eq!(447.0, a.cofactor(0, 1));
        assert_eq!(210.0, a.cofactor(0, 2));
        assert_eq!(51.0, a.cofactor(0, 3));
        assert_eq!(-4071.0, a.determinant());
    }

    #[test]
    fn invertable_matrix() {
        let a = Matrix4::new([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_eq!(-2120.0, a.determinant());
        assert!(a.is_invertable());
    }

    #[test]
    fn non_invertable_matrix() {
        let a = Matrix4::new([
            [4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(0.0, a.determinant());
        assert!(!a.is_invertable());
    }

    #[test]
    fn matrix_inverse() {
        let a = Matrix4::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let b = a.inverse();

        assert_eq!(532.0, a.determinant());
        assert_eq!(-160.0, a.cofactor(2, 3));
        assert_eq!(-160.0 / 532.0, b.get(3, 2));
        assert_eq!(105.0, a.cofactor(3, 2));
        assert_eq!(105.0 / 532.0, b.get(2, 3));
        assert_eq!(
            Matrix4::new([
                [0.21805, 0.45112783, 0.24060151, -0.04511278],
                [-0.8082707, -1.456767, -0.44360903, 0.5206767],
                [-0.078947365, -0.2236842, -0.05263158, 0.19736843],
                [-0.52255636, -0.81390977, -0.30075186, 0.30639097]
            ]),
            b
        );
    }

    #[test]
    fn multiple_matrix_by_its_inverse() {
        let a = Matrix4::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let b = Matrix4::new([
            [5.0, -9.0, 7.0, 3.0],
            [4.0, -8.0, 2.0, -9.0],
            [-4.0, -4.0, 4.0, 1.0],
            [-9.0, 5.0, -33.0, 1.0],
        ]);

        let c = a * b;

        assert_eq!(a, c * b.inverse());
    }

    #[test]
    fn multiply_by_translation() {
        let t = Matrix4::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(Tuple::point(2.0, 1.0, 7.0), t * p);
    }

    #[test]
    fn multiply_by_translation_inverse() {
        let t = Matrix4::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(Tuple::point(-8.0, 7.0, 3.0), t.inverse() * p);
    }

    #[test]
    fn translation_does_not_effect_vectors() {
        let t = Matrix4::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(v, t * v);
    }

    #[test]
    fn scaling_point() {
        let s = Matrix4::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(Tuple::point(-8.0, 18.0, 32.0), s * p);
    }

    #[test]
    fn scaling_vector() {
        let s = Matrix4::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(Tuple::vector(-8.0, 18.0, 32.0), s * v);
    }

    #[test]
    fn scaling_vector_inverse() {
        let s = Matrix4::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(Tuple::vector(-2.0, 2.0, 2.0), s.inverse() * v);
    }

    #[test]
    fn reflecting_point() {
        let s = Matrix4::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(Tuple::point(-1.0, 2.0, 3.0), s.inverse() * p);
    }

    #[test]
    fn rotate_x_point() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let r1 = Matrix4::rotation_x(PI / 4.0);
        let r2 = Matrix4::rotation_x(PI / 2.0);

        assert_eq!(Tuple::point(0.0, 0.7071068, 0.7071068), r1 * p);
        assert_eq!(Tuple::point(0.0, 0.0, 1.0), r2 * p);
    }

    #[test]
    fn rotate_y_point() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let r1 = Matrix4::rotation_y(PI / 4.0);
        let r2 = Matrix4::rotation_y(PI / 2.0);

        assert_eq!(Tuple::point(0.7071068, 0.0, 0.7071068), r1 * p);
        assert_eq!(Tuple::point(1.0, 0.0, 0.0), r2 * p);
    }

    #[test]
    fn rotate_z_point() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let r1 = Matrix4::rotation_z(PI / 4.0);
        let r2 = Matrix4::rotation_z(PI / 2.0);

        assert_eq!(Tuple::point(-0.7071068, 0.7071068, 0.0), r1 * p);
        assert_eq!(Tuple::point(-1.0, 0.0, 0.0), r2 * p);
    }

    #[test]
    fn shearing_xy() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let s = Matrix4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(Tuple::point(5.0, 3.0, 4.0), s * p);
    }
    #[test]
    fn shearing_xz() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let s = Matrix4::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(Tuple::point(6.0, 3.0, 4.0), s * p);
    }
    #[test]
    fn shearing_yx() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let s = Matrix4::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(Tuple::point(2.0, 5.0, 4.0), s * p);
    }

    #[test]
    fn shearing_yz() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let s = Matrix4::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(Tuple::point(2.0, 7.0, 4.0), s * p);
    }
    #[test]
    fn shearing_zx() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let s = Matrix4::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(Tuple::point(2.0, 3.0, 6.0), s * p);
    }
    #[test]
    fn shearing_zy() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let s = Matrix4::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(Tuple::point(2.0, 3.0, 7.0), s * p);
    }

    #[test]
    fn chain_transformations_individual() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix4::rotation_x(PI / 2.0);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(Tuple::point(1.0, -1.0, 0.0), p2);

        let p3 = b * p2;
        assert_eq!(Tuple::point(5.0, -5.0, 0.0), p3);

        let p4 = c * p3;
        assert_eq!(Tuple::point(15.0, 0.0, 7.0), p4);
    }

    #[test]
    fn chain_transformations() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix4::rotation_x(PI / 2.0);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);

        assert_eq!(Tuple::point(15.0, 0.0, 7.0), c * b * a * p);
    }
}
