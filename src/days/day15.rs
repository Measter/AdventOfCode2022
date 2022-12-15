use std::{collections::HashSet, ops::RangeInclusive};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

// 12:10
// 13:15

pub const DAY: Day = Day {
    day: 15,
    name: "Beacon Exclusion Zone",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let (sensors, beacons) = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1::<2_000_000>(&sensors, &beacons)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let (sensors, _) = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2::<4_000_000>(&sensors)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
}

fn parse(input: &str) -> Result<(Vec<Sensor>, Vec<Point>)> {
    let mut sensors = Vec::new();
    let mut beacons = HashSet::new();

    fn split_line(line: &str) -> Option<(&str, &str, &str, &str)> {
        let line = line.strip_prefix("Sensor at x=")?;
        let (sx, line) = line.split_once(", y=")?;
        let (sy, line) = line.split_once(": closest beacon is at x=")?;
        let (bx, by) = line.split_once(", y=")?;

        Some((sx, sy, bx, by))
    }

    for line in input.lines().map(str::trim) {
        let Some((sx, sy, bx, by)) = split_line(line) else {
            return Err(eyre!("Invalid line: `{}`", line));
        };

        let closest_beacon = Point {
            x: bx.parse()?,
            y: by.parse()?,
        };

        sensors.push(Sensor {
            pos: Point {
                x: sx.parse()?,
                y: sy.parse()?,
            },
            closest_beacon,
        });

        beacons.insert(closest_beacon);
    }

    Ok((sensors, beacons.into_iter().collect()))
}

fn get_x_range(sensor: &Sensor, row: i32) -> Option<RangeInclusive<i32>> {
    let deltay = (sensor.pos.y - sensor.closest_beacon.y).abs();
    let deltax = (sensor.pos.x - sensor.closest_beacon.x).abs();
    let range = deltay + deltax;
    let y_range = (sensor.pos.y - range)..=(sensor.pos.y + range);
    if !y_range.contains(&row) {
        return None;
    }
    let row_half_width = range - (row - sensor.pos.y).abs();
    let x_range = (sensor.pos.x - row_half_width)..=(sensor.pos.x + row_half_width);
    Some(x_range)
}

fn part1<const ROW: i32>(sensors: &[Sensor], beacons: &[Point]) -> usize {
    let mut ranges = Vec::new();

    for sensor in sensors {
        let Some(x_range) = get_x_range(sensor, ROW) else { continue };
        ranges.push(x_range);
    }

    merge_ranges(&mut ranges, &mut Vec::new());
    let mut count = 0;

    let row_beacons: Vec<Point> = beacons.iter().filter(|b| b.y == ROW).copied().collect();

    for range in ranges {
        for x in range {
            if row_beacons.iter().any(|b| b.x == x) {
                continue;
            }
            count += 1;
        }
    }

    count
}

fn merge_ranges(ranges: &mut Vec<RangeInclusive<i32>>, scratch: &mut Vec<RangeInclusive<i32>>) {
    ranges.sort_unstable_by(|a, b| a.end().cmp(b.end()));
    let mut cur_range = ranges.pop().unwrap();

    while let Some(r) = ranges.pop() {
        if r.end() >= cur_range.start() {
            // We overlap, so we can expand our range.
            cur_range = (*r.start()).min(*cur_range.start())..=*cur_range.end();
        } else {
            scratch.push(cur_range);
            cur_range = r;
        }
    }

    ranges.push(cur_range);
    ranges.append(scratch);
}

fn part2<const RANGE: i32>(sensors: &[Sensor]) -> i64 {
    let mut covered_ranges = Vec::<RangeInclusive<i32>>::new();
    let mut scratch = Vec::<RangeInclusive<i32>>::new();

    for row in 0..=RANGE {
        covered_ranges.clear();
        scratch.clear();

        for sensor in sensors {
            let Some(x_range) = get_x_range(sensor, row) else { continue };
            covered_ranges.push(x_range);
        }

        merge_ranges(&mut covered_ranges, &mut scratch);

        if let [first, second] = covered_ranges.as_slice() {
            let x = if first.start() < second.start() {
                second.start() - 1
            } else {
                first.start() + 1
            };

            return x as i64 * 4_000_000 + row as i64;
        }
    }

    panic!("hole not found");
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

        let (sensors, beacons) = parse(&data).unwrap();
        let expected = 26;
        let actual = part1::<10>(&sensors, &beacons);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let (sensors, _) = parse(&data).unwrap();
        let expected = 56_000_011;
        let actual = part2::<20>(&sensors);

        assert_eq!(expected, actual);
    }
}
