use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

// 11:43
// 12:01

pub const DAY: Day = Day {
    day: 1,
    name: "Calorie Counting",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve::<1>(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve::<3>(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>, std::num::ParseIntError> {
    input
        .trim()
        .split("\n\n")
        .map(|g| g.trim().lines().map(str::parse).collect())
        .collect()
}

struct Top<T, const N: usize>([T; N]);
impl<T: Ord + Clone, const N: usize> Top<T, N> {
    fn add(&mut self, mut value: T) {
        for v in &mut self.0 {
            if &mut value > v {
                std::mem::swap(v, &mut value);
            }
        }
    }
}

fn solve<const N: usize>(elves: &[Vec<u32>]) -> u32 {
    let mut leaders = Top([0; N]);
    elves
        .iter()
        .map(|e| e.iter().sum())
        .for_each(|e| leaders.add(e));
    leaders.0.into_iter().sum()
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

        let data = parse(&data).unwrap();

        let expected = 24000;
        let actual = solve::<1>(&data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let data = parse(&data).unwrap();

        let expected = 45000;
        let actual = solve::<3>(&data);

        assert_eq!(expected, actual);
    }
}
