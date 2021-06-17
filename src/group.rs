use crate::algebra::{Polynomial,Matrix};
use wasm_bindgen::prelude::*;
use num::complex::Complex;
use std::collections::HashMap;

#[repr(u8)]
#[wasm_bindgen]
#[derive(PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West
}

pub struct Group {
    north_matrix: Matrix,
    south_matrix: Matrix,
    east_matrix: Matrix,
    west_matrix: Matrix,
    current_matrix: Matrix
}

impl Group {
    pub fn new() -> Self {
        // See https://arxiv.org/abs/1904.11730v3
        let mut north_matrix = Matrix::zero();

        let mut map_north_r1c3 = HashMap::new();
        map_north_r1c3.insert(-1, -1);
        north_matrix.d[0][2] = Polynomial { data: map_north_r1c3 };

        let mut map_north_r2c2 = HashMap::new();
        map_north_r2c2.insert(1, -1);
        north_matrix.d[1][1] = Polynomial { data: map_north_r2c2 };

        let mut map_north_r2c3 = HashMap::new();
        map_north_r2c3.insert(-1, -1);
        map_north_r2c3.insert(1, 1);
        north_matrix.d[1][2] = Polynomial { data: map_north_r2c3 };

        let mut map_north_r3c1 = HashMap::new();
        map_north_r3c1.insert(0, -1);
        north_matrix.d[2][0] = Polynomial { data: map_north_r3c1 };

        let mut map_north_r3c3 = HashMap::new();
        map_north_r3c3.insert(-1, -1);
        map_north_r3c3.insert(0, 1);
        north_matrix.d[2][2] = Polynomial { data: map_north_r3c3 };

        let mut south_matrix = Matrix::zero();

        let mut map_south_r1c1 = HashMap::new();
        map_south_r1c1.insert(0, 1);
        map_south_r1c1.insert(1, -1);
        south_matrix.d[0][0] = Polynomial { data: map_south_r1c1 };
        
        let mut map_south_r1c3 = HashMap::new();
        map_south_r1c3.insert(0, -1);
        south_matrix.d[0][2] = Polynomial { data: map_south_r1c3 };
        
        let mut map_south_r2c1 = HashMap::new();
        map_south_r2c1.insert(-1, 1);
        map_south_r2c1.insert(1, -1);
        south_matrix.d[1][0] = Polynomial { data: map_south_r2c1 };
        
        let mut map_south_r2c2 = HashMap::new();
        map_south_r2c2.insert(-1, -1);
        south_matrix.d[1][1] = Polynomial { data: map_south_r2c2 };
        
        let mut map_south_r3c1 = HashMap::new();
        map_south_r3c1.insert(1, -1);
        south_matrix.d[2][0] = Polynomial { data: map_south_r3c1 };

        let mut east_matrix = Matrix::identity();

        let mut map_east_r1c1 = HashMap::new();
        map_east_r1c1.insert(-1, -1);
        east_matrix.d[0][0] = Polynomial { data: map_east_r1c1 };

        east_matrix.d[0][1] = Polynomial::one();
        east_matrix.d[2][1] = Polynomial::one();

        let mut map_east_r3c3 = HashMap::new();
        map_east_r3c3.insert(1, -1);
        east_matrix.d[2][2] = Polynomial { data: map_east_r3c3 };

        let mut west_matrix = Matrix::identity();

        let mut map_west_r1c1 = HashMap::new();
        map_west_r1c1.insert(1, -1);
        west_matrix.d[0][0] = Polynomial { data: map_west_r1c1 };

        let mut map_west_r1c2 = HashMap::new();
        map_west_r1c2.insert(1, 1);
        west_matrix.d[0][1] = Polynomial { data: map_west_r1c2 };

        let mut map_west_r3c2 = HashMap::new();
        map_west_r3c2.insert(-1, 1);
        west_matrix.d[2][1] = Polynomial { data: map_west_r3c2 };

        let mut map_west_r3c3 = HashMap::new();
        map_west_r3c3.insert(-1, -1);
        west_matrix.d[2][2] = Polynomial { data: map_west_r3c3 };

        let current_matrix = Matrix::identity();
        Self { north_matrix, south_matrix, east_matrix, west_matrix, current_matrix }
    }

    pub fn push(&mut self, direction: &Direction) {
        let matrix = match direction {
            Direction::North => &self.north_matrix,
            Direction::South => &self.south_matrix,
            Direction::East => &self.east_matrix,
            Direction::West => &self.west_matrix
        };
        self.current_matrix = &self.current_matrix * &matrix;
    }

    pub fn evaluate(&self, q: Complex<f64>) -> [f64; 18] {
        let mut res: [f64; 18] = [0.0; 18];
        for i in 0..3 {
            for j in 0..3 {
                let index = 2*(i*3 + j);
                let evaluation = evaluate_polynomial(&self.current_matrix.d[i][j], q);
                res[index] = evaluation.re;
                res[index + 1] = evaluation.im;
            }
        }
        res
    }
}

fn evaluated_matrix_is_trivial(matrix: [f64; 18]) -> bool {
    (0..18).all(|i| matrix[i] == match i {
        0 | 8 | 16 => 1.0,
        _ => 0.0
    })
}

fn evaluate_polynomial(p: &Polynomial, q: Complex<f64>) -> Complex<f64> {
    let mut res = Complex::new(0.0, 0.0);
    for (pow, coef) in p.data.iter() {
        res += (*coef as f64) * q.powi(*pow)
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_trivial_matrix() {
        let group = Group::new();
        let q = Complex::new(60.0, 42.0);
        let evaluated = group.evaluate(q);
        assert!(evaluated_matrix_is_trivial(evaluated));
    }

    #[test]
    fn evaluate_non_trivial_matrix() {
        let mut group = Group::new();
        group.push(&Direction::North);
        let q = Complex::new(60.0, 42.0);
        let evaluated = group.evaluate(q);
        assert!(!evaluated_matrix_is_trivial(evaluated));
    }

    #[test]
    fn pushing_north_moves_north() {
        let mut group = Group::new();
        group.push(&Direction::North);
        assert_eq!(group.current_matrix, group.north_matrix);
    }

    #[test]
    fn going_north_and_south_does_nothing() {
        let mut group = Group::new();
        group.push(&Direction::North);
        group.push(&Direction::South);
        assert_eq!(group.current_matrix, Matrix::identity());
    }

    #[test]
    fn going_south_and_north_does_nothing() {
        let mut group = Group::new();
        group.push(&Direction::South);
        group.push(&Direction::North);
        assert_eq!(group.current_matrix, Matrix::identity());
    }

    #[test]
    fn going_east_and_west_does_nothing() {
        let mut group = Group::new();
        group.push(&Direction::East);
        group.push(&Direction::West);
        assert_eq!(group.current_matrix, Matrix::identity());
    }

    #[test]
    fn going_west_and_east_does_nothing() {
        let mut group = Group::new();
        group.push(&Direction::West);
        group.push(&Direction::East);
        assert_eq!(group.current_matrix, Matrix::identity());
    }

    #[test]
    fn evaluate_polynomial_trivial_polynomial() {
        let p = Polynomial::zero();
        assert_eq!(Complex::new(0.0, 0.0), evaluate_polynomial(&p, Complex::new(1.0, 0.0)));
        assert_eq!(Complex::new(0.0, 0.0), evaluate_polynomial(&p, Complex::new(0.0, 1.0)));
        assert_eq!(Complex::new(0.0, 0.0), evaluate_polynomial(&p, Complex::new(1.0, 1.0)));
    }

    #[test]
    fn evaluate_polynomial_constant_polynomial() {
        let p = Polynomial::one();
        assert_eq!(Complex::new(1.0, 0.0), evaluate_polynomial(&p, Complex::new(1.0, 0.0)));
        assert_eq!(Complex::new(1.0, 0.0), evaluate_polynomial(&p, Complex::new(0.0, 1.0)));
        assert_eq!(Complex::new(1.0, 0.0), evaluate_polynomial(&p, Complex::new(1.0, 1.0)));
    }

    #[test]
    fn evaluate_polynomial_linear_polynomial() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        let p = Polynomial { data: map };
        assert_eq!(Complex::new(2.0, 0.0), evaluate_polynomial(&p, Complex::new(1.0, 0.0)));
        assert_eq!(Complex::new(0.0, 2.0), evaluate_polynomial(&p, Complex::new(0.0, 1.0)));
        assert_eq!(Complex::new(2.0, 2.0), evaluate_polynomial(&p, Complex::new(1.0, 1.0)));
    }

    #[test]
    fn evaluate_polynomial_non_trivial_polynomial() {
        let mut map = HashMap::new();
        map.insert(-2, -2);
        map.insert(2, -2);
        let p = Polynomial { data: map };
        assert_eq!(Complex::new(-4.0, 0.0), evaluate_polynomial(&p, Complex::new(1.0, 0.0)));
        assert_eq!(Complex::new(4.0, 0.0), evaluate_polynomial(&p, Complex::new(0.0, 1.0)));
        assert_eq!(Complex::new(0.0, -3.0), evaluate_polynomial(&p, Complex::new(1.0, 1.0)));
    }
}