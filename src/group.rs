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

        let mut east_matrix = Matrix::zero();
        east_matrix.d[0][0] = evaluate_polynomial(&vec![(-1, -1)], &q);
        east_matrix.d[0][1] = evaluate_polynomial(&vec![(0, 1)], &q);
        east_matrix.d[1][1] = evaluate_polynomial(&vec![(0, 1)], &q);
        east_matrix.d[2][1] = evaluate_polynomial(&vec![(0, 1)], &q);
        east_matrix.d[2][2] = evaluate_polynomial(&vec![(1, -1)], &q);

        let mut west_matrix = Matrix::zero();
        west_matrix.d[0][0] = evaluate_polynomial(&vec![(1, -1)], &q);
        west_matrix.d[0][1] = evaluate_polynomial(&vec![(1, 1)], &q);
        west_matrix.d[1][1] = evaluate_polynomial(&vec![(0, 1)], &q);
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

    pub fn current_is_identity(&self) -> bool {
        self.current_matrix == Matrix::identity()
    }

    pub fn distance_from_identity(&self) -> f64 {
        self.current_matrix.distance_from_identity()
    }

    pub fn current_det(&self) -> Complex<f64> {
        self.current_matrix.det()
    }

    pub fn current_tr(&self) -> Complex<f64> {
        self.current_matrix.tr()
    }

    pub fn flatten(&self) -> [Complex<f64>; 9] {
        self.current_matrix.flatten()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_starts_at_identity() {
        let q = Complex::new(60.0, 42.0);
        let group = Group::new(&q);
        assert!(group.current_is_identity());
    }

    #[test]
    fn group_moves_to_non_identity() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::North);
        assert!(!group.current_is_identity());
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