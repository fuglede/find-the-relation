use std::ops::Mul;

use num::Complex;

pub fn evaluate_polynomial(summands: &[(i32, i32)], q: &Complex<f64>)  -> Complex<f64> {
    summands.iter().map(|(exp, coef)| Complex::new(*coef as f64, 0.0) * q.powi(*exp)).sum()
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
                res.d[i][j] = (0..3).map(|k| self.d[i][k] * rhs.d[k][j]).sum();
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

    pub fn distance_from_identity(&self) -> f64 {
        let mut norm_square = 0.0;
        for i in 0..3 {
            for j in 0..3 {
                let target = Complex::new(if j == i { 1.0 } else { 0.0 }, 0.0);
                norm_square += (self.d[i][j] - target).norm_sqr()
            }
        }
        norm_square.sqrt()
    }

    pub fn flatten(&self) -> [Complex<f64>; 9] {
        let mut res: [Complex<f64>; 9] = [Complex::new(0.0, 0.0); 9];
        for i in 0..3 {
            for j in 0..3 {
                let index = 3*i + j;
                res[index] = self.d[i][j];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::{One, Zero};

    #[test]
    fn evaluate_polynomial_trivial_polynomial() {
        assert_eq!(Complex::zero(), evaluate_polynomial(&vec![], &Complex::new(1.0, 0.0)));
        assert_eq!(Complex::zero(), evaluate_polynomial(&vec![], &Complex::new(0.0, 1.0)));
        assert_eq!(Complex::zero(), evaluate_polynomial(&vec![], &Complex::new(1.0, 1.0)));
    }

    #[test]
    fn evaluate_polynomial_constant_polynomial() {
        assert_eq!(Complex::one(), evaluate_polynomial(&vec![(0, 1)], &Complex::new(1.0, 0.0)));
        assert_eq!(Complex::one(), evaluate_polynomial(&vec![(0, 1)], &Complex::new(0.0, 1.0)));
        assert_eq!(Complex::one(), evaluate_polynomial(&vec![(0, 1)], &Complex::new(1.0, 1.0)));
    }

    #[test]
    fn evaluate_polynomial_linear_polynomial() {
        assert_eq!(Complex::new(2.0, 0.0), evaluate_polynomial(&vec![(1, 2)], &Complex::new(1.0, 0.0)));
        assert_eq!(Complex::new(0.0, 2.0), evaluate_polynomial(&vec![(1, 2)], &Complex::new(0.0, 1.0)));
        assert_eq!(Complex::new(2.0, 2.0), evaluate_polynomial(&vec![(1, 2)], &Complex::new(1.0, 1.0)));
    }

    #[test]
    fn evaluate_polynomial_non_trivial_polynomial() {
        assert_eq!(Complex::new(-4.0, 0.0), evaluate_polynomial(&vec![(-2, -2), (2, -2)], &Complex::new(1.0, 0.0)));
        assert_eq!(Complex::new(4.0, 0.0), evaluate_polynomial(&vec![(-2, -2), (2, -2)], &Complex::new(0.0, 1.0)));
        assert_eq!(Complex::new(0.0, -3.0), evaluate_polynomial(&vec![(-2, -2), (2, -2)], &Complex::new(1.0, 1.0)));
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
        let q = Complex::new(60.0, 42.0);
        let a = Matrix::identity();
        let pol = evaluate_polynomial(&vec![(-2, 2), (-2, 2)], &q);
        let pol2 = evaluate_polynomial(&vec![(-2, 2), (-2, 2)], &q);
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
        let pol = evaluate_polynomial(&vec![(-2, 2), (-2, 2)], &q);
        let pol2 = evaluate_polynomial(&vec![(-2, 2), (-2, 2)], &q);
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
        mat1.d[0][0] = evaluate_polynomial(&vec![(1, 1)], &q);
        mat1.d[0][1] = evaluate_polynomial(&vec![(0, 1), (1, 1)], &q);
        mat1.d[1][0] = evaluate_polynomial(&vec![(-1, 1)], &q);
        mat1.d[1][1] = evaluate_polynomial(&vec![(2, 2)], &q);

        // Right:
        let mut mat2 = Matrix::identity();
        mat2.d[0][0] = evaluate_polynomial(&vec![(1, -1)], &q);
        mat2.d[1][0] = evaluate_polynomial(&vec![(2, 1), (3, 1)], &q);
        mat2.d[1][1] = evaluate_polynomial(&vec![(0, 5)], &q);

        // Expected product:
        let mut mat3 = Matrix::identity();
        mat3.d[0][0] = evaluate_polynomial(&vec![(3, 2), (4, 1)], &q);
        mat3.d[0][1] = evaluate_polynomial(&vec![(0, 5), (1, 5)], &q);
        mat3.d[1][0] = evaluate_polynomial(&vec![(0, -1), (4, 2), (5, 2)], &q);
        mat3.d[1][1] = evaluate_polynomial(&vec![(2, 10)], &q);

        let actual = &mat1 * &mat2;
        assert_eq!(actual, mat3);
    }
}

