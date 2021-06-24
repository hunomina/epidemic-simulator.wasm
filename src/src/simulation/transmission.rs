use crate::person::{Person, State};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::min;

pub struct TransmissionEvaluator;

impl TransmissionEvaluator {
    /*
       "target" is the person for which the probability is calculated for
       "source" is the person "target" is encountering
    */
    pub fn evaluate(target: &Person, source: &Person) -> u8 {
        assert!(!target.is(State::Sick) && source.is(State::Sick));
        100_u8
            .checked_sub(target.protection_coefficient())
            .or(Some(0))
            .unwrap()
            .checked_sub(source.contamination_reduction_coefficient())
            .or(Some(0))
            .unwrap()
    }
}

pub trait TransmissionBlocker {
    fn protection_coefficient(&self) -> u8;
    fn contamination_reduction_coefficient(&self) -> u8;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Protection {
    Mask,
    WashHands,
    Vacin,
}

impl TransmissionBlocker for Protection {
    fn protection_coefficient(&self) -> u8 {
        match self {
            Self::Mask => 70,
            Self::WashHands => 10,
            Self::Vacin => 90,
        }
    }
    fn contamination_reduction_coefficient(&self) -> u8 {
        match self {
            Self::Mask => 25,
            Self::WashHands => 20,
            Self::Vacin => 0,
        }
    }
}

impl TransmissionBlocker for Vec<Protection> {
    fn protection_coefficient(&self) -> u8 {
        self.iter()
            .fold(0, |acc, item| min(acc + item.protection_coefficient(), 100))
    }
    fn contamination_reduction_coefficient(&self) -> u8 {
        self.iter().fold(0, |acc, item| {
            min(acc + item.contamination_reduction_coefficient(), 100)
        })
    }
}

// todo : add to simulation configuration
// todo front: onglet "états", "space", "protections"
const VACIN_PROBABILITY: u8 = 35;
const MASK_PROBABILITY: u8 = 20;
const WASH_HAND_PROBABILITY: u8 = 50;

pub fn get_random_protections() -> Vec<Protection> {
    let mut protections = vec![];
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..100) <= MASK_PROBABILITY {
        protections.push(Protection::Mask);
    }
    if rng.gen_range(0..100) <= WASH_HAND_PROBABILITY {
        protections.push(Protection::WashHands);
    }
    if rng.gen_range(0..100) <= VACIN_PROBABILITY {
        protections.push(Protection::Vacin);
    }
    protections
}
