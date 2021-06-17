mod algebra;
mod utils;
mod group;

use js_sys::{Array,Float64Array};
use num::{Complex, One};
use wasm_bindgen::prelude::*;
use crate::group::{Direction, Group};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(name);
}

#[wasm_bindgen]
pub struct Game {
    qs: [Complex<f64>; 3],
    group: Group,
    q1_evaluated: [f64; 18],
    q2_evaluated: [f64; 18],
    q3_evaluated: [f64; 18],
}

#[wasm_bindgen]
impl Game {
    pub fn easy() -> Game {
        let group = Group::new();
        let evaluated = group.evaluate(Complex::one());
        Game {
            qs: [Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0), Complex::new(0.0, 1.0)],
            group: group,
            q1_evaluated: evaluated,
            q2_evaluated: evaluated,
            q3_evaluated: evaluated
        }
    }

    pub fn push(&mut self, direction: Direction) {
        self.group.push(&direction);
        self.q1_evaluated = self.group.evaluate(self.qs[0]);
        self.q2_evaluated = self.group.evaluate(self.qs[1]);
        self.q3_evaluated = self.group.evaluate(self.qs[2]);
    }

    pub fn q1_evaluated(self) -> Float64Array {
        let array: Array = self.q1_evaluated.iter().map(|x| JsValue::from(*x as f64)).collect();
        Float64Array::new(&array)
    }
}