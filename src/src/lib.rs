pub mod person;
pub mod simulation;
mod space;
mod utils;

use simulation::{config::Configuration, simulation::Simulation};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_simulation(configuration: Configuration, subjects_count: usize) -> JsValue {
    JsValue::from_serde(&Simulation::new(configuration, subjects_count)).unwrap()
}

#[wasm_bindgen]
pub fn next_generation(s: &JsValue) -> JsValue {
    let mut simulation: Simulation = s.into_serde().unwrap();
    simulation.next_generation();
    JsValue::from_serde(&simulation).unwrap()
}
