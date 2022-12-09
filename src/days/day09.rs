use std::collections::HashSet;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};
use derive_more::{Add, Sub};

pub const DAY: Day = Day {
    day: 9,
    name: "Rope Bridge",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1::<2>(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1::<10>(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_relative_position(self) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    dir: Direction,
    distance: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, Add, Sub)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Rope<const N: usize> {
    segments: [Position; N],
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            segments: [Position::default(); N],
        }
    }

    fn step(&mut self, dir: Direction) {
        self.segments[0] = self.segments[0] + dir.to_relative_position();

        for i in 0..N - 1 {
            let [ head, tail, .. ] = &mut self.segments[i..] else {
                panic!("wat");
            };

            let distance = *head - *tail;

            if distance.x.abs() > 1 || distance.y.abs() > 1 {
                let new_move = Position {
                    x: distance.x.min(1).max(-1),
                    y: distance.y.min(1).max(-1),
                };
                *tail = *tail + new_move;
            }
        }
    }

    fn tail(&mut self) -> Position {
        self.segments[N - 1]
    }
}

fn parse(input: &str) -> Result<Vec<Move>> {
    input
        .lines()
        .map(|l| {
            let Some((dir, distance)) = l.split_once(' ') else {
            return Err(eyre!("Invalid move"));
        };
            let distance = distance.parse()?;
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => return Err(eyre!("Invalid move")),
            };
            Ok(Move { dir, distance })
        })
        .collect()
}

fn part1<const N: usize>(moves: &[Move]) -> usize {
    let mut rope = Rope::<N>::new();
    let mut visited = HashSet::new();
    visited.insert(rope.tail());

    for mov in moves {
        (0..mov.distance).for_each(|_| {
            rope.step(mov.dir);
            visited.insert(rope.tail());
        });
    }

    visited.len()
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

        let moves = parse(&data).unwrap();
        let expected = 13;
        let actual = part1::<2>(&moves);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let moves = parse(&data).unwrap();
        let expected = 1;
        let actual = part1::<10>(&moves);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test_2() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part2, 1)
            .open()
            .unwrap();

        let moves = parse(&data).unwrap();
        let expected = 36;
        let actual = part1::<10>(&moves);

        assert_eq!(expected, actual);
    }
}
