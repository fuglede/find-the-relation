mod algebra;
mod utils;
mod group;

use js_sys::{Array};
use num::{Complex, One};
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
    qs: [Complex<f64>; 3],
    group: Group,
    evaluated: [[Complex<f64>; 9]; 3],
    evaluated_is_trivial: [bool; 3]
}

#[wasm_bindgen]
impl Game {
    pub fn easy() -> Game {
        let group = Group::new();
        let trivial_evaluated = group.evaluate(Complex::one());
        let evaluated = [trivial_evaluated, trivial_evaluated, trivial_evaluated];
        let evaluated_is_trivial = [false; 3];
        Game {
            qs: [Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0), Complex::new(0.0, 1.0)],
            group: group,
            evaluated: evaluated,
            evaluated_is_trivial: evaluated_is_trivial
        }
    }

    pub fn push(&mut self, direction: Direction) {
        self.group.push(&direction);
        self.evaluated = [
            self.group.evaluate(self.qs[0]),
            self.group.evaluate(self.qs[1]),
            self.group.evaluate(self.qs[2])];
        self.evaluated_is_trivial = [
            evaluated_matrix_is_trivial(self.evaluated[0]),
            evaluated_matrix_is_trivial(self.evaluated[1]),
            evaluated_matrix_is_trivial(self.evaluated[2])
        ]
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