use std::ops::Index;
use std::fmt::Debug;


type Matrix2 = Matrix<2, [f32; 4]>;
type Matrix3 = Matrix<3, [f32; 9]>;
type Matrix4 = Matrix<4, [f32; 16]>;

#[derive(PartialEq, Debug)]
pub struct Matrix<const S: usize, D: Index<usize, Output = f32>> {
    data: D,
}

impl<const S: usize, D: Index<usize, Output = f32> + Debug + Copy + Clone + PartialEq + PartialOrd> Matrix<{ S }, D> {
    pub fn new(data: D) -> Self {
        Self {
            data,
        }
    }

    pub fn get(&self, r: usize, c: usize) -> f32 {
        self.data[r * S + c]
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix2;
    use crate::matrix::Matrix3;
    use crate::matrix::Matrix4;

    #[test]
    fn basic_4x4() {
        let m = Matrix4::new([1.0, 2.0, 3.0, 4.0,
                     5.5, 6.5, 7.5, 8.5,
                     9.0, 10.0, 11.0, 12.0,
                     13.5, 14.5, 15.5, 16.5]);

        assert_eq!(1.0, m.get(0,0));
        assert_eq!(4.0, m.get(0,3));
        assert_eq!(5.5, m.get(1,0));
        assert_eq!(7.5, m.get(1,2));
        assert_eq!(11.0, m.get(2,2));
        assert_eq!(13.5, m.get(3,0));
        assert_eq!(15.5, m.get(3,2));
    }

    #[test]
    fn basic_3x3() {
        let m = Matrix3::new([-3.0, 5.0, 0.0,
                              1.0, -2.0, -7.0,
                              0.0, 1.0, 1.0]);

        assert_eq!(-3.0, m.get(0,0));
        assert_eq!(-2.0, m.get(1,1));
        assert_eq!(1.0, m.get(2,2));
    }

    #[test]
    fn basic_2x2() {
        let m = Matrix2::new([-3.0, 5.0,
                              1.0, -2.0]);

        assert_eq!(-3.0, m.get(0,0));
        assert_eq!(5.0, m.get(0,1));
        assert_eq!(1.0, m.get(1,0));
        assert_eq!(-2.0, m.get(1,1));
    }

    #[test]
    fn matrix_equality() {
        let m1 = Matrix4::new([1.0, 2.0, 3.0, 4.0,
                     5.0, 6.0, 7.0, 8.0,
                     9.0, 8.0, 7.0, 6.0,
                     5.0, 4.0, 3.0, 2.0]);

        let m2 = Matrix4::new([1.0, 2.0, 3.0, 4.0,
                                5.0, 6.0, 7.0, 8.0,
                                9.0, 8.0, 7.0, 6.0,
                                5.0, 4.0, 3.0, 2.0]);

        assert_eq!(m1, m2);
        
    }

    #[test]
    fn matrix_equality_ne() {
        let m1 = Matrix4::new([1.0, 2.0, 3.0, 4.0,
                                5.0, 6.0, 7.0, 8.0,
                                9.0, 8.0, 7.0, 6.0,
                                5.0, 4.0, 3.0, 2.0]);

        let m2 = Matrix4::new([2.0, 3.0, 4.0, 5.0,
                               6.0, 7.0, 8.0, 9.0,
                               8.0, 7.0, 6.0, 5.0,
                               4.0, 3.0, 2.0, 1.0]);

        assert!(m1 != m2);
    }
}