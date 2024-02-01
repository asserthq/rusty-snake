#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down
}

impl Direction {
    pub fn can_change_to(&self, dir: &Direction) -> bool {
        self.is_vertical() != dir.is_vertical()
    }

    pub fn is_vertical(&self) -> bool {
        self == &Direction::Down || self == &Direction::Up
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Coord {
    pub x: u16,
    pub y: u16
}

impl Coord {
    pub fn new(x: u16, y: u16) -> Self {
        Coord {
            x,
            y
        }
    }
}

impl From<(u16, u16)> for Coord {
    fn from(pos: (u16, u16)) -> Self {
        Coord::new(pos.0, pos.1)
    }
}
