use std::iter::{once, repeat};

use itertools::Itertools;

use crate::days::day_12::cardinals::*;

#[derive(Debug, Default)]
pub struct Garden {
    tiles: Vec<Vec<(Option<u32>, char)>>,
    next_id: u32,
}

impl Garden {
    pub fn new(s: &str) -> Self {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        let mut tiles = vec![vec![(None, ' '); height]; width];

        s.lines().enumerate().for_each(|(y, xs)| {
            xs.chars()
                .enumerate()
                .for_each(|(x, c)| tiles[x][y] = (None, c));
        });

        let mut garden = Garden {
            tiles,
            ..Default::default()
        };

        for x in 0..width {
            for y in 0..height {
                if let Some((id, c)) = garden.get_tile(x, y) {
                    if id.is_none() {
                        let new_id = garden.get_new_id();
                        garden.set_tile_id(new_id, c, x, y);
                    }
                }
            }
        }

        garden
    }

    pub fn calculate_price(&self) -> usize {
        (0..self.next_id)
            .map(|region_id| self.get_region_borders(region_id) * self.get_region_size(region_id))
            .sum()
    }

    pub fn calculate_bulk_price(&self) -> usize {
        (0..self.next_id)
            .map(|region_id| self.get_region_sides(region_id) * self.get_region_size(region_id))
            .sum()
    }

    fn set_tile_id(&mut self, id: u32, c: char, x: usize, y: usize) {
        self.tiles[x][y] = (Some(id), c);

        for (x, y) in cardinals_iter(x, y) {
            if let Some((neighbour_id, neighbour_char)) = self.get_tile(x, y) {
                if neighbour_id.is_none() && neighbour_char == c {
                    self.set_tile_id(id, c, x, y);
                }
            }
        }
    }

    fn get_new_id(&mut self) -> u32 {
        let id = self.next_id;

        self.next_id += 1;

        id
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<(Option<u32>, char)> {
        self.tiles.get(x)?.get(y).map(|opt| *opt)
    }

    fn get_surrounding_tiles(&self, x: usize, y: usize) -> [Option<(Option<u32>, char)>; 4] {
        let [right, down, left, up] = cardinals(x, y);

        [
            right.and_then(|(x, y)| self.get_tile(x, y)),
            down.and_then(|(x, y)| self.get_tile(x, y)),
            left.and_then(|(x, y)| self.get_tile(x, y)),
            up.and_then(|(x, y)| self.get_tile(x, y)),
        ]
    }

    fn get_tile_borders(&self, id: u32, x: usize, y: usize) -> usize {
        self.get_surrounding_tiles(x, y)
            .into_iter()
            .filter(|neighbour_tile| {
                if let Some((neighbour_id, _)) = neighbour_tile {
                    neighbour_id != &Some(id)
                } else {
                    true
                }
            })
            .count()
    }

    fn get_region_borders(&self, region_id: u32) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .map(|(x, ys)| {
                ys.iter()
                    .enumerate()
                    .filter(|(_, (id, _))| id == &Some(region_id))
                    .map(move |(y, _)| self.get_tile_borders(region_id, x, y))
            })
            .flatten()
            .sum()
    }

    fn get_region_size(&self, region_id: u32) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|(id, _)| id == &Some(region_id))
            .count()
    }

    fn get_region_sides(&self, region_id: u32) -> usize {
        // Programmers with weak constitutions should avert their eyes
        fn count_sides(map: &Vec<Vec<(Option<u32>, char)>>, region_id: u32) -> usize {
            let padding = repeat((None, ' ')).take(map[0].len()).collect();

            once(&padding)
                .chain(map.iter())
                .chain(once(&padding))
                .tuple_windows()
                .map(|(col1, col2)| {
                    col1.iter().zip(col2).fold(
                        (0, &None, &None),
                        |(count, prev_left_id, prev_right_id), ((left_id, _), (right_id, _))| {
                            (
                                count
                                    + (left_id != right_id
                                        && ((right_id == &Some(region_id)
                                            && (prev_right_id != right_id
                                                || prev_left_id == right_id
                                                    && prev_right_id == right_id))
                                            || (left_id == &Some(region_id)
                                                && (prev_left_id != left_id
                                                    || prev_left_id == left_id
                                                        && prev_right_id == left_id))))
                                        as usize,
                                left_id,
                                right_id,
                            )
                        },
                    )
                })
                .map(|(count, _, _)| count)
                .sum()
        }

        let mut sides = count_sides(&self.tiles, region_id);

        // The above only gets the vertical sides, so here we just rotate the tilemap to get the horizontal ones as well
        let swapped: Vec<Vec<(Option<u32>, char)>> = (0..self.tiles[0].len())
            .map(|x| self.tiles.iter().map(|ys| ys[x].clone()).collect())
            .collect();

        sides += count_sides(&swapped, region_id);

        sides
    }
}
