use core::panic;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    West,
    South,
    None,
}

impl Direction {
    pub fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::None => Direction::None,
        }
    }

    pub fn compass() -> [Direction; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }

    pub fn turn(&self) -> [Direction; 2] {
        match self {
            Direction::North => [Direction::West, Direction::East],
            Direction::East => [Direction::North, Direction::South],
            Direction::West => [Direction::South, Direction::North],
            Direction::South => [Direction::East, Direction::West],
            Direction::None => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn advance(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::None => *self,
        }
    }

    pub fn advance_check(&self, dir: Direction, width: usize, height: usize) -> Option<Self> {
        match dir {
            Direction::North if self.y > 0 => Some(self.advance(dir)),
            Direction::East if self.x < width - 1 => Some(self.advance(dir)),
            Direction::West if self.x > 0 => Some(self.advance(dir)),
            Direction::South if self.y < height - 1 => Some(self.advance(dir)),
            _ => None,
        }
    }

    pub fn von_neumann_neighbors(&self, width: usize, height: usize) -> Vec<Position> {
        Direction::compass()
            .into_iter()
            .filter_map(|dir| self.advance_check(dir, width, height))
            .collect()
    }

    pub fn grid_get<'a, 'b: 'a, T>(&'b self, grid: &'a [Vec<T>]) -> &'a T {
        &grid[self.y][self.x]
    }

    pub fn get_direction_to(&self, other: Position) -> Direction {
        let offset = (other.x as i32 - self.x as i32, other.y as i32 - self.y as i32);

        match offset {
            (0, 0) => Direction::None,
            (dx, 0) if dx > 0 => Direction::East,
            (dx, 0) if dx < 0 => Direction::West,
            (0, dy) if dy > 0 => Direction::South,
            (0, dy) if dy < 0 => Direction::North,
            _ => panic!("No clear direction"),
        }
    }
}
