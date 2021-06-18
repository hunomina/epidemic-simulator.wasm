use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub struct Movement {
    pub horizontal_direction: HorizontalDirection,
    pub vertical_direction: VerticalDirection,
}

impl Distribution<Movement> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Movement {
        Movement {
            horizontal_direction: rand::random(),
            vertical_direction: rand::random(),
        }
    }
}

pub enum VerticalDirection {
    Up,
    Down,
    None,
}

impl Distribution<VerticalDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VerticalDirection {
        match rng.gen_range(0..3) {
            0 => VerticalDirection::None,
            1 => VerticalDirection::Up,
            _ => VerticalDirection::Down,
        }
    }
}

pub enum HorizontalDirection {
    Left,
    Right,
    None,
}

impl Distribution<HorizontalDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HorizontalDirection {
        match rng.gen_range(0..3) {
            0 => HorizontalDirection::None,
            1 => HorizontalDirection::Left,
            _ => HorizontalDirection::Right,
        }
    }
}
