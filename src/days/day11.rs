use aoc_lib::{misc::Top, Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

// 11:48
// 12:56

pub const DAY: Day = Day {
    day: 11,
    name: "Monkey in the Middle",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve::<false>(data.clone())))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve::<true>(data.clone())))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
enum OpRhs {
    Literal(u64),
    Old,
}

impl OpRhs {
    fn rhs(self, old: u64) -> u64 {
        match self {
            OpRhs::Literal(v) => v,
            OpRhs::Old => old,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(OpRhs),
    Mul(OpRhs),
}

impl Operation {
    fn apply(self, rhs: u64) -> u64 {
        match self {
            Operation::Add(v) => rhs + v.rhs(rhs),
            Operation::Mul(v) => rhs * v.rhs(rhs),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_div: u64,
    paths: [usize; 2],
}

fn parse(input: &str) -> Result<Vec<Monkey>> {
    let mut monkeys = Vec::new();

    for monkey_segment in input.split("\n\n") {
        let mut lines = monkey_segment.lines();
        let _ = lines.next(); // Monkey ID

        let Some(items_str) = lines.next().and_then(|s| s.strip_prefix("  Starting items: ")) else {
            return Err(eyre!("Invalid monkey"));
        };
        let items: Vec<_> = items_str
            .split(", ")
            .map(str::parse::<u64>)
            .collect::<Result<_, _>>()?;

        let Some(operation) = lines.next().and_then(|s| {
            let s = s.strip_prefix("  Operation: new = old ")?;
            let mut parts = s.split(' ');
            let op = parts.next()?;
            let rhs = parts.next()?;

            let rhs = match rhs {
                "old" => OpRhs::Old,
                _ => OpRhs::Literal(rhs.parse().ok()?)
            };

            let op = match op {
                "*" => Operation::Mul(rhs),
                "+" => Operation::Add(rhs),
                _ => return None,
            };
            Some(op)
        }) else {
            return Err(eyre!("invalid monkey"));
        };

        let Some(test_div) = lines.next()
            .and_then(|s| s.strip_prefix("  Test: divisible by "))
            .and_then(|s| s.parse::<u64>().ok()) else {
            return Err(eyre!("Invalid monkey"));
        };

        let Some(true_path) = lines.next()
            .and_then(|s| s.strip_prefix("    If true: throw to monkey "))
            .and_then(|s| s.parse::<usize>().ok()) else {
                return Err(eyre!("Invalid monkey"));
        };

        let Some(false_path) = lines.next()
            .and_then(|s| s.strip_prefix("    If false: throw to monkey "))
            .and_then(|s| s.parse::<usize>().ok()) else {
                return Err(eyre!("Invalid monkey"));
        };

        let monkey = Monkey {
            items,
            operation,
            test_div,
            paths: [true_path, false_path],
        };

        monkeys.push(monkey);
    }

    Ok(monkeys)
}

fn solve<const PART2: bool>(mut monkeys: Vec<Monkey>) -> usize {
    let mut inspect_counts = vec![0; monkeys.len()];

    let rounds = if PART2 { 10000 } else { 20 };
    let lcm: u64 = monkeys.iter().map(|m| m.test_div).product();

    for _ in 0..rounds {
        for monkey_id in 0..monkeys.len() {
            let Monkey {
                operation,
                test_div,
                paths,
                ..
            } = monkeys[monkey_id];

            inspect_counts[monkey_id] += monkeys[monkey_id].items.len();

            for item_id in 0..monkeys[monkey_id].items.len() {
                let item = monkeys[monkey_id].items[item_id];
                let new_item = if PART2 {
                    operation.apply(item)
                } else {
                    operation.apply(item) / 3
                };
                let new_item = new_item % lcm;
                let target_monkey = paths[(new_item % test_div != 0) as usize];
                monkeys[target_monkey].items.push(new_item);
            }

            monkeys[monkey_id].items.clear();
        }
    }

    let mut top2 = Top([0; 2]);
    inspect_counts.into_iter().for_each(|c| top2.push(c));

    let [a, b] = top2.0;
    a * b
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
        let expected = 10605;
        let actual = solve::<false>(monkeys);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let monkeys = parse(&data).unwrap();
        let expected = 2_713_310_158;
        let actual = solve::<true>(monkeys);

        assert_eq!(expected, actual);
    }
}
