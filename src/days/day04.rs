use std::cmp::Ordering;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

// 11:35
// 12:00

pub const DAY: Day = Day {
    day: 4,
    name: "Camp Cleanup",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[
        ("Parse", run_parse),
        ("No Alloc Part 1", run_no_alloc_part1),
        ("No Alloc Part 2", run_no_alloc_part2),
    ],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn run_no_alloc_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(no_alloc_solve(input, no_alloc_part1_condition)))
}

fn run_no_alloc_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(no_alloc_solve(input, no_alloc_part2_condition)))
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u8,
    end: u8,
}

#[derive(Debug, Clone)]
struct Pair {
    elf1: Range,
    elf2: Range,
}

fn parse(input: &str) -> Result<Vec<Pair>, Report> {
    let parse_pair = |string: &str| -> Result<Range, Report> {
        let Some((start, end)) = string.split_once('-') else {
            return Err(eyre!("Invalid range definition"));
        };

        let start = start.parse()?;
        let end = end.parse()?;
        Ok(Range { start, end })
    };

    input
        .lines()
        .map(|line| {
            let Some((first, second)) = line.split_once(',') else {
                return Err( eyre!("Invalid pair definition"));
            };

            Ok(Pair {
                elf1: parse_pair(first)?,
                elf2: parse_pair(second)?,
            })
        })
        .collect()
}

fn part1(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|p| match p.elf1.start.cmp(&p.elf2.start) {
            Ordering::Less => p.elf2.end <= p.elf1.end,
            Ordering::Greater => p.elf1.end <= p.elf2.end,
            Ordering::Equal => true,
        })
        .count()
}

fn part2(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|p| match p.elf1.start.cmp(&p.elf2.start) {
            Ordering::Less => p.elf2.start <= p.elf1.end,
            Ordering::Equal => true,
            Ordering::Greater => p.elf1.start <= p.elf2.end,
        })
        .count()
}

fn no_alloc_part1_condition(numbers: [u8; 4]) -> bool {
    match numbers[0].cmp(&numbers[2]) {
        Ordering::Less => numbers[3] <= numbers[1],
        Ordering::Equal => true,
        Ordering::Greater => numbers[1] <= numbers[3],
    }
}

fn no_alloc_part2_condition(numbers: [u8; 4]) -> bool {
    match numbers[0].cmp(&numbers[2]) {
        Ordering::Less => numbers[2] <= numbers[1],
        Ordering::Equal => true,
        Ordering::Greater => numbers[0] <= numbers[3],
    }
}

fn no_alloc_solve(input: &str, nums: fn([u8; 4]) -> bool) -> u16 {
    let mut count = 0;

    let mut numbers = [0; 4];
    let mut numbers_idx = 0;
    for byte in input.bytes() {
        if byte == b'\n' {
            // We've hit the end of the line, do the check.
            count += nums(numbers) as u16;

            numbers_idx = 0;
            numbers = [0; 4];
        } else if let b',' | b'-' = byte {
            numbers_idx += 1;
        } else {
            // We're in a digit.
            numbers[numbers_idx] *= 10;
            numbers[numbers_idx] += byte - b'0';
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let pairs = parse(&data).unwrap();
        let expected = 2;
        let actual = part1(&pairs);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let pairs = parse(&data).unwrap();
        let expected = 4;
        let actual = part2(&pairs);

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_alloc_part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let expected = 2;
        let actual = no_alloc_solve(&data, no_alloc_part1_condition);

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_alloc_part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let expected = 4;
        let actual = no_alloc_solve(&data, no_alloc_part2_condition);

        assert_eq!(expected, actual);
    }
}
