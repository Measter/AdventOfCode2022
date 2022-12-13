use std::{cmp::Ordering, iter::Peekable};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

// 12:25
// 15:02

pub const DAY: Day = Day {
    day: 13,
    name: "Distress Signal",
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

#[derive(Debug, Clone, Eq)]
enum PacketContent {
    Integer(u8),
    List(Vec<PacketContent>),
}

impl PartialEq for PacketContent {
    fn eq(&self, other: &Self) -> bool {
        use PacketContent::*;
        match (self, other) {
            (Integer(l0), Integer(r0)) => l0 == r0,
            (List(l0), List(r0)) => l0 == r0,

            (Integer(_), List(list)) => [self.clone()].as_slice() == list,
            (List(list), Integer(_)) => list == [other.clone()].as_slice(),
        }
    }
}

impl Ord for PacketContent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use PacketContent::*;
        match (self, other) {
            (Integer(left), Integer(right)) => left.cmp(right),

            (List(left), List(right)) => left
                .iter()
                .zip(right)
                .map(|(l, r)| l.cmp(r))
                .fold(Ordering::Equal, Ordering::then)
                .then(left.len().cmp(&right.len())),

            (Integer(_), List(right)) if right.is_empty() => Ordering::Greater,
            (Integer(_), List(right)) => [self]
                .into_iter()
                .zip(right)
                .map(|(l, r)| l.cmp(r))
                .fold(Ordering::Equal, Ordering::then)
                .then(1.cmp(&right.len())),

            (List(list), Integer(_)) if list.is_empty() => Ordering::Less,
            (List(left), Integer(_)) => left
                .iter()
                .zip([other])
                .map(|(l, r)| l.cmp(r))
                .fold(Ordering::Equal, Ordering::then)
                .then(left.len().cmp(&1)),
        }
    }
}

impl PartialOrd for PacketContent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct PacketPair {
    left: PacketContent,
    right: PacketContent,
}

fn parse_item(
    packet: &str,
    chars: &mut Peekable<impl Iterator<Item = (usize, u8)>>,
) -> Result<PacketContent> {
    match chars.next() {
        // Our item is a list.
        Some((idx, b'[')) => {
            let start = idx + 1;
            let mut end_idx = start + 1;
            let mut depth = 1;
            // Search for matching bracket.
            for (idx, ch) in chars.by_ref() {
                if ch == b'[' {
                    depth += 1;
                } else if ch == b']' {
                    depth -= 1;
                }
                if depth == 0 {
                    end_idx = idx;
                    break;
                }
            }

            // Now we know where the end of our list is, we substring only that part of our
            // input, not including surrounding brackets.
            let mut list = Vec::new();
            let item_str = &packet[start..end_idx];
            let mut item_iter = item_str.bytes().enumerate().peekable();

            // Parse items out of our substring.
            while item_iter.peek().is_some() {
                let next_item = parse_item(item_str, &mut item_iter)?;
                list.push(next_item);

                match item_iter.peek() {
                    None => {}
                    Some((_, b',')) => {
                        item_iter.next();
                    }
                    Some((_, ch)) => {
                        return Err(eyre!("Unexpected character: `{ch}`"));
                    }
                }
            }

            Ok(PacketContent::List(list))
        }
        Some((idx, b'0'..=b'9')) => {
            let mut end_idx = idx + 1;
            while let Some((_, b'0'..=b'9')) = chars.peek() {
                end_idx += 1;
                chars.next();
            }

            let num_str = &packet[idx..end_idx];
            Ok(PacketContent::Integer(num_str.parse()?))
        }

        Some((_, ch)) => Err(eyre!("Unexpected character: `{ch}`")),
        _ => Err(eyre!("Unexpected EOL")),
    }
}

fn parse(input: &str) -> Result<Vec<PacketPair>> {
    let mut pairs = Vec::new();

    for line_pair in input.trim().split("\n\n") {
        let Some((left, right)) = line_pair.split_once('\n') else {
            return Err(eyre!("Invalid packet"));
        };

        let left = parse_item(left, &mut left.bytes().enumerate().peekable())?;
        let right = parse_item(right, &mut right.bytes().enumerate().peekable())?;

        pairs.push(PacketPair { left, right })
    }

    Ok(pairs)
}

fn part1(data: &[PacketPair]) -> usize {
    data.iter()
        .enumerate()
        .filter(|(_, pair)| pair.left < pair.right)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(data: &[PacketPair]) -> usize {
    let divider = parse("[[2]]\n[[6]]").unwrap();
    let PacketPair { left, right } = &divider[0];

    let mut all_packets: Vec<_> = data
        .iter()
        .flat_map(|pair| [pair.left.clone(), pair.right.clone()])
        .collect();
    all_packets.push(left.clone());
    all_packets.push(right.clone());
    all_packets.sort();

    let left_pos = all_packets.iter().position(|p| p == left).unwrap() + 1;
    let right_pos = all_packets.iter().position(|p| p == right).unwrap() + 1;

    left_pos * right_pos
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

        let packets = parse(&data).unwrap();
        let expected = 13;
        let actual = part1(&packets);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let packets = parse(&data).unwrap();
        let expected = 140;
        let actual = part2(&packets);

        assert_eq!(expected, actual);
    }
}
