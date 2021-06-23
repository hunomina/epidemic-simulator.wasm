use crate::{simulation::config::SubjectsRepartition, space::physics::Position};
use rand::Rng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub position: Position,
    pub state_history: Vec<StateEvent>,
}

impl Person {
    pub fn current_state(&self) -> Option<&StateEvent> {
        self.state_history.iter().last()
    }

    pub fn is(&self, state: State) -> bool {
        self.current_state()
            .map_or_else(|| false, |state_event| state_event.state == state)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Healthy,
    Sick,
    Recovered,
}

impl State {
    pub fn random(repartition: SubjectsRepartition) -> State {
        let is_sick =
            |value: &u8| -> bool { ((100 - repartition.sick_percentage)..=100).contains(value) };
        let is_recovered = |value: &u8| -> bool {
            ((100 - repartition.sick_percentage - repartition.recovered_percentage)
                ..=(100 - repartition.sick_percentage))
                .contains(value)
        };
        let v = rand::thread_rng().gen_range(0..100);

        if is_sick(&v) {
            State::Sick
        } else if is_recovered(&v) {
            State::Recovered
        } else {
            State::Healthy
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StateEvent {
    pub state: State,
    pub since: usize,
}
