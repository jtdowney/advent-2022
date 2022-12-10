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

enum Instruction {
    Noop,
    AddX(i32),
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

fn execute(instructions: &[Instruction]) -> impl Iterator<Item = i32> + '_ {
    instructions
        .iter()
        .scan(1, |state, instruction| match instruction {
            Instruction::Noop => Some(vec![*state]),
            Instruction::AddX(value) => {
                let x = *state;
                *state += value;
                Some(vec![x, x])
            }
        })
        .flatten()
}

#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> i32 {
    execute(input)
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, x)| (i + 1) as i32 * x)
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &[Instruction]) -> String {
    iproduct!((0..6), (0..40)).zip(execute(input)).fold(
        String::with_capacity(6 * 40 + 6),
        |mut acc, ((_, x), sprite)| {
            if x == 0 {
                acc.push('\n');
            }

            if ((sprite - 1)..=(sprite + 1)).contains(&x) {
                acc.push('#');
            } else {
                acc.push('.');
            }

            acc
        },
    )
}
