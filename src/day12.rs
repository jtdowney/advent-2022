use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    iter,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i32, i32);

fn distance((ax, ay): Point, (bx, by): Point) -> i32 {
    (ax - bx).abs() + (ay - by).abs()
}

#[derive(Clone, Debug, Default)]
struct Grid {
    start: Point,
    end: Point,
    map: HashMap<Point, u8>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Search {
    point: Point,
    score: u32,
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
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

fn find_shortest_path(grid: &Grid, start: Point) -> Option<usize> {
    let mut search = BinaryHeap::new();
    search.push(Search {
        point: start,
        score: distance(start, grid.end) as u32,
    });

    let mut path = HashMap::new();
    let mut scores = HashMap::new();
    scores.insert(start, 0);

    while let Some(current) = search.pop() {
        if current.point == grid.end {
            let steps = iter::successors(Some(&grid.end), |&point| path.get(point)).count() - 1;
            return Some(steps);
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
            let tentative_score = scores[&current.point] + 1;
            let neighbor_score = scores.get(&neighbor).copied().unwrap_or(u32::MAX);
            if tentative_score < neighbor_score {
                path.insert(neighbor, current.point);
                scores.insert(neighbor, tentative_score);

                let candidate = Search {
                    point: neighbor,
                    score: tentative_score + distance(neighbor, grid.end) as u32,
                };
                search.push(candidate);
            }
        }
    }

    None
}

#[aoc(day12, part1)]
fn part1(input: &Grid) -> Option<usize> {
    find_shortest_path(input, input.start)
}

#[aoc(day12, part2)]
fn part2(input: &Grid) -> Option<usize> {
    input
        .map
        .iter()
        .filter(|(_, &height)| height == b'a')
        .filter_map(|(&start, _)| find_shortest_path(input, start))
        .sorted()
        .next()
}
