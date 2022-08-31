use std::fmt::Debug;
use std::ops;

use crate::Tuple;

type Matrix2 = Matrix<2, [f32; 4]>;
type Matrix3 = Matrix<3, [f32; 9]>;
type Matrix4 = Matrix<4, [f32; 16]>;

pub trait DataStorage:
    ops::Index<usize, Output = f32> + ops::IndexMut<usize> + Copy + Clone + Debug
{
}
impl DataStorage for [f32; 4] {}
impl DataStorage for [f32; 9] {}
impl DataStorage for [f32; 16] {}

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const S: usize, D: DataStorage>
where
    D: Clone,
{
    data: D,
}

impl<const S: usize, D: DataStorage> Matrix<{ S }, D> {
    pub fn new(data: D) -> Self {
        Self { data }
    }

    #[inline]
    pub fn get(&self, r: usize, c: usize) -> f32 {
        self.data[r * S + c]
    }

    #[inline]
    pub fn set(&mut self, r: usize, c: usize, v: f32) {
        self.data[r * S + c] = v
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

impl Matrix4 {
    fn identity() -> Self {
        Self::new([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
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
}

impl Matrix3 {
    fn identity() -> Self {
        Self::new([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
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

    fn minor(&self, r: usize, c: usize) -> f32 {
        self.submatrix(r, c).determinant()
    }
}

impl Matrix2 {
    fn identity() -> Self {
        Self::new([1.0, 0.0, 0.0, 1.0])
    }

    fn determinant(&self) -> f32 {
        self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
    }
}

impl<const S: usize, D: DataStorage> PartialEq<Matrix<S, D>> for Matrix<S, D> {
    fn eq(&self, rhs: &Matrix<S, D>) -> bool {
        (0..S * S).all(|i| (self.data[i] - rhs.data[i]).abs() < f32::EPSILON)
    }
}

impl<const S: usize, D: DataStorage> ops::Mul<Matrix<S, D>> for Matrix<S, D> {
    type Output = Matrix<S, D>;

    fn mul(self, rhs: Matrix<S, D>) -> Matrix<S, D> {
        let mut out = self.clone();
        for r in 0..S {
            for c in 0..S {
                out.set(r, c, (0..S).map(|i| self.get(r, i) * rhs.get(i, c)).sum());
            }
        }
        out
    }
}

impl<const S: usize, D: DataStorage> ops::Mul<Tuple> for Matrix<S, D> {
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
    use crate::matrix::Matrix2;
    use crate::matrix::Matrix3;
    use crate::matrix::Matrix4;
    use crate::Tuple;

    #[test]
    fn basic_4x4() {
        let m = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
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
        let m = Matrix3::new([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(-3.0, m.get(0, 0));
        assert_eq!(-2.0, m.get(1, 1));
        assert_eq!(1.0, m.get(2, 2));
    }

    #[test]
    fn basic_2x2() {
        let m = Matrix2::new([-3.0, 5.0, 1.0, -2.0]);

        assert_eq!(-3.0, m.get(0, 0));
        assert_eq!(5.0, m.get(0, 1));
        assert_eq!(1.0, m.get(1, 0));
        assert_eq!(-2.0, m.get(1, 1));
    }

    #[test]
    fn matrix_equality() {
        let m1 = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        let m2 = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_ne() {
        let m1 = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        let m2 = Matrix4::new([
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ]);

        assert!(m1 != m2);
    }

    #[test]
    fn matrix_multiply() {
        let m1 = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        let m2 = Matrix4::new([
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ]);

        assert_eq!(
            Matrix4::new([
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0
            ]),
            m1 * m2
        );
    }

    #[test]
    fn matrix_multiply_with_tuple() {
        let m = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ]);

        let t = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(Tuple::new(18.0, 24.0, 33.0, 1.0), m * t);
    }

    #[test]
    fn matrix_multiply_with_identity() {
        let m = Matrix4::new([
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        ]);

        assert_eq!(m, m * Matrix4::identity());
    }

    #[test]
    fn matrix_transpose() {
        let m = Matrix4::new([
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ]);

        assert_eq!(
            Matrix4::new([
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
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
        assert_eq!(17.0, Matrix2::new([1.0, 5.0, -3.0, 2.0]).determinant());
    }

    #[test]
    fn submatrix4() {
        let m = Matrix4::new([
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        ]);

        assert_eq!(
            Matrix3::new([-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]),
            m.submatrix(2, 1)
        );
    }

    #[test]
    fn submatrix3() {
        let m = Matrix3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);

        assert_eq!(Matrix2::new([-3.0, 2.0, 0.0, 6.0]), m.submatrix(0, 2));
    }

    #[test]
    fn minor_matrix3() {
        let a = Matrix3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        let b = a.submatrix(1, 0);

        assert_eq!(25.0, b.determinant());

        assert_eq!(25.0, a.minor(1, 0));
    }
}
