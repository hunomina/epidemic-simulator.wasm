use super::config::Configuration;
use crate::{
    person::{Person, State, StateEvent},
    space::{movement::*, physics::Position},
};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Simulation {
    counter: usize,
    configuration: Configuration,
    subjects: Vec<Person>,
}

impl Simulation {
    pub fn new(configuration: Configuration, subjects_count: usize) -> Simulation {
        let mut rng = rand::thread_rng();
        let size = configuration.size;
        let repartition = configuration.subject_repartition;

        Simulation {
            counter: 0,
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
                })
                .collect(),
        }
    }

    pub fn next_generation(&mut self) {
        self.counter += 1;
        self.move_subjects();
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
                if position.y + distance >= max_height {
                    next_position.y = max_height;
                } else {
                    next_position.y += distance;
                }
            }
            VerticalDirection::Down => {
                let distance = rng.gen_range(0..(speed + 1));
                if position.y as i32 - distance as i32 > 0 {
                    next_position.y -= distance;
                } else {
                    next_position.y = 0;
                }
            }
            _ => {}
        }

        match movement.horizontal_direction {
            HorizontalDirection::Left => {
                let distance = rng.gen_range(0..(speed + 1));
                if position.x as i32 - distance as i32 > 0 {
                    next_position.x -= distance;
                } else {
                    next_position.x = 0;
                }
            }
            HorizontalDirection::Right => {
                let max_width = config.size.0 - 1;
                let distance = rng.gen_range(0..(speed + 1));
                if position.x + distance >= max_width {
                    next_position.x = max_width;
                } else {
                    next_position.x += distance;
                }
            }
            HorizontalDirection::None => {}
        }
        next_position
    }
}
