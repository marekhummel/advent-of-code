#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn advance(&self, dir: Direction, width: usize, height: usize) -> Option<Self> {
        match dir {
            Direction::North if self.y > 0 => Some(Position {
                x: self.x,
                y: self.y - 1,
            }),
            Direction::East if self.x < width - 1 => Some(Position {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::West if self.x > 0 => Some(Position {
                x: self.x - 1,
                y: self.y,
            }),
            Direction::South if self.y < height - 1 => Some(Position {
                x: self.x,
                y: self.y + 1,
            }),
            _ => None,
        }
    }
}
