use std::cmp::min;

use super::rules::Rules;
use crate::space::physics::Size;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub size: Size,
    pub subject_repartition: SubjectsRepartition,
    pub protection_repartition: ProtectionRepartition,
    pub rules: Rules,
}

#[wasm_bindgen]
impl Configuration {
    #[wasm_bindgen(constructor)]
    pub fn new(
        size: Size,
        subject_repartition: SubjectsRepartition,
        protection_repartition: ProtectionRepartition,
        rules: Rules,
    ) -> Configuration {
        Configuration {
            size,
            subject_repartition,
            protection_repartition,
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

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProtectionRepartition {
    pub mask_percentage: u8,
    pub wash_hands_percentage: u8,
    pub vacin_percentage: u8,
}

#[wasm_bindgen]
impl ProtectionRepartition {
    #[wasm_bindgen(constructor)]
    pub fn new(mask_percentage: u8, wash_hands_percentage: u8, vacin_percentage: u8) -> Self {
        ProtectionRepartition {
            mask_percentage: min(mask_percentage, 100),
            wash_hands_percentage: min(wash_hands_percentage, 100),
            vacin_percentage: min(vacin_percentage, 100),
        }
    }
}
