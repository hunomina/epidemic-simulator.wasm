use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rules {
    pub movement_speed: usize,
    pub safe_distance: f32,
}

#[wasm_bindgen]
impl Rules {
    #[wasm_bindgen(constructor)]
    pub fn new(movement_speed: usize, safe_distance: f32) -> Rules {
        Rules {
            movement_speed,
            safe_distance,
        }
    }
}
