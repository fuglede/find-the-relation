use crate::algebra::{evaluate_polynomial, Matrix};
use wasm_bindgen::prelude::*;
use num::complex::Complex;

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
    pub fn new(q: &Complex<f64>) -> Self {
        // See https://arxiv.org/abs/1904.11730v3
        let mut north_matrix = Matrix::zero();
        north_matrix.d[0][2] = evaluate_polynomial(&vec![(-1, -1)], &q);
        north_matrix.d[1][1] = evaluate_polynomial(&vec![(1, -1)], &q);
        north_matrix.d[1][2] = evaluate_polynomial(&vec![(-1, -1), (1, 1)], &q);
        north_matrix.d[2][0] = evaluate_polynomial(&vec![(0, -1)], &q);
        north_matrix.d[2][2] = evaluate_polynomial(&vec![(-1, -1), (0, 1)], &q);

        let mut south_matrix = Matrix::zero();
        south_matrix.d[0][0] = evaluate_polynomial(&vec![(0, 1), (1, -1)], &q);
        south_matrix.d[0][2] = evaluate_polynomial(&vec![(0, -1)], &q);
        south_matrix.d[1][0] = evaluate_polynomial(&vec![(-1, 1), (1, -1)], &q);
        south_matrix.d[1][1] = evaluate_polynomial(&vec![(-1, -1)], &q);
        south_matrix.d[2][0] = evaluate_polynomial(&vec![(1, -1)], &q);

        let mut east_matrix = Matrix::identity();
        east_matrix.d[0][0] = evaluate_polynomial(&vec![(-1, -1)], &q);
        east_matrix.d[0][1] = evaluate_polynomial(&vec![(0, 1)], &q);
        east_matrix.d[2][1] = evaluate_polynomial(&vec![(0, 1)], &q);
        east_matrix.d[2][2] = evaluate_polynomial(&vec![(1, -1)], &q);

        let mut west_matrix = Matrix::identity();
        west_matrix.d[0][0] = evaluate_polynomial(&vec![(1, -1)], &q);
        west_matrix.d[0][1] = evaluate_polynomial(&vec![(1, 1)], &q);
        west_matrix.d[2][1] = evaluate_polynomial(&vec![(-1, 1)], &q);
        west_matrix.d[2][2] = evaluate_polynomial(&vec![(-1, -1)], &q);

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

    pub fn flatten(&self) -> [Complex<f64>; 9] {
        let mut res: [Complex<f64>; 9] = [Complex::new(0.0, 0.0); 9];
        for i in 0..3 {
            for j in 0..3 {
                let index = 3*i + j;
                res[index] = self.current_matrix.d[i][j];
            }
        }
        res
    }
}

pub fn evaluated_matrix_is_trivial(matrix: [Complex<f64>; 9]) -> bool {
    (0..9).all(|i| matrix[i] == match i {
        0 | 4 | 8 => Complex::new(1.0, 0.0),
        _ => Complex::new(0.0, 0.0)
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_trivial_matrix() {
        let q = Complex::new(60.0, 42.0);
        let group = Group::new(&q);
        let evaluated = group.flatten();
        assert!(evaluated_matrix_is_trivial(evaluated));
    }

    #[test]
    fn evaluate_non_trivial_matrix() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::North);
        let evaluated = group.flatten();
        assert!(!evaluated_matrix_is_trivial(evaluated));
    }

    #[test]
    fn pushing_north_moves_north() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::North);
        assert_eq!(group.current_matrix, group.north_matrix);
    }

    #[test]
    fn going_north_and_south_does_nothing() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::North);
        group.push(&Direction::South);
        assert_eq!(group.current_matrix, Matrix::identity());
    }

    #[test]
    fn going_south_and_north_does_nothing() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::South);
        group.push(&Direction::North);
        assert_eq!(group.current_matrix, Matrix::identity());
    }

    #[test]
    fn going_east_and_west_does_nothing() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::East);
        group.push(&Direction::West);
        assert_eq!(group.current_matrix, Matrix::identity());
    }

    #[test]
    fn going_west_and_east_does_nothing() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::West);
        group.push(&Direction::East);
        assert_eq!(group.current_matrix, Matrix::identity());
    }
}