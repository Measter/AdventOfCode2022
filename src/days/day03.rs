use aoc_lib::{misc::ArrChunks, Bench, BenchResult, Day, NoError, ParseResult};
use color_eyre::Report;

// 14:57
// 15:28

pub const DAY: Day = Day {
    day: 3,
    name: "Rucksack Reorganization",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data: Vec<_> = input.trim().lines().map(parse).collect();
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data: Vec<_> = input.trim().lines().map(parse).collect();
    b.bench(|| Ok::<_, NoError>(part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data: Vec<_> = input.lines().map(parse).collect();
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
struct Rucksack {
    slot1: u64,
    slot2: u64,
}

fn to_u64(slot: &str) -> u64 {
    slot.bytes()
        .map(|b| match b {
            b'a'..=b'z' => 1 << ((b - b'a') as u64),
            b'A'..=b'Z' => (1 << ((b - b'A') as u64)) << 26,
            _ => panic!("Invalid Character"),
        })
        .fold(0, |acc, bit| acc | bit)
}

fn parse(line: &str) -> Rucksack {
    let (slot1, slot2) = line.trim().split_at(line.len() / 2);

    Rucksack {
        slot1: to_u64(slot1),
        slot2: to_u64(slot2),
    }
}

fn part1(data: &[Rucksack]) -> u32 {
    data.iter()
        .map(|sack| {
            let shared = sack.slot1 & sack.slot2;
            shared.trailing_zeros() + 1
        })
        .sum()
}

fn part2(data: &[Rucksack]) -> u32 {
    ArrChunks::new(data)
        .map(|[a, b, c]| {
            let shared = (a.slot1 | a.slot2) & (b.slot1 | b.slot2) & (c.slot1 | c.slot2);
            shared.trailing_zeros() + 1
        })
        .sum()
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

        let parsed: Vec<_> = data.lines().map(parse).collect();

        let expected = 157;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed: Vec<_> = data.lines().map(parse).collect();

        let expected = 70;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
