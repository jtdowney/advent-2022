use std::{collections::HashMap, iter};

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::eyre;
use itertools::iproduct;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    Finish, IResult,
};

#[derive(Debug, Default)]
struct Input {
    names: Vec<String>,
    flow_rates: Vec<u32>,
    connections: Vec<Vec<String>>,
}

fn parse_valve(input: &str) -> IResult<&str, (String, u32, Vec<String>)> {
    let tunnels = separated_list1(tag(", "), map(alpha1, String::from));

    map(
        tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            u32,
            alt((
                tag("; tunnel leads to valve "),
                tag("; tunnels lead to valves "),
            )),
            tunnels,
        )),
        |(_, name, _, flow_rate, _, connections)| (String::from(name), flow_rate, connections),
    )(input)
}

#[aoc_generator(day16)]
fn generator(input: &str) -> eyre::Result<Input> {
    input
        .lines()
        .map(|line| {
            parse_valve(line)
                .finish()
                .map_err(|e| eyre!("Error parsing {}: {}", line, e))
                .map(|(_, o)| o)
        })
        .try_fold(Input::default(), |mut acc, item| {
            let (name, flow_rate, connections) = item?;
            acc.names.push(name);
            acc.flow_rates.push(flow_rate);
            acc.connections.push(connections);
            Ok(acc)
        })
}

fn compute_distances(input: &Input) -> HashMap<(usize, usize), u8> {
    let length = input.names.len();
    let mut distances = input
        .names
        .iter()
        .enumerate()
        .flat_map(|(i, _)| {
            iter::once(((i, i), 0)).chain(input.connections[i].iter().map(move |connection| {
                let j = input.names.iter().position(|n| n == connection).unwrap();
                ((i, j), 1)
            }))
        })
        .collect::<HashMap<(usize, usize), u8>>();

    for (k, j, i) in iproduct!(0..length, 0..length, 0..length) {
        let current = distances.get(&(i, j)).copied().unwrap_or(u8::MAX);
        let tentative = distances
            .get(&(i, k))
            .copied()
            .unwrap_or(u8::MAX)
            .saturating_add(distances.get(&(k, j)).copied().unwrap_or(u8::MAX));
        distances.insert((i, j), current.min(tentative));
    }

    distances
}

fn search(
    start: usize,
    current: usize,
    rest: Vec<usize>,
    time: u8,
    distances: &HashMap<(usize, usize), u8>,
    flow_rates: &[u32],
    elephant: bool,
) -> u32 {
    rest.iter()
        .enumerate()
        .filter(|(_, &c)| distances[&(current, c)] < time)
        .flat_map(|(i, _)| {
            let mut results = vec![];
            let mut candidates = rest.clone();
            let next = candidates.swap_remove(i);
            let time_remaining = time - distances[&(current, next)] - 1;
            let next_rate = flow_rates[next] * time_remaining as u32;
            let future_rate = search(
                start,
                next,
                candidates.clone(),
                time_remaining,
                distances,
                flow_rates,
                elephant,
            );

            let answer = next_rate + future_rate;
            results.push(answer);

            if elephant {
                let answer = search(start, start, rest.clone(), 26, distances, flow_rates, false);
                results.push(answer);
            }

            results
        })
        .max()
        .unwrap_or_default()
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> u32 {
    let distances = compute_distances(input);
    let candidates = input
        .flow_rates
        .iter()
        .enumerate()
        .filter(|(_, &r)| r > 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let start = input.names.iter().position(|n| n == "AA").unwrap();
    search(
        start,
        start,
        candidates,
        30,
        &distances,
        &input.flow_rates,
        false,
    )
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> u32 {
    let distances = compute_distances(input);
    let candidates = input
        .flow_rates
        .iter()
        .enumerate()
        .filter(|(_, &r)| r > 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let start = input.names.iter().position(|n| n == "AA").unwrap();
    search(
        start,
        start,
        candidates,
        26,
        &distances,
        &input.flow_rates,
        true,
    )
}
