//use hashbrown::HashMap;
use std::collections::HashMap;
use std::ops::{Add, Mul};

use num::BigInt;

#[derive(Debug)]
pub struct Polynomial {
    pub data: HashMap<i32, BigInt>
}

impl Add for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let mut c = HashMap::new();
        for (a_pow, a_coef) in self.data.iter() {
            *c.entry(*a_pow).or_insert(BigInt::from(0)) += a_coef;
        }
        for (b_pow, b_coef) in rhs.data.iter() {
            *c.entry(*b_pow).or_insert(BigInt::from(0)) += b_coef;
        }
        c.retain(|_, v| *v != BigInt::from(0));
        Polynomial { data: c }
    }
}

impl Mul for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        let mut c = HashMap::new();
        for (a_pow, a_coef) in self.data.iter() {
            for (b_pow, b_coef) in rhs.data.iter() {
                *c.entry(a_pow + b_pow).or_insert(BigInt::from(0)) +=
                    a_coef * b_coef;
            }
        }
        c.retain(|_, v| *v != BigInt::from(0));
        Polynomial { data: c }
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, rhs: &Polynomial) -> bool {
        self.data == rhs.data
    }
}

impl Polynomial {
    pub fn new(elements: Vec<(i32, i32)>) -> Polynomial {
        let mut data = HashMap::new();
        for (a, b) in elements {
            data.insert(a, BigInt::from(b));
        }
        Polynomial { data }
    }

    pub fn zero() -> Polynomial {
        Self::new(vec![])
    }

    pub fn one() -> Polynomial {
        Self::new(vec![(0, 1)])
    }
}

#[derive(Debug)]
pub struct Matrix {
    pub d: [[Polynomial; 3]; 3]
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Matrix {
        let mut res = Matrix::zero();
        for i in 0..3 {
            for j in 0..3 {
                res.d[i][j] = (0..3).map(|k| &self.d[i][k] * &rhs.d[k][j]).fold(Polynomial::zero(), |sum, val| &sum + &val);
            }
        }
        res
    }
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Matrix) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.d[i][j] != rhs.d[i][j] {
                    return false
                }
            }
        }
        true
    }
}

impl Matrix {
    pub fn zero() -> Matrix {
        let d: [[Polynomial; 3]; 3] = 
            [[Polynomial::zero(), Polynomial::zero(), Polynomial::zero()],
             [Polynomial::zero(), Polynomial::zero(), Polynomial::zero()],
             [Polynomial::zero(), Polynomial::zero(), Polynomial::zero()]];
        Matrix { d }
    }

    pub fn identity() -> Matrix {
        let mut res = Self::zero();
        for i in 0..3 {
            res.d[i][i] = Polynomial::one();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_polynomial_overlapping_coefficient() {
        let a = Polynomial::new(vec![(1, 1), (2, 2)]);
        let b = Polynomial::new(vec![(0, -1), (1, 1)]);
        let c = (&a + &b).data;
        assert_eq!(c.values().count(), 3);
        assert_eq!(c[&0], BigInt::from(-1));
        assert_eq!(c[&1], BigInt::from(2));
        assert_eq!(c[&2], BigInt::from(2));
    }

    #[test]
    fn add_polynomial_terms_cancel() {
        let a = Polynomial::new(vec![(1, 1), (2, 2)]);
        let b = Polynomial::new(vec![(0, -1), (1, -1)]);
        let c = (&a + &b).data;
        assert_eq!(c.values().count(), 2);
        assert_eq!(c[&0], BigInt::from(-1));
        assert_eq!(c[&2], BigInt::from(2));
    }

    #[test]
    fn multiply_polynomial_negative_coefficient() {
        let a = Polynomial::new(vec![(1, 1), (2, 2)]);
        let b = Polynomial::new(vec![(0, -1)]);
        let c = (&a * &b).data;
        // (t + 2t^2) * (-1) = -t - 2t^2
        assert_eq!(c.values().count(), 2);
        assert_eq!(c[&1], BigInt::from(-1));
        assert_eq!(c[&2], BigInt::from(-2));
    }

    #[test]
    fn multiply_polynomial_terms_cancel() {
        let a = Polynomial::new(vec![(-2, 2), (2, 2)]);
        let b = Polynomial::new(vec![(-2, 2), (2, 2)]);
        let c = (&a * &b).data;
        // (2t^{-2} + 2t^2) * (2t^{-2} + 2t^2) = 4t^{-4} + 8 + 4t^4
        assert_eq!(c.values().count(), 3);
        assert_eq!(c[&-4], BigInt::from(4));
        assert_eq!(c[&0], BigInt::from(8));
        assert_eq!(c[&-4], BigInt::from(4));
    }

    #[test]
    fn multiply_matrix_two_identities() {
        let a = Matrix::identity();
        let b = Matrix::identity();
        let c = &a * &b;
        assert_eq!(c, Matrix::identity());
    }

    #[test]
    fn multiply_matrix_left_side_identity() {
        let a = Matrix::identity();
        let pol = Polynomial::new(vec![(-2, 2), (-2, 2)]);
        let pol2 = Polynomial::new(vec![(-2, 2), (-2, 2)]);
        let mut b = Matrix::zero();
        b.d[0][1] = Polynomial::one();
        b.d[0][2] = pol;
        b.d[2][1] = pol2;
        let c = &a * &b;
        assert_eq!(c, b);
    }

    #[test]
    fn multiply_matrix_left_side_zero() {
        let pol = Polynomial::new(vec![(-2, 2), (-2, 2)]);
        let pol2 = Polynomial::new(vec![(-2, 2), (-2, 2)]);
        let a = Matrix::zero();
        let mut b = Matrix::zero();
        b.d[0][1] = Polynomial::one();
        b.d[0][2] = pol;
        b.d[2][1] = pol2;
        let c = &a * &b;
        assert_eq!(c, a);
    }

    #[test]
    fn multiply_matrix_non_trivial_example() {
        // Upper left 2x2 block:
        // [ t,       1 + t ] [ -t          0 ]     [ 2t^3 + t^4          5 + 5t ]
        // [ t^{-1},   2t^2 ] [ t^2 + t^3   5 ]  =  [ -1 + 2t^4 + 2t^5     10t^2 ]
        // Left:
        let mut mat1 = Matrix::identity();
        mat1.d[0][0] = Polynomial::new(vec![(1, 1)]);
        mat1.d[0][1] = Polynomial::new(vec![(0, 1), (1, 1)]);
        mat1.d[1][0] = Polynomial::new(vec![(-1, 1)]);
        mat1.d[1][1] = Polynomial::new(vec![(2, 2)]);

        // Right:
        let mut mat2 = Matrix::identity();
        mat2.d[0][0] = Polynomial::new(vec![(1, -1)]);
        mat2.d[1][0] = Polynomial::new(vec![(2, 1), (3, 1)]);
        mat2.d[1][1] = Polynomial::new(vec![(0, 5)]);

        // Expected product:
        let mut mat3 = Matrix::identity();
        mat3.d[0][0] = Polynomial::new(vec![(3, 2), (4, 1)]);
        mat3.d[0][1] = Polynomial::new(vec![(0, 5), (1, 5)]);
        mat3.d[1][0] = Polynomial::new(vec![(0, -1), (4, 2), (5, 2)]);
        mat3.d[1][1] = Polynomial::new(vec![(2, 10)]);

        let actual = &mat1 * &mat2;
        assert_eq!(actual, mat3);
    }
}

