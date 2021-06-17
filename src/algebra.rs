//use hashbrown::HashMap;
use std::collections::HashMap;
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Polynomial {
    pub data: HashMap<i32, i32>
}

impl Add for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let mut c = HashMap::new();
        for (a_pow, a_coef) in self.data.iter() {
            *c.entry(*a_pow).or_insert(0) += *a_coef;
        }
        for (b_pow, b_coef) in rhs.data.iter() {
            *c.entry(*b_pow).or_insert(0) += *b_coef;
        }
        c.retain(|_, v| *v != 0);
        Polynomial { data: c }
    }
}

impl Mul for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        let mut c = HashMap::new();
        for (a_pow, a_coef) in self.data.iter() {
            for (b_pow, b_coef) in rhs.data.iter() {
                *c.entry(a_pow + b_pow).or_insert(0) += a_coef * b_coef;
            }
        }
        c.retain(|_, v| *v != 0);
        Polynomial { data: c }
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, rhs: &Polynomial) -> bool {
        self.data == rhs.data
    }
}
impl Polynomial {
    pub fn zero() -> Polynomial {
        let data = HashMap::new();
        Polynomial { data }
    }

    pub fn one() -> Polynomial {
        let mut data = HashMap::new();
        data.insert(0, 1);
        Polynomial { data }
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
        let mut a = HashMap::new();
        a.insert(1, 1);
        a.insert(2, 2);
        let mut b = HashMap::new();
        b.insert(0, -1);
        b.insert(1, 1);
        let c = (&(Polynomial { data: a }) + &(Polynomial { data: b })).data;
        assert_eq!(c.values().count(), 3);
        assert_eq!(c[&0], -1);
        assert_eq!(c[&1], 2);
        assert_eq!(c[&2], 2);
    }

    #[test]
    fn add_polynomial_terms_cancel() {
        let mut a = HashMap::new();
        a.insert(1, 1);
        a.insert(2, 2);
        let mut b = HashMap::new();
        b.insert(0, -1);
        b.insert(1, -1);
        let c = (&(Polynomial { data: a }) + &(Polynomial { data: b })).data;
        assert_eq!(c.values().count(), 2);
        assert_eq!(c[&0], -1);
        assert_eq!(c[&2], 2);
    }

    #[test]
    fn multiply_polynomial_negative_coefficient() {
        let mut a = HashMap::new();
        a.insert(1, 1);
        a.insert(2, 2);
        let mut b = HashMap::new();
        b.insert(0, -1);
        let c = (&(Polynomial { data: a }) * &(Polynomial { data: b })).data;
        // (t + 2t^2) * (-1) = -t - 2t^2
        assert_eq!(c.values().count(), 2);
        assert_eq!(c[&1], -1);
        assert_eq!(c[&2], -2);
    }

    #[test]
    fn multiply_polynomial_terms_cancel() {
        let mut a = HashMap::new();
        a.insert(-2, 2);
        a.insert(2, 2);
        let mut b = HashMap::new();
        b.insert(-2, 2);
        b.insert(2, 2);
        let c = (&(Polynomial { data: a }) * &(Polynomial { data: b })).data;
        // (2t^{-2} + 2t^2) * (2t^{-2} + 2t^2) = 4t^{-4} + 8 + 4t^4
        assert_eq!(c.values().count(), 3);
        assert_eq!(c[&-4], 4);
        assert_eq!(c[&0], 8);
        assert_eq!(c[&-4], 4);
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
        let mut map = HashMap::new();
        map.insert(-2, 2);
        map.insert(2, 2);
        let map2 = map.clone();
        let pol = Polynomial { data: map };
        let pol2 = Polynomial { data: map2 };
        let mut b = Matrix::zero();
        b.d[0][1] = Polynomial::one();
        b.d[0][2] = pol;
        b.d[2][1] = pol2;
        let c = &a * &b;
        assert_eq!(c, b);
    }

    #[test]
    fn multiply_matrix_left_side_zero() {
        let a = Matrix::zero();
        let mut map = HashMap::new();
        map.insert(-2, 2);
        map.insert(2, 2);
        let map2 = map.clone();
        let pol = Polynomial { data: map };
        let pol2 = Polynomial { data: map2 };
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
        let mut map1r1c1  = HashMap::new();
        map1r1c1.insert(1, 1);
        let mut map1r1c2  = HashMap::new();
        map1r1c2.insert(0, 1);
        map1r1c2.insert(1, 1);
        let mut map1r2c1  = HashMap::new();
        map1r2c1.insert(-1, 1);
        let mut map1r2c2  = HashMap::new();
        map1r2c2.insert(2, 2);
        let mut mat1 = Matrix::identity();
        mat1.d[0][0] = Polynomial { data: map1r1c1 };
        mat1.d[0][1] = Polynomial { data: map1r1c2 };
        mat1.d[1][0] = Polynomial { data: map1r2c1 };
        mat1.d[1][1] = Polynomial { data: map1r2c2 };

        // Right:
        let mut map2r1c1  = HashMap::new();
        map2r1c1.insert(1, -1);
        let mut map2r2c1  = HashMap::new();
        map2r2c1.insert(2, 1);
        map2r2c1.insert(3, 1);
        let mut map2r2c2  = HashMap::new();
        map2r2c2.insert(0, 5);
        let mut mat2 = Matrix::identity();
        mat2.d[0][0] = Polynomial { data: map2r1c1 };
        mat2.d[1][0] = Polynomial { data: map2r2c1 };
        mat2.d[1][1] = Polynomial { data: map2r2c2 };

        // Expected product:
        let mut map3r1c1  = HashMap::new();
        map3r1c1.insert(3, 2);
        map3r1c1.insert(4, 1);
        let mut map3r1c2  = HashMap::new();
        map3r1c2.insert(0, 5);
        map3r1c2.insert(1, 5);
        let mut map3r2c1  = HashMap::new();
        map3r2c1.insert(0, -1);
        map3r2c1.insert(4, 2);
        map3r2c1.insert(5, 2);
        let mut map3r2c2  = HashMap::new();
        map3r2c2.insert(2, 10);
        let mut mat3 = Matrix::identity();
        mat3.d[0][0] = Polynomial { data: map3r1c1 };
        mat3.d[0][1] = Polynomial { data: map3r1c2 };
        mat3.d[1][0] = Polynomial { data: map3r2c1 };
        mat3.d[1][1] = Polynomial { data: map3r2c2 };

        let actual = &mat1 * &mat2;
        assert_eq!(actual, mat3);
    }
}

