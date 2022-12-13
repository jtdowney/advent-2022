use std::{cmp::Ordering, collections::HashSet, iter};

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::bail;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u32},
    combinator::{complete, map},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Value {
    Literal(u32),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Literal(lhs), Value::Literal(rhs)) => lhs.cmp(rhs),
            (Value::List(lhs), Value::List(rhs)) => {
                for (l, r) in iter::zip(lhs, rhs) {
                    match l.cmp(r) {
                        Ordering::Equal => continue,
                        o => return o,
                    }
                }

                lhs.len().cmp(&rhs.len())
            }
            (Value::Literal(lhs), Value::List(_)) => {
                let new_lhs = Value::List(vec![Value::Literal(*lhs)]);
                new_lhs.cmp(other)
            }
            (Value::List(_), Value::Literal(rhs)) => {
                let new_rhs = Value::List(vec![Value::Literal(*rhs)]);
                self.cmp(&new_rhs)
            }
        }
    }
}

fn parse_literal(input: &str) -> IResult<&str, Value> {
    map(u32, Value::Literal)(input)
}

fn parse_list(input: &str) -> IResult<&str, Value> {
    map(
        delimited(
            tag("["),
            separated_list0(tag(","), alt((parse_literal, parse_list))),
            tag("]"),
        ),
        Value::List,
    )(input)
}

fn parse_pair(input: &str) -> IResult<&str, (Value, Value)> {
    separated_pair(parse_list, line_ending, parse_list)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Value, Value)>> {
    complete(separated_list1(
        tuple((line_ending, line_ending)),
        parse_pair,
    ))(input)
}

#[aoc_generator(day13)]
fn generator(input: &str) -> eyre::Result<Vec<(Value, Value)>> {
    match parse_input(input) {
        Ok((_, i)) => Ok(i),
        Err(e) => bail!("error parsing: {}", e),
    }
}

#[aoc(day13, part1)]
fn part1(input: &[(Value, Value)]) -> usize {
    input
        .iter()
        .zip(1..)
        .filter(|((left, right), _)| left.cmp(right) == Ordering::Less)
        .map(|(_, i)| i)
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[(Value, Value)]) -> usize {
    let mut dividers = HashSet::new();
    dividers.insert(Value::List(vec![Value::List(vec![Value::Literal(2)])]));
    dividers.insert(Value::List(vec![Value::List(vec![Value::Literal(6)])]));

    let input = input.to_vec();
    let (left, right): (Vec<Value>, Vec<Value>) = input.into_iter().unzip();
    let mut packets = left
        .into_iter()
        .chain(right)
        .chain(dividers.clone())
        .collect::<Vec<_>>();

    packets.sort();
    packets
        .iter()
        .zip(1..)
        .filter(|(packet, _)| dividers.contains(packet))
        .map(|(_, i)| i)
        .product()
}
