use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::bail;
use nom::{
    branch::alt,
    character::complete::{char, space1, u32},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    direction: Direction,
    count: u32,
}

type Point = (i32, i32);

struct State<const N: usize> {
    positions: [Point; N],
    visited: HashSet<Point>,
}

impl<const N: usize> Default for State<N> {
    fn default() -> Self {
        let positions = [(0, 0); N];
        let visited = HashSet::new();
        Self { positions, visited }
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    map(
        alt((char('U'), char('D'), char('L'), char('R'))),
        |c| match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        },
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        separated_pair(parse_direction, space1, u32),
        |(direction, count)| Instruction { direction, count },
    )(input)
}

#[aoc_generator(day9)]
fn generator(input: &str) -> eyre::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| match parse_instruction(line) {
            Ok((_, i)) => Ok(i),
            Err(e) => bail!("error parsing{:?}: {}", line, e),
        })
        .collect()
}

fn neighbors((x, y): Point) -> impl Iterator<Item = Point> {
    [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]
    .iter()
    .map(move |&(dx, dy)| (x + dx, y + dy))
}

fn is_touching(left: Point, right: Point) -> bool {
    if left == right {
        return true;
    }

    neighbors(left).any(|n| n == right)
}

fn solve<const N: usize>(input: &[Instruction]) -> usize {
    let state: State<N> = input.iter().fold(
        State::default(),
        |mut state, &Instruction { direction, count }| {
            for _ in 0..count {
                let (hx, hy) = state.positions[0];
                state.positions[0] = match direction {
                    Direction::Up => (hx, hy + 1),
                    Direction::Down => (hx, hy - 1),
                    Direction::Left => (hx - 1, hy),
                    Direction::Right => (hx + 1, hy),
                };

                for i in 1..N {
                    let head @ (hx, hy) = state.positions[i - 1];
                    let tail = state.positions[i];

                    if is_touching(head, tail) {
                        continue;
                    }

                    let candidates = neighbors(tail)
                        .filter(|&nt| is_touching(nt, head))
                        .collect::<Vec<_>>();
                    let best = candidates
                        .iter()
                        .copied()
                        .find(|&(nx, ny)| nx == hx || ny == hy);
                    state.positions[i] = match best {
                        Some(p) => p,
                        None => candidates[0],
                    };
                }

                state.visited.insert(state.positions[N - 1]);
            }

            state
        },
    );

    state.visited.len()
}

#[aoc(day9, part1)]
fn part1(input: &[Instruction]) -> usize {
    solve::<2>(input)
}

#[aoc(day9, part2)]
fn part2(input: &[Instruction]) -> usize {
    solve::<10>(input)
}
