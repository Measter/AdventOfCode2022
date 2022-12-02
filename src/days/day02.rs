use aoc_lib::{misc::ArrChunks, Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

pub const DAY: Day = Day {
    day: 2,
    name: "Rock Paper Scissors",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[
        ("Parse Part 1", run_parse_part_1),
        ("Parse Part 2", run_parse_part_2),
        ("Part 1 Fast", run_part1_fast),
        ("Part 2 Fast", run_part2_fast),
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

fn run_part1_fast(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(part1_no_alloc(input)))
}

fn run_part2_fast(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(part2_no_alloc(input)))
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

const ROCK_X: u32 = u32::from_le_bytes(*b"A X\n");
const ROCK_Y: u32 = u32::from_le_bytes(*b"A Y\n");
const ROCK_Z: u32 = u32::from_le_bytes(*b"A Z\n");
const PAPER_X: u32 = u32::from_le_bytes(*b"B X\n");
const PAPER_Y: u32 = u32::from_le_bytes(*b"B Y\n");
const PAPER_Z: u32 = u32::from_le_bytes(*b"B Z\n");
const SCISSORS_X: u32 = u32::from_le_bytes(*b"C X\n");
const SCISSORS_Y: u32 = u32::from_le_bytes(*b"C Y\n");
const SCISSORS_Z: u32 = u32::from_le_bytes(*b"C Z\n");

fn part1_no_alloc(input: &str) -> u16 {
    assert!(input.len() % 4 == 0);

    let mut total_score = 0;
    for chunk in ArrChunks::new(input.as_bytes()) {
        let as_u32 = u32::from_ne_bytes(*chunk);

        let match_score = match as_u32 {
            ROCK_X | PAPER_Y | SCISSORS_Z => 3,
            ROCK_Y | PAPER_Z | SCISSORS_X => 6,
            ROCK_Z | PAPER_X | SCISSORS_Y => 0,
            _ => unreachable!(),
        };

        let hand_score = (chunk[2] - b'X' + 1) as u16;

        total_score += hand_score + match_score;
    }

    total_score
}

fn part2_no_alloc(input: &str) -> u16 {
    assert!(input.len() % 4 == 0);

    let mut total_score = 0;
    for chunk in ArrChunks::new(input.as_bytes()) {
        let as_u32 = u32::from_ne_bytes(*chunk);

        let match_score = ((chunk[2] - b'X') * 3) as u16;

        let hand_score = match as_u32 {
            ROCK_X | PAPER_Z | SCISSORS_Y => 3,
            ROCK_Y | PAPER_X | SCISSORS_Z => 1,
            ROCK_Z | PAPER_Y | SCISSORS_X => 2,
            _ => unreachable!(),
        };

        total_score += hand_score + match_score;
    }

    total_score
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

    #[test]
    fn part1_fast() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let hand = part1_no_alloc(&data);
        let expected_scores = 15;

        assert_eq!(expected_scores, hand);
    }

    #[test]
    fn part2_fast() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let hand = part2_no_alloc(&data);
        let expected_scores = 12;

        assert_eq!(expected_scores, hand);
    }
}
