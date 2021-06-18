mod algebra;
mod group;

use std::f64::consts::PI;

use js_sys::{Array};
use num::{Complex};
use num::integer::{div_mod_floor};
use wasm_bindgen::prelude::*;
use crate::group::{Direction, Group};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    groups: Vec<Group>,
    evaluated: Vec<[Complex<f64>; 9]>
}

#[wasm_bindgen]
impl Game {
    pub fn easy() -> Game {
        let groups = vec![
            Group::new(&Complex::new(0.5, 0.0)),
            ];
        let evaluated = vec![groups[0].flatten()];
        Game {
            groups,
            evaluated
        }
    }

    pub fn push(&mut self, direction: Direction) {
        for i in 0..self.groups.len() {
            self.groups[i].push(&direction);
            self.evaluated[i] = self.groups[i].flatten();
        }
    }

    pub fn evaluated(&self) -> Array {
        let length = (self.groups.len() as u32) * 9;
        let arr = Array::new_with_length(length);
        for i in 0..length {
            let (j, k) = div_mod_floor(i, 9);
            let z = self.evaluated[j as usize][k as usize];
            let s = JsValue::from_str(&format!("{}", z));
            arr.set(i as u32, s);
        }
        arr
    }

    pub fn evaluated_polar(&self) -> Array {
        let length = (self.groups.len() as u32) * 9;
        let arr = Array::new_with_length(length);
        for i in 0..length {
            let (j, k) = div_mod_floor(i, 9);
            let (r, theta) = self.evaluated[j as usize][k as usize].to_polar();
            let s = JsValue::from_str(&format!("({}, {})", r, theta));
            arr.set(i as u32, s);
        }
        arr
    }

    pub fn evaluation_is_trivial(&self) -> Array {
        let length = self.groups.len();
        let arr = Array::new_with_length(length as u32);
        for i in 0..length {
            arr.set(i as u32, JsValue::from_bool(self.groups[i].current_is_identity()));
        }
        arr
    }

    pub fn det(&self) -> Array {
        let length = self.groups.len();
        let arr = Array::new_with_length(length as u32);
        for i in 0..length {
            arr.set(i as u32, JsValue::from_str(&format!("{}", self.groups[i].current_det())));
        }
        arr
    }

    pub fn tr(&self) -> Array {
        let length = self.groups.len();
        let arr = Array::new_with_length(length as u32);
        for i in 0..length {
            arr.set(i as u32, JsValue::from_str(&format!("{}", self.groups[i].current_tr())));
        }
        arr
    }
}