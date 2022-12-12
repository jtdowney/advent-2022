use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i32, i32);

#[derive(Clone, Debug, Default)]
struct Grid {
    start: Point,
    end: Point,
    map: HashMap<Point, u8>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Search {
    point: Point,
    distance: u32,
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

#[aoc_generator(day12)]
fn generator(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .fold(Grid::default(), |mut grid, (y, line)| {
            for (x, b) in line.bytes().enumerate() {
                let point = (x as i32, y as i32);

                if b == b'S' {
                    grid.start = point;
                } else if b == b'E' {
                    grid.end = point;
                }

                let b = match b {
                    b'S' => b'a',
                    b'E' => b'z',
                    b => b,
                };

                grid.map.insert(point, b);
            }

            grid
        })
}

fn find_shortest_path(grid: &Grid, start: Point) -> Option<u32> {
    let mut search = BinaryHeap::new();
    search.push(Search {
        point: start,
        distance: 0,
    });

    let mut distances = HashMap::new();
    distances.insert(start, 0);

    while let Some(current) = search.pop() {
        if current.point == grid.end {
            return distances.get(&grid.end).copied();
        }

        let (x, y) = current.point;
        let current_height = grid.map[&current.point];
        let neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(move |&(dx, dy)| (x + dx, y + dy))
            .filter(|n| grid.map.contains_key(n))
            .filter(|n| {
                let neighbor_height = grid.map[n];
                neighbor_height <= current_height + 1
            });
        for neighbor in neighbors {
            let neighbor_distance = current.distance + 1;
            let existing_distance = distances.get(&neighbor).copied().unwrap_or(u32::MAX);
            if neighbor_distance < existing_distance {
                distances.insert(neighbor, neighbor_distance);
                search.push(Search {
                    point: neighbor,
                    distance: neighbor_distance,
                });
            }
        }
    }

    None
}

#[aoc(day12, part1)]
fn part1(input: &Grid) -> Option<u32> {
    find_shortest_path(input, input.start)
}

#[aoc(day12, part2)]
fn part2(input: &Grid) -> Option<u32> {
    input
        .map
        .iter()
        .filter(|(_, &height)| height == b'a')
        .filter_map(|(&start, _)| find_shortest_path(input, start))
        .sorted()
        .next()
}
