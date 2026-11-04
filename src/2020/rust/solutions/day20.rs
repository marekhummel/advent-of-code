use std::collections::HashSet;

use aoc_lib::cartesian::{Grid, Index, Size};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use num::integer::Roots;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edges {
    top: u16,
    left: u16,
    bot: u16,
    right: u16,
}

impl Edges {
    fn new(top: u16, left: u16, bot: u16, right: u16) -> Self {
        Self { top, left, bot, right }
    }
}

type Tile = (u64, [Edges; 8]); // A square has a symmetry group of size 8
type PlacedTile = (u64, Edges);

pub struct Solution20;
impl Solution20 {
    /// Parses each tile into its edges, once normal and once flipped.
    fn parse(input: ProblemInput) -> Vec<(Tile, Grid<bool>)> {
        let lines = input.lines();

        let tile_lines = lines.split(|l| l.is_empty());

        let tiles = tile_lines
            .filter(|lines| !lines.is_empty())
            .map(|lines| {
                let (header, grid_lines) = lines.split_first().unwrap();
                let tile_id = header
                    .trim_start_matches("Tile ")
                    .trim_end_matches(':')
                    .parse()
                    .unwrap();

                let rows = grid_lines
                    .iter()
                    .map(|l| l.bytes().map(|c| c == b'#').collect_vec())
                    .collect_vec();

                let grid = Grid::new(rows);

                let mut edges = [Edges::new(0, 0, 0, 0); 8];
                for (i, flip_rot_grid) in grid.symmetry_group().into_iter().enumerate() {
                    let flip_rot_grid_transposed = flip_rot_grid.transpose();

                    let edge_top = Self::edge_to_uint(&flip_rot_grid.rows[0]);
                    let edge_bot = Self::edge_to_uint(&flip_rot_grid.rows[flip_rot_grid.rows.len() - 1]);
                    let edge_left = Self::edge_to_uint(&flip_rot_grid_transposed.rows[0]);
                    let edge_right = Self::edge_to_uint(&flip_rot_grid_transposed.rows[flip_rot_grid.rows.len() - 1]);
                    edges[i] = Edges::new(edge_top, edge_left, edge_bot, edge_right);
                }

                ((tile_id, edges), grid)
            })
            .collect();

        tiles
    }

    fn edge_to_uint(edge: &[bool]) -> u16 {
        edge.iter().fold(0, |acc, b| (acc << 1) + (*b as u16))
    }

    fn assemble_tiles(image: &mut Vec<PlacedTile>, tiles: &mut Vec<Tile>, width: usize) -> Option<u64> {
        // Image filled, return success
        if tiles.is_empty() {
            let corner_idcs = [0, width - 1, width * (width - 1), width * width - 1];
            let corners = corner_idcs.map(|i| image[i].0).iter().product();
            return Some(corners);
        }

        // Try each available tile in each rotation
        for i in 0..tiles.len() {
            let (tile_id, symmetries) = tiles.remove(i);
            for edges in symmetries {
                // Check if tile fits
                if !image.is_empty() {
                    // Check left border
                    if image.len() % width != 0 {
                        let left_tile = image.last().unwrap();
                        if edges.left != left_tile.1.right {
                            continue;
                        }
                    }

                    // Check top border
                    if image.len() >= width {
                        let top_tile = image.iter().nth_back(width - 1).unwrap();
                        if edges.top != top_tile.1.bot {
                            continue;
                        }
                    }
                }

                // Set and recurse
                image.push((tile_id, edges));
                if let Some(corner_product) = Self::assemble_tiles(image, tiles, width) {
                    return Some(corner_product);
                }
                // Undo
                image.pop();
            }
            // Undo
            tiles.insert(i, (tile_id, symmetries));
        }

        None
    }

    fn build_image(image_tiles: &[PlacedTile], tile_grids: &[(Tile, Grid<bool>)]) -> Grid<bool> {
        let tiled_size = image_tiles.len().sqrt();
        let cropped_tile_size = tile_grids[0].1.size.width - 2;
        let mut image = Grid::empty(Size::square(cropped_tile_size * tiled_size), false);

        for (idx, (tile_id, flip_rot)) in image_tiles.iter().enumerate() {
            // Find actual grid and the flip and rotation
            let ((_, edges), tile_grid) = tile_grids.iter().find(|((tg_id, _), _)| tile_id == tg_id).unwrap();
            let flip_rot_id = edges.iter().position(|e| e == flip_rot).unwrap();

            // Apply transformations to have same grid as was placed when assembling it
            let grid = &tile_grid.symmetry_group()[flip_rot_id];

            // Compute tile coordinates
            let ti = idx % tiled_size;
            let tj = idx / tiled_size;

            // Set grid in image
            for dj in 1..=cropped_tile_size {
                for di in 1..=cropped_tile_size {
                    let image_idx = Index::new(ti * cropped_tile_size + di - 1, tj * cropped_tile_size + dj - 1);
                    let tile_idx = Index::new(di, dj);
                    image.set(&image_idx, *grid.get(&tile_idx));
                }
            }
        }

        image
    }

    fn create_seamonster() -> (Vec<Index>, Size) {
        let seamonster = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";
        let seamonster_indices = seamonster
            .split('\n')
            .enumerate()
            .flat_map(|(j, row)| {
                row.bytes()
                    .enumerate()
                    .filter_map(move |(i, c)| (c == b'#').then_some(Index { i, j }))
            })
            .collect_vec();

        let width = seamonster_indices.iter().map(|idx| idx.i).max().unwrap() + 1;
        let height = seamonster_indices.iter().map(|idx| idx.j).max().unwrap() + 1;

        (seamonster_indices, Size::new(width, height))
    }

    fn find_seamonster(image: &Grid<bool>, seamonster: &[Index], seamonster_size: &Size) -> HashSet<Index> {
        let mut seamonster_tiles = HashSet::new();
        for j in 0..image.size.height - seamonster_size.height {
            for i in 0..image.size.width - seamonster_size.width {
                let idcs = seamonster
                    .iter()
                    .map(|offset| Index::new(i + offset.i, j + offset.j))
                    .collect_vec();
                if idcs.iter().all(|idx| *image.get(idx)) {
                    seamonster_tiles.extend(idcs);
                }
            }
        }

        seamonster_tiles
    }
}

impl Solution for Solution20 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut tiles = Self::parse(input).into_iter().map(|(tile, _)| tile).collect_vec();
        let width = tiles.len().sqrt();
        let solution = Self::assemble_tiles(&mut vec![], &mut tiles, width).unwrap();

        solution.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let tile_grids = Self::parse(input);
        let mut tiles = tile_grids.iter().map(|(tile, _)| *tile).collect_vec();
        let mut image_tiles = vec![];
        let width = tiles.len().sqrt();
        let _ = Self::assemble_tiles(&mut image_tiles, &mut tiles, width).unwrap();

        let image = Self::build_image(&image_tiles, &tile_grids);
        let (seamonster_idcs, seamonster_size) = Self::create_seamonster();

        for flip_rot_image in image.symmetry_group() {
            let seamonster_tiles = Self::find_seamonster(&flip_rot_image, &seamonster_idcs, &seamonster_size);
            if !seamonster_tiles.is_empty() {
                let roughness = flip_rot_image
                    .enumerate()
                    .filter(|(idx, water)| **water && !seamonster_tiles.contains(idx))
                    .count();
                return roughness.to_result();
            }
        }

        unreachable!()
    }
}
