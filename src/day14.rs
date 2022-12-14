use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::eyre;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i32, line_ending},
    combinator::complete,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};

type Point = (i32, i32);
type Grid = HashSet<Point>;

const DROP_POINT: Point = (500, 0);

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    let parse_point = separated_pair(i32, char(','), i32);
    let parse_line = separated_list1(tag(" -> "), parse_point);
    complete(separated_list1(line_ending, parse_line))(input)
}

#[aoc_generator(day14)]
fn generator(input: &str) -> eyre::Result<Grid> {
    let lines = parse_input(input)
        .finish()
        .map_err(|e| eyre!("error parsing: {}", e))
        .map(|(_, o)| o)?;
    let grid = lines.into_iter().fold(Grid::default(), |mut grid, line| {
        let points = line.into_iter().tuple_windows();
        for ((lx, ly), (rx, ry)) in points {
            let sx = lx.min(rx);
            let ex = lx.max(rx);
            let sy = ly.min(ry);
            let ey = ly.max(ry);

            for y in sy..=ey {
                for x in sx..=ex {
                    grid.insert((x, y));
                }
            }
        }

        grid
    });

    Ok(grid)
}

fn simulate<P>(mut grid: Grid, goal: P) -> usize
where
    P: Fn(Point) -> bool,
{
    let maxy = grid.iter().map(|&(_, y)| y).max().unwrap();
    let floor = maxy + 2;

    for round in 0.. {
        let mut position = DROP_POINT;
        loop {
            let (px, py) = position;
            let next_position = [(0, 1), (-1, 1), (1, 1)]
                .iter()
                .map(|(dx, dy)| (dx + px, dy + py))
                .take_while(|&(_, y)| y < floor)
                .find(|p| !grid.contains(p));
            if let Some(next) = next_position {
                position = next;
            } else {
                if goal(position) {
                    return round;
                }

                grid.insert(position);
                break;
            }
        }
    }

    unreachable!()
}

#[aoc(day14, part1)]
fn part1(input: &Grid) -> usize {
    let grid = input.clone();
    let goal = grid.iter().map(|&(_, y)| y).max().unwrap();
    simulate(grid, |(_, y)| y >= goal)
}

#[aoc(day14, part2)]
fn part2(input: &Grid) -> usize {
    let grid = input.clone();
    simulate(grid, |point| point == DROP_POINT) + 1
}
