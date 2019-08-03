use core::cmp;

#[derive(Debug, Clone)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Debug)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl From<u8> for Direction {
    fn from(u: u8) -> Self {
        match u {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("cannot cast {} to Direction", u),
        }
    }
}

impl Into<u8> for &Direction {
    fn into(self: Self) -> u8 {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

impl Direction {
    pub fn into_u8(self: &Self) -> u8 {
        self.into()
    }

    pub fn rotate(self: &Self, rot: &Direction) -> Self {
        ((self.into_u8() + rot.into_u8()) % 4).into()
    }
}

impl Coords {
    pub fn shift(self: &mut Self, dir: &Direction) {
        match dir {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }
}

impl cmp::PartialEq for Coords {
    fn eq(self: &Self, other: &Self) -> bool {
        return self.y == other.y && self.x == other.x
    }
}