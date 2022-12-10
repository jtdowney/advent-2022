use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::bail;
use itertools::iproduct;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, space1},
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

#[derive(Copy, Clone, Debug)]
struct State {
    x: i32,
    cycle: usize,
}

impl Default for State {
    fn default() -> Self {
        State { x: 1, cycle: 0 }
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let parse_noop = map(tag("noop"), |_| Instruction::Noop);
    let parse_addx = map(preceded(tuple((tag("addx"), space1)), i32), |value| {
        Instruction::AddX(value)
    });

    alt((parse_noop, parse_addx))(input)
}

#[aoc_generator(day10)]
fn generator(input: &str) -> eyre::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| match parse_instruction(line) {
            Ok((_, i)) => Ok(i),
            Err(e) => bail!("error parsing{:?}: {}", line, e),
        })
        .collect()
}

fn execute(instructions: &[Instruction]) -> impl Iterator<Item = (usize, i32)> + '_ {
    instructions
        .iter()
        .scan(
            State::default(),
            |mut state, instruction| match instruction {
                Instruction::Noop => {
                    state.cycle += 1;
                    Some(vec![(state.cycle, state.x)])
                }
                Instruction::AddX(value) => {
                    let cycle = state.cycle;
                    let x = state.x;

                    state.cycle += 2;
                    state.x += value;

                    Some(vec![(cycle + 1, x), (cycle + 2, x)])
                }
            },
        )
        .flatten()
}

#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let output = execute(input).collect::<HashMap<_, _>>();

    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&i| output[&i] * i as i32)
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &[Instruction]) -> String {
    iproduct!((0..6), (0..40)).zip(execute(input)).fold(
        String::new(),
        |mut output, ((_, x), (_, sprite))| {
            if x == 0 {
                output.push('\n');
            }

            if sprite - 1 == x || sprite == x || sprite + 1 == x {
                output.push('#');
            } else {
                output.push('.');
            }

            output
        },
    )
}
