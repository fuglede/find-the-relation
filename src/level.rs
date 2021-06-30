use crate::group::{Direction, Group};
use num::Complex;

pub struct Level {
    qs: Vec<Complex<f64>>,
    pub groups: Vec<Group>,
    pub word: Vec<Direction>,
    pub flattened: Vec<[Complex<f64>; 9]>
}

impl Level {
    pub fn new(qs: Vec<Complex<f64>>) -> Level {
        let groups: Vec<Group> = Self::make_groups(&qs);
        let flattened = groups.iter().map(
            |g| g.flatten()).collect();
        let word = vec![];
        Level {
            qs,
            groups,
            word,
            flattened
        }
    }

    fn make_groups(qs: &[Complex<f64>]) -> Vec<Group> {
        qs.iter().map(Group::new).collect()
    }

    pub fn push(&mut self, direction: Direction) {
        for i in 0..self.groups.len() {
            self.groups[i].push(&direction);
        }
        let last_is_opposite = !self.word.is_empty() &&
            self.word.last().unwrap() == &match direction {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East
        };
        if last_is_opposite {
            self.word.pop();
        } else {
            self.word.push(direction);
        }
        self.update_flattened();
    }

    pub fn reset(&mut self) {
        self.groups = Self::make_groups(&self.qs);
        self.word = vec![];
        self.update_flattened();
    }

    fn update_flattened(&mut self) {
        self.flattened = self.groups.iter().map(|g| g.flatten()).collect();
    }

    pub fn word(&self) -> String {
        self.word.iter().map(|d| match d {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::East => 'E',
            Direction::West => 'W',
        }).collect::<String>()
    }

    pub fn is_solved(&self) -> bool {
        !self.word.is_empty() && self.groups.iter().all(|g| g.current_is_identity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_word_updates_as_expected() {
        let q = Complex::new(60.0, 42.0);
        let mut level = Level::new(vec![q]);

        assert_eq!(level.word, vec![]);
        assert_eq!(level.word(), "");

        level.push(Direction::North);
        assert_eq!(level.word, vec![Direction::North]);
        assert_eq!(level.word(), "N");

        level.push(Direction::East);
        assert_eq!(level.word, vec![Direction::North, Direction::East]);
        assert_eq!(level.word(), "NE");

        level.push(Direction::West);
        assert_eq!(level.word, vec![Direction::North]);
        assert_eq!(level.word(), "N");

        level.push(Direction::South);
        assert_eq!(level.word, vec![]);
        assert_eq!(level.word(), "");
    }

    #[test]
    fn level_reset_resets() {
        let q = Complex::new(60.0, 42.0);
        let mut level = Level::new(vec![q]);

        assert!(level.groups[0].current_is_identity());

        level.push(Direction::North);
        assert!(!level.word.is_empty());
        assert!(!level.groups[0].current_is_identity());

        level.reset();
        assert!(level.word.is_empty());
        assert!(level.groups[0].current_is_identity());
    }

    #[test]
    fn level_is_solved() {
        let q = Complex::new(1.0, 0.0);
        let mut level = Level::new(vec![q]);

        assert!(!level.is_solved());
        level.push(Direction::North);
        assert!(!level.is_solved());
        level.push(Direction::North);
        assert!(level.is_solved());
    }
}