use std::collections::HashMap;

use super::{
    config::Configuration,
    report::Report,
    transmission::{get_random_protections, TransmissionEvaluator},
};
use crate::{
    person::{Person, State, StateEvent},
    space::{
        movement::*,
        physics::{distance, Position},
    },
};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Simulation {
    generation: usize,
    configuration: Configuration,
    subjects: Vec<Person>,
    reports: HashMap<usize, Report>,
}

impl Simulation {
    pub fn new(configuration: Configuration, subjects_count: usize) -> Simulation {
        let mut rng = rand::thread_rng();
        let size = configuration.size;
        let repartition = configuration.subject_repartition;

        Simulation {
            generation: 0,
            configuration,
            subjects: (0..subjects_count)
                .map(|_| Person {
                    position: Position {
                        x: rng.gen_range(0..(size.0 - 1)),
                        y: rng.gen_range(0..(size.1 - 1)),
                    },
                    state_history: vec![StateEvent {
                        state: State::random(repartition),
                        since: 0,
                    }],
                    protections: get_random_protections(),
                })
                .collect(),
            reports: HashMap::new(),
        }
    }

    pub fn next_generation(&mut self) {
        self.generation += 1;
        self.move_subjects();
        self.update_subjects_state();
        self.create_report();
    }

    fn move_subjects(&mut self) {
        for subject in self.subjects.iter_mut() {
            subject.position = Simulation::compute_next_position(
                subject.position.clone(),
                rand::random(),
                &self.configuration,
            );
        }
    }

    fn compute_next_position(
        position: Position,
        movement: Movement,
        config: &Configuration,
    ) -> Position {
        let mut rng = rand::thread_rng();
        let speed = config.rules.movement_speed;
        let mut next_position = position.clone();

        match movement.vertical_direction {
            VerticalDirection::Up => {
                let max_height = config.size.1 - 1;
                let distance = rng.gen_range(0..(speed + 1));
                next_position.y = if position.y + distance >= max_height {
                    max_height
                } else {
                    next_position.y + distance
                };
            }
            VerticalDirection::Down => {
                let distance = rng.gen_range(0..(speed + 1));
                next_position.y = if position.y as i32 - distance as i32 > 0 {
                    next_position.y - distance
                } else {
                    0
                };
            }
            _ => {}
        }

        match movement.horizontal_direction {
            HorizontalDirection::Left => {
                let distance = rng.gen_range(0..(speed + 1));
                next_position.x = if position.x as i32 - distance as i32 > 0 {
                    next_position.x - distance
                } else {
                    0
                };
            }
            HorizontalDirection::Right => {
                let max_width = config.size.0 - 1;
                let distance = rng.gen_range(0..(speed + 1));
                next_position.x = if position.x + distance >= max_width {
                    max_width
                } else {
                    next_position.x + distance
                };
            }
            _ => {}
        }
        next_position
    }

    fn update_subjects_state(&mut self) {
        let new_sicks = self
            .subjects
            .iter()
            .enumerate()
            .filter_map(|(i, subject)| {
                if !subject.is(State::Healthy) {
                    return None;
                }
                if let Some((_, n)) = self.subjects.iter().enumerate().find(|(j, n)| {
                    i != *j
                        && n.is(State::Sick)
                        && distance(subject.position, n.position)
                            < self.configuration.rules.safe_distance
                }) {
                    return Some((i, TransmissionEvaluator::evaluate(&subject, &n)));
                }
                None
            })
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();
        for (i, probability_to_become_sick) in new_sicks.into_iter() {
            let target = self.subjects.iter_mut().nth(i).unwrap();
            if probability_to_become_sick
                .checked_sub(rng.gen_range(0..100))
                .or(Some(0))
                .unwrap()
                != 0
            {
                target.state_history.push(StateEvent {
                    state: State::Sick,
                    since: self.generation,
                })
            }
        }
    }

    fn create_report(&mut self) {
        self.reports.insert(
            self.generation,
            self.subjects.iter().fold(Report::empty(), |mut acc, sub| {
                match sub.current_state().unwrap().state {
                    State::Healthy => acc.healthy += 1,
                    State::Sick => acc.sick += 1,
                    State::Recovered => acc.recovered += 1,
                };
                acc
            }),
        );
    }
}
