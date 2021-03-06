mod algebra;
mod group;
mod level;

use std::f64::consts::PI;

use js_sys::Array;
use num::Complex;
use num::integer::{div_mod_floor};
use wasm_bindgen::prelude::*;
use crate::group::Direction;
use crate::level::Level;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    levels: Vec<Level>,
    qs: Vec<Vec<String>>,
    level_descriptions: Vec<String>,
    active_level: usize
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        let mut levels = vec![];
        let mut qs = vec![];
        let mut level_descriptions = vec![];

        // Level 1; q = 1
        levels.push(Level::new(vec![Complex::new(1.0, 0.0)]));
        qs.push(vec!["1".to_owned()]);
        level_descriptions.push(
            "In this first level, we will find a solution for 𝑞 = 1. That is,
            each of the four directions correspond to multiplication by one of
            the four matrices given in the rules in which all instances of 𝑞 have
            been replaced with 1. This is the simplest case of all of them. Find a
            solution by using the four buttons below, or by using the keyboard
            shortcuts. Use the reset button to get back to where you started. Remember
            that the level is completed when you find a path to the identity matrix."
            .to_owned());
        
        // Level 2; q = -1
        levels.push(Level::new(vec![Complex::new(-1.0, 0.0)]));
        qs.push(vec!["−1".to_owned()]);
        level_descriptions.push(
            "Hopefully the first level wasn't too bad. In this next one, we use
            𝑞 = −1 instead. This will make things a bit harder. Pay close attention
            to how the entries of the matrix change in your quest to find a path
            to the identity matrix. What's the shortest path you can find?"
            .to_owned());
        
        // Level 3; q = i
        levels.push(Level::new(vec![Complex::new(0.0, 1.0)]));
        qs.push(vec!["𝑖".to_owned()]);
        level_descriptions.push(
            "In the first two levels, 𝑞 was a real number. In this one, we consider
            𝑞 = 𝑖, the imaginary unit satisfying 𝑖² = −1. Can you still find a path?"
            .to_owned());
        
        // Level 4: q = 1, -1, i
        levels.push(Level::new(vec![Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0), Complex::new(0.0, 1.0)]));
        qs.push(vec!["1".to_owned(), "−1".to_owned(), "𝑖".to_owned()]);
        level_descriptions.push(
            "So far, we have tried to find paths for individual values of 𝑞. In this
            one, we have to find a path that works for all previously considered values
            at the same time. The level is completed when all three matrices are the
            identity matrix."
            .to_owned());
        
        // Level 5: q = exp(2*pi*i/3)
        levels.push(Level::new(vec![Complex::from_polar(1.0, 2.0 * PI / 3.0)]));
        qs.push(vec!["exp(2π𝑖/3)".to_owned()]);
        level_descriptions.push(
            "We continue our journey into the complex plane, this time with a value
            of 𝑞 that's neither real or imaginary but still lives on the unit circle.
            At this point, numerical errors will start to show up, and we just require
            that the result is sufficiently close to the identity matrix."
            .to_owned());
        
        // Level 6: q = exp(2*pi*i/5)
        levels.push(Level::new(vec![Complex::from_polar(1.0, 2.0 * PI / 5.0)]));
        qs.push(vec!["exp(2π𝑖/5)".to_owned()]);
        level_descriptions.push(
            "Let's do another one on the unit circle."
            .to_owned());
        
        // Level 7: q = exp(2*pi*i/3), exp(2*pi*i/5)
        levels.push(Level::new(vec![
            Complex::from_polar(1.0, 2.0 * PI / 3.0),
            Complex::from_polar(1.0, 2.0 * PI / 5.0)]));
        qs.push(vec!["exp(2π𝑖/3)".to_owned(), "exp(2π𝑖/5)".to_owned()]);
        level_descriptions.push(
            "Can you complete both of the previous levels at once?"
            .to_owned());

        // Level 8; q = 2
        levels.push(Level::new(vec![Complex::new(2.0, 0.0)]));
        qs.push(vec!["2".to_owned()]);
        level_descriptions.push(
            "So far, all of our values of 𝑞 have been somewhere on the unit
            circle. Let's move on to ones that aren't, starting with 𝑞 = 2."
            .to_owned());

        // Level 9; q = 3
        levels.push(Level::new(vec![Complex::new(3.0, 0.0)]));
        qs.push(vec!["3".to_owned()]);
        level_descriptions.push(
            "Final level! As our final challenge, we will consider 𝑞 = 3.
            It is an open question of mathematics whether or not this one is
            possible. If you find a solution, or if you find that none can 
            possibly exist, please get in touch."
            .to_owned());

        let active_level = 0;
        Game { levels, qs, level_descriptions, active_level }
    }

    pub fn change_level(&mut self, i: usize) {
        self.active_level = i;
    }

    pub fn active_level(&self) -> usize {
        self.active_level
    }

    pub fn level_description(&self) -> String {
        self.level_descriptions[self.active_level].clone()
    }

    pub fn qs(&self) -> Array {
        let level_qs = self.qs[self.active_level].clone();
        let length = level_qs.len();
        let arr = Array::new_with_length(length as u32);
        for (i, item) in level_qs.iter().enumerate() {
            arr.set(i as u32, JsValue::from_str(&item.to_string()));
        }
        arr
    }

    pub fn matrix_entries(&self) -> Array {
        let level = &self.levels[self.active_level];
        let length = (level.groups.len() as u32) * 9;
        let arr = Array::new_with_length(length);
        for i in 0..length {
            let (j, k) = div_mod_floor(i, 9);
            let z = &level.flattened[j as usize][k as usize];
            // Only show imaginary part if 𝑞 itself is non-real.
            let s = if level.qs[j as usize].im == 0.0 { f64toa(&z.re) } else { ztoa(z) };
            arr.set(i as u32, JsValue::from_str(&s));
        }
        arr
    }

    pub fn matrix_is_identity(&self) -> Array {
        let level = &self.levels[self.active_level];
        let length = level.groups.len();
        let arr = Array::new_with_length(length as u32);
        for (i, group) in level.groups.iter().enumerate() {
            arr.set(i as u32, JsValue::from_bool(group.current_is_identity()));
        }
        arr
    }

    pub fn distance(&self) -> Array {
        let level = &self.levels[self.active_level];
        let length = level.groups.len();
        let arr = Array::new_with_length(length as u32);
        for (i, group) in level.groups.iter().enumerate() {
            let distance_string = if level.word.is_empty() {
                "∞".to_owned()
            } else {
                format!("{:.5}", group.distance_from_identity())
            };
            arr.set(i as u32, JsValue::from_str(&distance_string));
        }
        arr
    }

    pub fn push(&mut self, direction: Direction) {
        self.levels[self.active_level].push(direction);
    }

    pub fn reset(&mut self) {
        self.levels[self.active_level].reset();
    }

    pub fn is_solved(&self) -> bool {
        self.levels[self.active_level].is_solved()
    }

    pub fn word(&self) -> String {
        self.levels[self.active_level].word()
    }
}

impl Default for Game {   
    fn default() -> Self {
        Self::new()       
    }                     
}

fn f64toa(x: &f64) -> String {
    let mut buf = Vec::new();
    dtoa::write(&mut buf, *x).unwrap();
    std::str::from_utf8(&buf).unwrap().to_string()
}

fn ztoa(z: &Complex<f64>) -> String {
    format!("{} + {}𝑖", f64toa(&z.re), f64toa(&z.im))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64toa_works_small_number() {
        let actual = f64toa(&2.0);
        assert_eq!("2.0", actual);
    }
    #[test]
    fn f64toa_works_large_number() {
        let actual = f64toa(&2e25);
        assert_eq!("2e25", actual);
    }
}