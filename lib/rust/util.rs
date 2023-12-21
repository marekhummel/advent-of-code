use core::panic;

use crate::types::Grid;

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
pub struct Index {
    pub i: usize,
    pub j: usize,
}

impl Index {
    pub fn from_tuple(tuple: (usize, usize)) -> Self {
        Self { i: tuple.0, j: tuple.1 }
    }

    pub fn advance(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Index {
                i: self.i,
                j: self.j - 1,
            },
            Direction::East => Index {
                i: self.i + 1,
                j: self.j,
            },
            Direction::West => Index {
                i: self.i - 1,
                j: self.j,
            },
            Direction::South => Index {
                i: self.i,
                j: self.j + 1,
            },
            Direction::None => *self,
        }
    }

    pub fn advance_check(&self, dir: Direction, size: Size) -> Option<Self> {
        match dir {
            Direction::North if self.j > 0 => Some(self.advance(dir)),
            Direction::East if self.i < size.width - 1 => Some(self.advance(dir)),
            Direction::West if self.i > 0 => Some(self.advance(dir)),
            Direction::South if self.j < size.height - 1 => Some(self.advance(dir)),
            _ => None,
        }
    }

    pub fn von_neumann_neighbors(&self, size: Size) -> Vec<Index> {
        Direction::compass()
            .into_iter()
            .filter_map(|dir| self.advance_check(dir, size))
            .collect()
    }

    pub fn grid_get<'a, 'b: 'a, T>(&'b self, grid: &'a [Vec<T>]) -> &'a T {
        &grid[self.j][self.i]
    }

    pub fn get_direction_to(&self, other: Index) -> Direction {
        let offset = (other.i as i32 - self.i as i32, other.j as i32 - self.j as i32);

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

impl From<Position> for Index {
    fn from(pos: Position) -> Self {
        Index {
            i: pos.x as usize,
            j: pos.y as usize,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: i128,
    pub y: i128,
}

impl Position {
    pub fn advance_by(&self, dir: Direction, delta: i128) -> Self {
        match dir {
            Direction::North => Position {
                x: self.x,
                y: self.y - delta,
            },
            Direction::East => Position {
                x: self.x + delta,
                y: self.y,
            },
            Direction::West => Position {
                x: self.x - delta,
                y: self.y,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + delta,
            },
            Direction::None => *self,
        }
    }

    pub fn get_direction_to(&self, other: Position) -> Direction {
        let offset = (other.x - self.x, other.y - self.y);

        match offset {
            (0, 0) => Direction::None,
            (dx, 0) if dx > 0 => Direction::East,
            (dx, 0) if dx < 0 => Direction::West,
            (0, dy) if dy > 0 => Direction::South,
            (0, dy) if dy < 0 => Direction::North,
            _ => panic!("No clear direction"),
        }
    }

    pub fn von_neumann_neighbors(&self, depth: i128) -> Vec<Position> {
        Direction::compass()
            .into_iter()
            .map(|dir| self.advance_by(dir, depth))
            .collect()
    }

    pub fn wrap_modular(&self, size: Size) -> Index {
        let (mx, my) = (size.width as i128, size.height as i128);
        Index {
            i: (((self.x % mx) + mx) % mx) as usize,
            j: (((self.y % my) + my) % my) as usize,
        }
    }
}

impl From<Index> for Position {
    fn from(idx: Index) -> Self {
        Position {
            x: idx.i as i128,
            y: idx.j as i128,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn from_grid<T>(grid: &Grid<T>) -> Size {
        Size {
            height: grid.len(),
            width: grid[0].len(),
        }
    }
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
