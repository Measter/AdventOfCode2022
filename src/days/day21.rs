use std::{
    collections::HashMap,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

pub const DAY: Day = Day {
    day: 21,
    name: "Monkey Math",
    part_1: run_part1,
    part_2: None,
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(data.clone())))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
struct MonkeyId(usize);

#[derive(Clone, Copy)]
enum Operation {
    Literal(u64),
    Op {
        left: MonkeyId,
        right: MonkeyId,
        op: fn(u64, u64) -> u64,
    },
}

#[derive(Clone)]
struct Monkeys {
    root: MonkeyId,
    monkeys: Vec<Operation>,
}

impl Index<MonkeyId> for Monkeys {
    type Output = Operation;

    fn index(&self, index: MonkeyId) -> &Self::Output {
        &self.monkeys[index.0]
    }
}

impl IndexMut<MonkeyId> for Monkeys {
    fn index_mut(&mut self, index: MonkeyId) -> &mut Self::Output {
        &mut self.monkeys[index.0]
    }
}

fn parse(input: &str) -> Result<Monkeys> {
    let mut monkeys = Monkeys {
        root: MonkeyId(0),
        monkeys: Vec::new(),
    };
    let mut id_map = HashMap::new();

    for line in input.lines() {
        let Some((name, value)) = line.split_once(": ") else {
            return Err(eyre!("Invalid monkey"));
        };

        let id = *id_map.entry(name).or_insert_with(|| {
            let len = monkeys.monkeys.len();
            monkeys.monkeys.push(Operation::Literal(u64::MAX));
            MonkeyId(len)
        });

        if name == "root" {
            monkeys.root = id;
        }

        let op = if let Some((idx, op)) = value.match_indices(['+', '-', '*', '/']).next() {
            let left = &value[..idx];
            let right = &value[idx + 1..];

            let left = *id_map.entry(left.trim()).or_insert_with(|| {
                let len = monkeys.monkeys.len();
                monkeys.monkeys.push(Operation::Literal(u64::MAX));
                MonkeyId(len)
            });
            let right = *id_map.entry(right.trim()).or_insert_with(|| {
                let len = monkeys.monkeys.len();
                monkeys.monkeys.push(Operation::Literal(u64::MAX));
                MonkeyId(len)
            });

            let op = match op {
                "+" => Add::add,
                "-" => Sub::sub,
                "*" => Mul::mul,
                "/" => Div::div,
                _ => unreachable!(),
            };

            Operation::Op { left, right, op }
        } else {
            Operation::Literal(value.trim().parse()?)
        };

        monkeys[id] = op;
    }

    Ok(monkeys)
}

fn part1(mut monkeys: Monkeys) -> u64 {
    let mut src: Vec<usize> = (0..monkeys.monkeys.len()).collect();
    let mut dst = Vec::with_capacity(src.len());
    loop {
        for idx in src.drain(..) {
            let Operation::Op { left, right, op } = monkeys.monkeys[idx] else { continue };
            let Operation::Literal(left) = monkeys[left] else { 
                dst.push(idx); 
                continue;
            };
            let Operation::Literal(right) = monkeys[right] else { 
                dst.push(idx); 
                continue
             };

            monkeys.monkeys[idx] = Operation::Literal(op(left, right));
        }

        if let Operation::Literal(answer) = monkeys[monkeys.root] {
            return answer;
        }
        std::mem::swap(&mut src, &mut dst);
    }
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

        let monkeys = parse(&data).unwrap();
        let expected = 152;
        let actual = part1(monkeys);

        assert_eq!(expected, actual);
    }
}
