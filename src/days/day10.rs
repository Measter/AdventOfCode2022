use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

pub const DAY: Day = Day {
    day: 10,
    name: "Cathode-Ray Tube",
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
    b.bench_alt(|| Ok::<_, NoError>(part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NoOp,
    AddX(i16),
}

impl Instruction {
    fn cycle_len(self) -> u8 {
        match self {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .map(|l| {
            if l == "noop" {
                Ok(Instruction::NoOp)
            } else if let Some(("addx", val)) = l.split_once(' ') {
                Ok(Instruction::AddX(val.parse()?))
            } else {
                Err(eyre!("Invalid instruction"))
            }
        })
        .collect()
}

fn part1(instrs: &[Instruction]) -> i16 {
    let mut cycle = 0;
    let mut ans = 0;

    let mut instrs = instrs.iter().copied();
    let mut cur_instr = instrs.next().unwrap();
    let mut instr_cycle = cur_instr.cycle_len();
    let mut x = 1;

    loop {
        cycle += 1;
        instr_cycle -= 1;
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => ans += x * cycle,
            220.. => break,
            _ => {}
        }

        if instr_cycle == 0 {
            if let Instruction::AddX(val) = cur_instr {
                x += val;
            }
            match instrs.next() {
                Some(i) => {
                    cur_instr = i;
                    instr_cycle = cur_instr.cycle_len();
                }
                None => break,
            }
        }
    }

    ans
}

fn part2(instrs: &[Instruction]) -> String {
    let mut cycle: i16 = 0;
    let mut ans = String::with_capacity(40 * 7);

    let mut instrs = instrs.iter().copied();
    let mut cur_instr = instrs.next().unwrap();
    let mut instr_cycle = cur_instr.cycle_len();
    let mut x = 1;

    loop {
        if (cycle % 40) == 0 {
            ans.push('\n');
        }
        if ((cycle % 40) - x).abs() <= 1 {
            ans.push('#');
        } else {
            ans.push('.');
        }

        cycle += 1;
        instr_cycle -= 1;

        if instr_cycle == 0 {
            if let Instruction::AddX(val) = cur_instr {
                x += val;
            }

            let Some(next_instr) = instrs.next()  else {
                break;
            };
            cur_instr = next_instr;
            instr_cycle = cur_instr.cycle_len();
        }
    }

    ans
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

        let instrs = parse(&data).unwrap();
        let expected = 13140;
        let actual = part1(&instrs);

        assert_eq!(expected, actual);
    }
}
