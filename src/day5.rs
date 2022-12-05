use std::collections::{BTreeMap, HashMap};

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::ContextCompat;

#[derive(Copy, Clone, Debug)]
struct Instruction {
    count: usize,
    source: u32,
    destination: u32,
}

#[derive(Debug)]
struct Input {
    stacks: BTreeMap<u32, Vec<char>>,
    instructions: Vec<Instruction>,
}

#[aoc_generator(day5)]
fn generator(input: &str) -> eyre::Result<Input> {
    let mut parts = input.split("\n\n");

    let stack_part = parts.next().context("reading stacks")?;
    let mut lines = stack_part.lines().rev();
    let labels = lines
        .next()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| c.to_digit(10).map(|n| (i, n)))
                .collect::<HashMap<usize, u32>>()
        })
        .context("parsing labels")?;
    let stacks = lines.fold(BTreeMap::<u32, Vec<char>>::new(), |mut acc, line| {
        for (i, c) in line.chars().enumerate() {
            if c.is_whitespace() {
                continue;
            }

            match labels.get(&i) {
                Some(&s) => {
                    acc.entry(s).or_default().push(c);
                }
                None => continue,
            }
        }

        acc
    });

    let instruction_part = parts.next().context("reading instructions")?;
    let instructions = instruction_part
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let _ = parts.next()?;
            let count = parts.next().and_then(|m| m.parse().ok())?;
            let _ = parts.next()?;
            let source = parts.next().and_then(|m| m.parse().ok())?;
            let _ = parts.next()?;
            let destination = parts.next().and_then(|m| m.parse().ok())?;
            Some(Instruction {
                count,
                source,
                destination,
            })
        })
        .collect::<Option<Vec<_>>>()
        .context("parsing instructions")?;

    Ok(Input {
        stacks,
        instructions,
    })
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> eyre::Result<String> {
    let stacks = input.instructions.iter().try_fold(
        input.stacks.clone(),
        |mut stacks,
         &Instruction {
             count,
             source,
             destination,
         }| {
            let mut temp = vec![];
            let source_stack = stacks.get_mut(&source).context("getting source")?;
            for _ in 0..count {
                let value = source_stack.pop().context("popping stack")?;
                temp.push(value);
            }

            let destination_stack = stacks
                .get_mut(&destination)
                .context("getting destination")?;
            destination_stack.extend_from_slice(&temp);

            Ok::<_, eyre::Report>(stacks)
        },
    )?;

    let answer = stacks.values().filter_map(|stack| stack.last()).collect();
    Ok(answer)
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> eyre::Result<String> {
    let stacks = input.instructions.iter().try_fold(
        input.stacks.clone(),
        |mut stacks,
         &Instruction {
             count,
             source,
             destination,
         }| {
            let mut temp = vec![];
            let source_stack = stacks.get_mut(&source).context("getting source")?;
            for _ in 0..count {
                let value = source_stack.pop().context("popping stack")?;
                temp.push(value);
            }

            let destination_stack = stacks
                .get_mut(&destination)
                .context("getting destination")?;

            temp.reverse();
            destination_stack.extend_from_slice(&temp);

            Ok::<_, eyre::Report>(stacks)
        },
    )?;

    let answer = stacks.values().filter_map(|stack| stack.last()).collect();
    Ok(answer)
}
