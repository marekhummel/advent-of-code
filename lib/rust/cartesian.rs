use core::panic;
use std::fmt::{Debug, Display};

use itertools::{iproduct, Itertools};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
    None,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::None => Direction::None,
        }
    }

    pub fn compass() -> [Self; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }

    pub fn turn(&self) -> [Self; 2] {
        [self.left(), self.right()]
    }

    pub fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::None => panic!(),
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::None => panic!(),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "north" | "U" | "N" | "^" => Ok(Direction::North),
            "east" | "R" | "E" | ">" => Ok(Direction::East),
            "south" | "D" | "S" | "v" => Ok(Direction::South),
            "west" | "L" | "W" | "<" => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value.to_string().as_str().try_into()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Index {
    pub i: usize,
    pub j: usize,
}

impl Index {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    pub fn from_tuple(tuple: (usize, usize)) -> Self {
        Self { i: tuple.0, j: tuple.1 }
    }

    pub fn advance(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Index::new(self.i, self.j - 1),
            Direction::East => Index::new(self.i + 1, self.j),
            Direction::West => Index::new(self.i - 1, self.j),
            Direction::South => Index::new(self.i, self.j + 1),
            Direction::None => *self,
        }
    }

    pub fn advance_check(&self, dir: Direction, size: Size) -> Option<Self> {
        match dir {
            Direction::North if self.j > 0 => Some(self.advance(dir)),
            Direction::East if self.i < size.width - 1 => Some(self.advance(dir)),
            Direction::South if self.j < size.height - 1 => Some(self.advance(dir)),
            Direction::West if self.i > 0 => Some(self.advance(dir)),
            Direction::None => Some(*self),
            _ => None,
        }
    }

    /// 4-Neighborhood, excluding diagonals
    pub fn von_neumann_neighbors(&self, size: Size) -> Vec<Index> {
        Direction::compass()
            .into_iter()
            .filter_map(|dir| self.advance_check(dir, size))
            .collect()
    }

    /// 8-Neighborhood, including diagonals
    pub fn moore_neighbors(&self, size: Size) -> Vec<Index> {
        Direction::compass()
            .into_iter()
            .flat_map(|dir| {
                [
                    self.advance_check(dir, size),
                    self.advance_check(dir, size)
                        .and_then(|next| next.advance_check(dir.left(), size)),
                ]
            })
            .flatten()
            .collect()
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

    pub fn dist(&self, other: &Index) -> usize {
        self.i.abs_diff(other.i) + self.j.abs_diff(other.j)
    }
}

impl From<Position> for Index {
    fn from(pos: Position) -> Self {
        Index::new(pos.x as usize, pos.y as usize)
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.j, self.i).cmp(&(other.j, other.i))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i128,
    pub y: i128,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}|{})", self.x, self.y)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl Position {
    pub fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }

    pub fn from_tuple(tuple: (i128, i128)) -> Self {
        Self { x: tuple.0, y: tuple.1 }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn advance_by(&self, dir: Direction, delta: i128) -> Self {
        match dir {
            Direction::North => Position::new(self.x, self.y - delta),
            Direction::East => Position::new(self.x + delta, self.y),
            Direction::South => Position::new(self.x, self.y + delta),
            Direction::West => Position::new(self.x - delta, self.y),
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

    pub fn moore_neighbors(&self) -> Vec<Position> {
        Direction::compass()
            .into_iter()
            .flat_map(|dir| {
                [
                    self.advance_by(dir, 1),
                    self.advance_by(dir, 1).advance_by(dir.left(), 1),
                ]
            })
            .collect()
    }

    pub fn wrap_modular(&self, size: Size) -> Index {
        let (mx, my) = (size.width as i128, size.height as i128);
        Index::new(
            (((self.x % mx) + mx) % mx) as usize,
            (((self.y % my) + my) % my) as usize,
        )
    }

    pub fn dist(&self, other: &Position) -> u128 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<Index> for Position {
    fn from(idx: Index) -> Self {
        Self {
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
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn square(size: usize) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    pub fn indices(&self) -> impl Iterator<Item = Index> {
        iproduct!(0..self.height, 0..self.width).map(|(j, i)| Index { i, j })
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub rows: Vec<Vec<T>>,
    pub size: Size,
}

impl<T> Grid<T> {
    pub fn empty(size: Size, default: T) -> Self
    where
        T: Clone,
    {
        Grid {
            rows: vec![vec![default; size.width]; size.height],
            size,
        }
    }

    pub fn new(rows: Vec<Vec<T>>) -> Self {
        let size = Size {
            width: rows[0].len(),
            height: rows.len(),
        };
        Grid { rows, size }
    }

    pub fn get(&self, idx: &Index) -> &T {
        &self.rows[idx.j][idx.i]
    }

    pub fn get_checked(&self, idx: &Index) -> Option<&T> {
        self.rows.get(idx.j).and_then(|r| r.get(idx.i))
    }

    pub fn get_mut(&mut self, idx: &Index) -> &mut T {
        &mut self.rows[idx.j][idx.i]
    }

    pub fn set(&mut self, idx: &Index, value: T) {
        self.rows[idx.j][idx.i] = value;
    }

    pub fn corners(&self) -> [Index; 4] {
        [
            Index::new(0, 0),
            Index::new(0, self.size.height - 1),
            Index::new(self.size.width - 1, 0),
            Index::new(self.size.width - 1, self.size.height - 1),
        ]
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.rows.iter().flatten()
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
        self.rows.iter_mut().flatten()
    }

    pub fn enumerate(&self) -> impl DoubleEndedIterator<Item = (Index, &T)> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(j, row)| row.iter().enumerate().map(move |(i, v)| (Index { i, j }, v)))
    }

    pub fn extend(&mut self, dir: Direction, value: T)
    where
        T: Clone,
    {
        match dir {
            Direction::North => {
                self.rows.insert(0, vec![value; self.size.width]);
                self.size.height += 1;
            }
            Direction::East => {
                self.rows.iter_mut().for_each(|r| r.push(value.clone()));
                self.size.width += 1
            }
            Direction::South => {
                self.rows.push(vec![value; self.size.width]);
                self.size.height += 1
            }
            Direction::West => {
                self.rows.iter_mut().for_each(|r| r.insert(0, value.clone()));
                self.size.width += 1
            }
            Direction::None => (),
        }
    }

    pub fn map_elements<R, F: Fn(&T) -> R>(&self, func: F) -> Grid<R> {
        Grid {
            rows: self
                .rows
                .iter()
                .map(|row| row.iter().map(&func).collect_vec())
                .collect_vec(),
            size: self.size,
        }
    }

    pub fn transpose(&self) -> Self
    where
        T: Copy,
    {
        Grid {
            rows: (0..self.rows[0].len())
                .map(|i| self.rows.iter().map(|inner| inner[i]).collect())
                .collect(),
            size: Size {
                width: self.size.height,
                height: self.size.width,
            },
        }
    }

    pub fn flip_vertical(&self) -> Self
    where
        T: Copy,
    {
        Grid {
            rows: self.rows.iter().map(|r| r.iter().rev().copied().collect()).collect(),
            size: self.size,
        }
    }

    pub fn rotate_left(&self) -> Self
    where
        T: Copy,
    {
        Grid {
            rows: (0..self.rows[0].len())
                .rev()
                .map(|i| self.rows.iter().map(|inner| inner[i]).collect())
                .collect(),
            size: Size {
                width: self.size.height,
                height: self.size.width,
            },
        }
    }

    /// Creates array of all 8 symmetries of this grid / square
    pub fn symmetry_group(&self) -> [Self; 8]
    where
        T: Copy + Debug,
    {
        let mut symmetries = Vec::new();
        let mut clone = self.clone();
        for _flip in [0, 1] {
            for _rotate in [0, 1, 2, 3] {
                symmetries.push(clone.clone());
                clone = clone.rotate_left();
            }
            clone = clone.flip_vertical();
        }

        symmetries.try_into().unwrap()
    }

    pub fn print<F: Fn(Index, &T) -> S, S: Display>(&self, display_fn: F) {
        for (j, row) in self.rows.iter().enumerate() {
            for (i, item) in row.iter().enumerate() {
                print!("{}", display_fn(Index { i, j }, item));
            }
            println!();
        }
        println!();
    }
}

/// Implementation of directions in hex grid, double-height coordinates (flat side up)
#[derive(Debug, Clone, Copy)]
pub enum HexDirection {
    North,
    NorthWest,
    NorthEast,
    South,
    SouthWest,
    SouthEast,
}

impl TryFrom<&str> for HexDirection {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "n" => Ok(HexDirection::North),
            "nw" => Ok(HexDirection::NorthWest),
            "ne" => Ok(HexDirection::NorthEast),
            "s" => Ok(HexDirection::South),
            "sw" => Ok(HexDirection::SouthWest),
            "se" => Ok(HexDirection::SouthEast),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexIndex {
    pub x: i128,
    pub y: i128,
}

// Use double-height coordinates
// https://www.redblobgames.com/grids/hexagons/
impl HexIndex {
    pub fn step(&self, dir: &HexDirection) -> HexIndex {
        match dir {
            HexDirection::North => HexIndex {
                x: self.x,
                y: self.y - 2,
            },
            HexDirection::NorthWest => HexIndex {
                x: self.x - 1,
                y: self.y - 1,
            },
            HexDirection::NorthEast => HexIndex {
                x: self.x + 1,
                y: self.y - 1,
            },
            HexDirection::South => HexIndex {
                x: self.x,
                y: self.y + 2,
            },
            HexDirection::SouthWest => HexIndex {
                x: self.x - 1,
                y: self.y + 1,
            },
            HexDirection::SouthEast => HexIndex {
                x: self.x + 1,
                y: self.y + 1,
            },
        }
    }

    pub fn neighbors(&self) -> [HexIndex; 6] {
        [
            self.step(&HexDirection::North),
            self.step(&HexDirection::NorthEast),
            self.step(&HexDirection::NorthWest),
            self.step(&HexDirection::South),
            self.step(&HexDirection::SouthEast),
            self.step(&HexDirection::SouthWest),
        ]
    }

    pub fn dist(&self, other: &HexIndex) -> u128 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx + dy.saturating_sub(dx) / 2
    }
}
