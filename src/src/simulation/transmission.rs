use super::config::ProtectionRepartition;
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

pub fn get_random_protections(protection_repartition: ProtectionRepartition) -> Vec<Protection> {
    let mut protections = vec![];
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..100) <= protection_repartition.mask_percentage {
        protections.push(Protection::Mask);
    }
    if rng.gen_range(0..100) <= protection_repartition.wash_hands_percentage {
        protections.push(Protection::WashHands);
    }
    if rng.gen_range(0..100) <= protection_repartition.vacin_percentage {
        protections.push(Protection::Vacin);
    }
    protections
}
