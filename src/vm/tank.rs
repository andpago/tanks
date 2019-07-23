use crate::vm::geom::{Coords, Direction};

#[derive(Clone, Debug)]
pub struct Tank {
    pub pos: Coords,
    pub dir: Direction,
    pub hp: i64
}

const MAX_HP: i64 = 100;

impl Tank {
    pub fn new() -> Tank {
        Tank {
            pos: Coords { x: 0, y: 0 },
            dir: Direction::Right,
            hp: MAX_HP
        }
    }

    pub fn step(self: &mut Self) {
        self.pos.shift(&self.dir);
    }
}