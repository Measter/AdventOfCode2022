use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

// 11:35
// 12:00

pub const DAY: Day = Day {
    day: 4,
    name: "Camp Cleanup",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
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
            std::cmp::Ordering::Less => p.elf2.end <= p.elf1.end,
            std::cmp::Ordering::Greater => p.elf1.end <= p.elf2.end,
            std::cmp::Ordering::Equal => true,
        })
        .count()
}

fn part2(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|p| match p.elf1.start.cmp(&p.elf2.start) {
            std::cmp::Ordering::Less => p.elf2.start <= p.elf1.end,
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Greater => p.elf1.start <= p.elf2.end,
        })
        .count()
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
}
