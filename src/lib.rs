mod algebra;
mod utils;
mod group;

use js_sys::{Array};
use num::{Complex};
use num::integer::{div_mod_floor};
use wasm_bindgen::prelude::*;
use crate::group::{Direction, Group, evaluated_matrix_is_trivial};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    groups: [Group; 3],
    evaluated: [[Complex<f64>; 9]; 3],
    evaluated_is_trivial: [bool; 3]
}

#[wasm_bindgen]
impl Game {
    pub fn easy() -> Game {
        let groups = [
            Group::new(&Complex::new(1.0, 0.0)),
            Group::new(&Complex::new(-1.0, 0.0)),
            Group::new(&Complex::new(0.0, 1.0))
            ];
        let evaluated = [groups[0].flatten(), groups[1].flatten(), groups[2].flatten()];
        let evaluated_is_trivial = [false; 3];
        Game {
            groups,
            evaluated,
            evaluated_is_trivial
        }
    }

    pub fn push(&mut self, direction: Direction) {
        for i in 0..3 {
            self.groups[i].push(&direction);
            self.evaluated[i] = self.groups[i].flatten();
            self.evaluated_is_trivial[i] =
                evaluated_matrix_is_trivial(self.evaluated[i]);
        }
    }

    pub fn evaluated(&self) -> Array {
        let arr = Array::new_with_length(27);
        for i in 0..27 {
            let (j, k) = div_mod_floor(i, 9);
            let s = JsValue::from_str(&format!("{}", self.evaluated[j][k]));
            arr.set(i as u32, s);
        }
        arr
    }

    pub fn evaluation_is_trivial(&self) -> Array {
        let arr = Array::new_with_length(3);
        for i in 0..3 {
            arr.set(i as u32, JsValue::from_bool(self.evaluated_is_trivial[i]));
        }
        arr
    }
}