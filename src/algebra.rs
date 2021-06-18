//use hashbrown::HashMap;
use std::collections::HashMap;
use std::ops::{Mul};

use num::{BigInt, Complex, ToPrimitive};

#[derive(Debug)]
pub struct Polynomial {
    pub data: HashMap<i32, BigInt>
}

impl Polynomial {
    pub fn new(elements: Vec<(i32, i32)>) -> Polynomial {
        let mut data = HashMap::new();
        for (a, b) in elements {
            data.insert(a, BigInt::from(b));
        }
        Polynomial { data }
    }

    pub fn one() -> Polynomial {
        Self::new(vec![(0, 1)])
    }

    pub fn evaluate(&self, q: &Complex<f64>) -> Complex<f64> {
        let mut res = Complex::new(0.0, 0.0);
        for (pow, coef) in self.data.iter() {
            res += (*coef).to_f64().unwrap() * q.powi(*pow)
        }
        res
    }
}

#[derive(Debug)]
pub struct Matrix {
    pub d: [[Complex<f64>; 3]; 3]
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Matrix {
        let mut res = Matrix::zero();
        for i in 0..3 {
            for j in 0..3 {
                res.d[i][j] = (0..3).map(|k| &self.d[i][k] * &rhs.d[k][j]).fold(Complex::new(0.0, 0.0), |sum, val| &sum + &val);
            }
        }
        res
    }
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Matrix) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if (self.d[i][j] - rhs.d[i][j]).norm() > 1e-10 {
                    return false
                }
            }
        }
        true
    }
}

impl Matrix {
    pub fn zero() -> Matrix {
        let d: [[Complex<f64>; 3]; 3] = [[Complex::new(0.0, 0.0); 3]; 3];
        Matrix { d }
    }

    pub fn identity() -> Matrix {
        let mut res = Self::zero();
        for i in 0..3 {
            res.d[i][i] = Complex::new(1.0, 0.0);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::{One, Zero};

    #[test]
    fn multiply_matrix_two_identities() {
        let a = Matrix::identity();
        let b = Matrix::identity();
        let c = &a * &b;
        assert_eq!(c, Matrix::identity());
    }

    #[test]
    fn multiply_matrix_left_side_identity() {
        let q = Complex::new(60.0, 42.0);
        let a = Matrix::identity();
        let pol = Polynomial::new(vec![(-2, 2), (-2, 2)]).evaluate(&q);
        let pol2 = Polynomial::new(vec![(-2, 2), (-2, 2)]).evaluate(&q);
        let mut b = Matrix::zero();
        b.d[0][1] = Complex::one();
        b.d[0][2] = pol;
        b.d[2][1] = pol2;
        let c = &a * &b;
        assert_eq!(c, b);
    }

    #[test]
    fn multiply_matrix_left_side_zero() {
        let q = Complex::new(60.0, 42.0);
        let pol = Polynomial::new(vec![(-2, 2), (-2, 2)]).evaluate(&q);
        let pol2 = Polynomial::new(vec![(-2, 2), (-2, 2)]).evaluate(&q);
        let a = Matrix::zero();
        let mut b = Matrix::zero();
        b.d[0][1] = Complex::one();
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
        let q = Complex::new(60.0, 42.0);
        let mut mat1 = Matrix::identity();
        mat1.d[0][0] = Polynomial::new(vec![(1, 1)]).evaluate(&q);
        mat1.d[0][1] = Polynomial::new(vec![(0, 1), (1, 1)]).evaluate(&q);
        mat1.d[1][0] = Polynomial::new(vec![(-1, 1)]).evaluate(&q);
        mat1.d[1][1] = Polynomial::new(vec![(2, 2)]).evaluate(&q);

        // Right:
        let mut mat2 = Matrix::identity();
        mat2.d[0][0] = Polynomial::new(vec![(1, -1)]).evaluate(&q);
        mat2.d[1][0] = Polynomial::new(vec![(2, 1), (3, 1)]).evaluate(&q);
        mat2.d[1][1] = Polynomial::new(vec![(0, 5)]).evaluate(&q);

        // Expected product:
        let mut mat3 = Matrix::identity();
        mat3.d[0][0] = Polynomial::new(vec![(3, 2), (4, 1)]).evaluate(&q);
        mat3.d[0][1] = Polynomial::new(vec![(0, 5), (1, 5)]).evaluate(&q);
        mat3.d[1][0] = Polynomial::new(vec![(0, -1), (4, 2), (5, 2)]).evaluate(&q);
        mat3.d[1][1] = Polynomial::new(vec![(2, 10)]).evaluate(&q);

        let actual = &mat1 * &mat2;
        assert_eq!(actual, mat3);
    }

    #[test]
    fn evaluate_polynomial_trivial_polynomial() {
        let p = Polynomial::new(vec![]);
        assert_eq!(Complex::zero(), p.evaluate(&Complex::new(1.0, 0.0)));
        assert_eq!(Complex::zero(), p.evaluate(&Complex::new(0.0, 1.0)));
        assert_eq!(Complex::zero(), p.evaluate(&Complex::new(1.0, 1.0)));
    }

    #[test]
    fn evaluate_polynomial_constant_polynomial() {
        let p = Polynomial::one();
        assert_eq!(Complex::one(), p.evaluate(&Complex::new(1.0, 0.0)));
        assert_eq!(Complex::one(), p.evaluate(&Complex::new(0.0, 1.0)));
        assert_eq!(Complex::one(), p.evaluate(&Complex::new(1.0, 1.0)));
    }

    #[test]
    fn evaluate_polynomial_linear_polynomial() {
        let p = Polynomial::new(vec![(1, 2)]);
        assert_eq!(Complex::new(2.0, 0.0), p.evaluate(&Complex::new(1.0, 0.0)));
        assert_eq!(Complex::new(0.0, 2.0), p.evaluate(&Complex::new(0.0, 1.0)));
        assert_eq!(Complex::new(2.0, 2.0), p.evaluate(&Complex::new(1.0, 1.0)));
    }

    #[test]
    fn evaluate_polynomial_non_trivial_polynomial() {
        let p = Polynomial::new(vec![(-2, -2), (2, -2)]);
        assert_eq!(Complex::new(-4.0, 0.0), p.evaluate(&Complex::new(1.0, 0.0)));
        assert_eq!(Complex::new(4.0, 0.0), p.evaluate(&Complex::new(0.0, 1.0)));
        assert_eq!(Complex::new(0.0, -3.0), p.evaluate(&Complex::new(1.0, 1.0)));
    }
}

