use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size(pub usize, pub usize);

#[wasm_bindgen]
impl Size {
    #[wasm_bindgen(constructor)]
    pub fn new(a: usize, b: usize) -> Size {
        Size(a, b)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub fn distance(p1: Position, p2: Position) -> f32 {
    ((p2.x as f32 - p1.x as f32).powi(2) + (p2.y as f32 - p1.y as f32).powi(2)).sqrt()
}
