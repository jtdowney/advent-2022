use aoc_runner_derive::{aoc, aoc_generator};
use eyre::ContextCompat;

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Play {
    fn from(play: char) -> Self {
        match play {
            'A' | 'X' => Play::Rock,
            'B' | 'Y' => Play::Paper,
            'C' | 'Z' => Play::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Play {
    fn play(self, other: Play) -> Outcome {
        match (self, other) {
            (Play::Rock, Play::Rock) => Outcome::Draw,
            (Play::Rock, Play::Paper) => Outcome::Loss,
            (Play::Rock, Play::Scissors) => Outcome::Win,
            (Play::Paper, Play::Rock) => Outcome::Win,
            (Play::Paper, Play::Paper) => Outcome::Draw,
            (Play::Paper, Play::Scissors) => Outcome::Loss,
            (Play::Scissors, Play::Rock) => Outcome::Loss,
            (Play::Scissors, Play::Paper) => Outcome::Win,
            (Play::Scissors, Play::Scissors) => Outcome::Draw,
        }
    }

    fn score(self) -> usize {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn find_win(self) -> Play {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }

    fn find_loss(self) -> Play {
        match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }
}

#[aoc_generator(day2)]
fn generator(input: &str) -> eyre::Result<Vec<(char, char)>> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let left = chars.next().context("getting left")?;
            let right = chars.nth(1).context("getting right")?;
            Ok((left, right))
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(char, char)]) -> usize {
    input
        .iter()
        .map(|&(opponent, you)| {
            let opponent = Play::from(opponent);
            let you = Play::from(you);
            let outcome = you.play(opponent);
            you.score() + outcome.score()
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(char, char)]) -> usize {
    input
        .iter()
        .map(|&(opponent, you)| {
            let opponent = Play::from(opponent);
            let you = match you {
                'X' => opponent.find_loss(),
                'Y' => opponent,
                'Z' => opponent.find_win(),
                _ => unreachable!(),
            };

            let outcome = you.play(opponent);
            you.score() + outcome.score()
        })
        .sum()
}
