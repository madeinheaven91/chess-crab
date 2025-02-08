#[derive(Copy, Clone, Debug)]
pub enum DIRECTION {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl From<usize> for DIRECTION {
    fn from(val: usize) -> Self {
        match val {
            0 => DIRECTION::N,
            1 => DIRECTION::NE,
            2 => DIRECTION::E,
            3 => DIRECTION::SE,
            4 => DIRECTION::S,
            5 => DIRECTION::SW,
            6 => DIRECTION::W,
            7 => DIRECTION::NW,
            _ => unreachable!()
        }
    }
}

impl From<DIRECTION> for usize {
    fn from(value: DIRECTION) -> Self {
        match value {
            DIRECTION::N => 0,
            DIRECTION::NE => 1,
            DIRECTION::E => 2,
            DIRECTION::SE => 3,
            DIRECTION::S => 4,
            DIRECTION::SW => 5,
            DIRECTION::W => 6,
            DIRECTION::NW => 7,
        }
    }
}
