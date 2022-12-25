use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

pub const DAY: Day = Day {
    day: 25,
    name: "Full of Hot Air",
    part_1: run_part1,
    part_2: None,
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn from_snafu(num_str: &str) -> Result<i64> {
    let digits = num_str.trim().bytes().map(|b| match b {
        b'0'..=b'2' => Ok((b - b'0') as i64),
        b'-' => Ok(-1),
        b'=' => Ok(-2),
        _ => Err(eyre!("Invalid character")),
    });

    (0..)
        .map(|b| i64::pow(5, b as u32))
        .zip(digits.rev())
        .map(|(mul, digit)| digit.map(|d| d * mul))
        .sum()
}

fn to_snafu(mut number: i64, buf: &mut [u8; 64]) -> &str {
    let mut start = 64;

    for byte in buf.iter_mut().rev() {
        let digit = number % 5;
        let mut carry = false;
        *byte = match digit {
            0..=2 => b'0' + digit as u8,
            3 => {
                carry = true;
                b'='
            }
            4 => {
                carry = true;
                b'-'
            }
            _ => unreachable!(),
        };

        number /= 5;
        if carry {
            number += 1;
        }
        start -= 1;
        if number == 0 {
            break;
        }
    }

    // SAFETY: We only put ASCII characters in the buffer.
    unsafe { std::str::from_utf8_unchecked(&buf[start..]) }
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input.lines().map(str::trim).map(from_snafu).collect()
}

fn part1(data: &[i64]) -> String {
    let mut buffer = [0; 64];
    let number = data.iter().sum();
    to_snafu(number, &mut buffer).to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn from_snafu_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Parse, 1)
            .open()
            .unwrap();

        let pairs = data.lines().map(|l| {
            let Some((decimal, snafu)) = l.trim().split_once(' ') else { panic!("Invalid input") };
            (decimal.parse::<i64>().unwrap(), snafu.trim())
        });

        for (decimal, snafu) in pairs {
            let actual = from_snafu(snafu).unwrap();
            assert_eq!(decimal, actual, "{snafu}");
        }
    }

    #[test]
    fn to_snafu_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Parse, 1)
            .open()
            .unwrap();

        let pairs = data.lines().map(|l| {
            let Some((decimal, snafu)) = l.trim().split_once(' ') else { panic!("Invalid input") };
            (decimal.parse::<i64>().unwrap(), snafu.trim())
        });

        for (decimal, snafu) in pairs {
            let mut buffer = [0; 64];
            let actual = to_snafu(decimal, &mut buffer);
            assert_eq!(snafu, actual, "{decimal}");
        }
    }
}
