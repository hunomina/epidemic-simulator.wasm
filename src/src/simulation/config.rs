use super::rules::Rules;
use crate::space::physics::Size;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub size: Size,
    pub subject_repartition: SubjectsRepartition,
    pub rules: Rules,
}

#[wasm_bindgen]
impl Configuration {
    #[wasm_bindgen(constructor)]
    pub fn new(
        size: Size,
        subject_repartition: SubjectsRepartition,
        rules: Rules,
    ) -> Configuration {
        Configuration {
            size,
            subject_repartition,
            rules,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SubjectsRepartition {
    pub sick_percentage: u8,
    pub recovered_percentage: u8,
}

#[wasm_bindgen]
impl SubjectsRepartition {
    #[wasm_bindgen(constructor)]
    pub fn new(sick_percentage: u8, recovered_percentage: Option<u8>) -> SubjectsRepartition {
        assert!(sick_percentage <= 100);
        let r = match recovered_percentage {
            Some(v) => {
                assert!(sick_percentage + v <= 100);
                v
            }
            _ => 0,
        };
        SubjectsRepartition {
            sick_percentage,
            recovered_percentage: r,
        }
    }
}
