use std::collections::HashSet;

use aoc_runner_derive::aoc;

fn solve(input: &[u8], length: usize) -> Option<usize> {
    input.windows(length).enumerate().find_map(|(i, window)| {
        let set = window.iter().collect::<HashSet<_>>();
        if set.len() == length {
            Some(i + length)
        } else {
            None
        }
    })
}

#[aoc(day6, part1)]
fn part1(input: &[u8]) -> Option<usize> {
    solve(input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &[u8]) -> Option<usize> {
    solve(input, 14)
}
