use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

// 12:31
// 13:35

pub const DAY: Day = Day {
    day: 5,
    name: "Supply Stacks",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve::<false>(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve::<true>(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Supply {
    stack: Vec<Vec<u8>>,
    procedure: Vec<Step>,
}

fn parse_stack(stack_str: &str) -> Result<Vec<Vec<u8>>> {
    let num_lines = stack_str.lines().count();
    let Some(last_line) = stack_str
        .bytes()
        .rposition(|b| b == b'\n')
        .map(|idx| &stack_str[idx + 1..]) else {
            return Err(eyre!("Invalid stack definition"));
        };

    // Oh this is so brittle...
    let Some(num_stacks) = last_line.trim_end().chars().last().and_then(|c| c.to_digit(10)).map(|d| d as usize) else {
        return Err(eyre!("Invalid last column"));
    };

    let mut stacks = vec![Vec::new(); num_stacks];
    let line_len = last_line.len() + 1; // +1 for the newline.

    for line in 0..num_lines - 1 {
        for (id, stack) in stacks.iter_mut().enumerate() {
            let col_idx = id * 4 + 1;
            let idx = line * line_len + col_idx;
            let ch = stack_str.as_bytes()[idx];
            if ch != b' ' {
                stack.push(ch);
            }
        }
    }

    stacks.iter_mut().for_each(|s| s.reverse());

    Ok(stacks)
}

fn parse_procedure(procedure: &str) -> Result<Vec<Step>> {
    fn split_line(line: &str) -> Option<(&str, &str, &str)> {
        let (count, rest) = line[5..].split_once(' ')?;
        let (from, rest) = rest[5..].split_once(' ')?;
        let to = &rest[3..];

        Some((count, from, to))
    }

    let mut steps = Vec::new();

    for line in procedure.lines() {
        let Some((count, from, to)) = split_line(line) else {
            return Err(eyre!("Invalid step definition"));
        };

        steps.push(Step {
            count: count.parse()?,
            from: from.parse()?,
            to: to.parse()?,
        })
    }

    Ok(steps)
}

fn parse(input: &str) -> Result<Supply> {
    let Some((stack, procedure)) = input.split_once("\n\n") else {
        return Err(eyre!("Invalid input"));
    };

    let stack = parse_stack(stack)?;
    let procedure = parse_procedure(procedure)?;
    Ok(Supply { stack, procedure })
}

fn solve<const KEEP_ORDER: bool>(supply: &Supply) -> String {
    let mut stacks = supply.stack.clone();
    let total_len = stacks.iter().map(|s| s.len()).sum();
    stacks.iter_mut().for_each(|s| s.reserve(total_len));

    for step in &supply.procedure {
        if step.from == step.to {
            panic!("wut");
        }

        let from_idx = step.from - 1;
        let to_idx = step.to - 1;

        // This is ugly. wtf, mate.
        let (from_stack, to_stack) = if step.from > step.to {
            let (part1, part2) = stacks.split_at_mut(from_idx);
            (&mut part2[0], &mut part1[to_idx])
        } else {
            let (part1, part2) = stacks.split_at_mut(to_idx);
            (&mut part1[from_idx], &mut part2[0])
        };

        let start_idx = from_stack.len() - step.count;
        if !KEEP_ORDER {
            from_stack[start_idx..].reverse();
        }
        to_stack.extend_from_slice(&from_stack[start_idx..]);
        from_stack.truncate(start_idx);
    }

    stacks
        .into_iter()
        .filter_map(|s| s.last().copied())
        .map(|b| b as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn parse_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Parse, 1)
            .open()
            .unwrap();

        let expected = Supply {
            stack: vec![vec![b'Z', b'N'], vec![b'M', b'C', b'D'], vec![b'P']],
            procedure: vec![
                Step {
                    count: 1,
                    from: 2,
                    to: 1,
                },
                Step {
                    count: 3,
                    from: 1,
                    to: 3,
                },
                Step {
                    count: 2,
                    from: 2,
                    to: 1,
                },
                Step {
                    count: 1,
                    from: 1,
                    to: 2,
                },
            ],
        };
        let actual = parse(&data).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Parse, 1)
            .open()
            .unwrap();

        let supply = parse(&data).unwrap();
        let expected = "CMZ";
        let actual = solve::<false>(&supply);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Parse, 1)
            .open()
            .unwrap();

        let supply = parse(&data).unwrap();
        let expected = "MCD";
        let actual = solve::<true>(&supply);
        assert_eq!(expected, actual);
    }
}
