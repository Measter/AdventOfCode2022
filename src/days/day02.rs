use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

pub const DAY: Day = Day {
    day: 2,
    name: "Rock Paper Scissors",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[
        ("Parse Part 1", run_parse_part_1),
        ("Parse Part 2", run_parse_part_2),
    ],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse_part1(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse_part2(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_parse_part_1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse_part1(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn run_parse_part_2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse_part2(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Play {
    player_one: Hand,
    player_two: Hand,
}

impl Play {
    fn score(&self) -> u32 {
        use Hand::*;
        let match_score = match (self.player_one, self.player_two) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
        };

        match_score + self.player_two.score()
    }
}

fn parse_part1(input: &str) -> Result<Vec<Play>> {
    let mut plays = Vec::new();

    for line in input.lines() {
        let Some((a, b)) = line.trim().split_once(' ') else {return Err(eyre!("Invalid character"))};

        use Hand::*;
        let player_one = match a {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => return Err(eyre!("Invalid character")),
        };

        let player_two = match b {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => return Err(eyre!("Invalid character")),
        };

        plays.push(Play {
            player_one,
            player_two,
        });
    }

    Ok(plays)
}

fn parse_part2(input: &str) -> Result<Vec<Play>> {
    let mut plays = Vec::new();

    for line in input.lines() {
        let Some((a, b)) = line.trim().split_once(' ') else {return Err(eyre!("Invalid character"))};

        use Hand::*;
        let player_one = match a {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => return Err(eyre!("Invalid character")),
        };

        let player_two = match (player_one, b) {
            (Rock, "X") => Scissors,
            (Paper, "X") => Rock,
            (Scissors, "X") => Paper,

            (_, "Y") => player_one,

            (Rock, "Z") => Paper,
            (Paper, "Z") => Scissors,
            (Scissors, "Z") => Rock,
            _ => return Err(eyre!("Invalid character")),
        };

        plays.push(Play {
            player_one,
            player_two,
        });
    }

    Ok(plays)
}

fn part1(plays: &[Play]) -> u32 {
    plays.iter().map(Play::score).sum()
}

#[cfg(test)]
mod day01_tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let hand = parse_part1(data.trim()).unwrap();
        let expected_scores = [8, 1, 6];

        for (idx, (hand, expected)) in hand.iter().zip(expected_scores).enumerate() {
            let actual = hand.score();
            assert_eq!(expected, actual, "{}", idx);
        }
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let hand = parse_part2(data.trim()).unwrap();
        let expected_scores = [4, 1, 7];

        for (idx, (hand, expected)) in hand.iter().zip(expected_scores).enumerate() {
            let actual = hand.score();
            assert_eq!(expected, actual, "{}", idx);
        }
    }
}
