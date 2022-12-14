use std::{num::ParseIntError, ops::RangeInclusive};

use aoc_lib::{misc::ResultZip, Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};
use itertools::Itertools;

// 10:59
// 13:04

pub const DAY: Day = Day {
    day: 14,
    name: "Regolith Reservoir",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(do_fall(&data)))
}

#[allow(unused)]
fn run_part2(input: &str, b: Bench) -> BenchResult {
    let mut data = parse(input).map_err(UserError)?;
    insert_floor(&mut data);
    b.bench(|| Ok::<_, NoError>(do_fall(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn x_range(self) -> RangeInclusive<u16> {
        self.start.x..=self.end.x
    }

    fn y_range(self) -> RangeInclusive<u16> {
        self.start.y..=self.end.y
    }
}

fn build_blocked_map(lines: &[Line]) -> (u16, usize, u16, Vec<bool>) {
    let floor_height = lines.iter().max_by_key(|l| l.end.y).copied().unwrap().end.y + 2;
    let height = floor_height as usize + 1;
    let left = 500 - (floor_height + 10);
    let right = 500 + floor_height + 10;
    let width = (right - left) as usize;

    let mut blocked = vec![false; width * height];

    for line in lines {
        if line.start.x == line.end.x {
            // Vertical line
            let start = line.start.y.min(line.end.y);
            let end = line.start.y.max(line.end.y);
            for y in start..=end {
                blocked[y as usize * width + (line.start.x - left) as usize] = true;
            }
        } else {
            // Horizontal line
            let start = line.start.x.min(line.end.x);
            let end = line.start.x.max(line.end.x);
            for x in start..=end {
                blocked[line.start.y as usize * width + (x - left) as usize] = true;
            }
        }
    }

    (left, width, floor_height, blocked)
}

fn parse(input: &str) -> Result<Vec<Line>> {
    let mut lines = Vec::new();

    for line in input.lines() {
        let coords = line
            .split(" -> ")
            .flat_map(|pair| pair.split_once(','))
            .map(|(x, y)| -> Result<Point, ParseIntError> {
                x.parse().zip(y.parse()).map(|(x, y)| Point { x, y })
            })
            .tuple_windows()
            .map(|(start, end)| -> Result<Line, ParseIntError> {
                let start = start?;
                let end = end?;

                // Normalise our lines so the start is the left-most and top-most.

                let (start, end) = if end.y < start.y {
                    (end, start)
                } else {
                    (start, end)
                };

                let (start, end) = if end.x < start.x {
                    (end, start)
                } else {
                    (start, end)
                };

                Ok(Line { start, end })
            });

        for line in coords {
            lines.push(line?);
        }
    }

    // Sorting by our Y coordinate means that a linear search will find the first collision.
    lines.sort_unstable_by(|a, b| a.start.y.cmp(&b.start.y));

    Ok(lines)
}

#[allow(unused)]
fn insert_floor(lines: &mut Vec<Line>) {
    let floor_height = lines.iter().max_by_key(|l| l.end.y).copied().unwrap().end.y + 2;

    lines.push(Line {
        start: Point {
            x: 500 - (floor_height + 10),
            y: floor_height,
        },
        end: Point {
            x: 500 + (floor_height + 10),
            y: floor_height,
        },
    });
}

fn do_fall(lines: &[Line]) -> usize {
    let (left_bound, width, floor_height, mut blocked) = build_blocked_map(lines);
    let to_idx = |x: u16, y: u16| y as usize * width + (x - left_bound) as usize;

    let mut num_sand = 0;
    let mut position_stack = Vec::new();
    'outer: loop {
        let mut sand = position_stack.pop().unwrap_or(Point { x: 500, y: 0 });

        // We're full!;
        if blocked[to_idx(sand.x, sand.y)] {
            break;
        }

        loop {
            // We've found an intersecting line, but we don't know if there's any resting sand between it and us.
            // We need to search.
            let blocking_sand = (sand.y + 1..=floor_height)
                .map(|y| (y, blocked[to_idx(sand.x, y)]))
                .find(|(_, pb)| *pb);

            if let Some((blocked_y, _)) = blocking_sand {
                // Check if we fall to either side.

                let lower_left = to_idx(sand.x - 1, blocked_y);
                let lower_right = to_idx(sand.x + 1, blocked_y);
                if !blocked[lower_left] {
                    position_stack.push(sand);
                    sand.x -= 1;
                    sand.y = blocked_y;
                } else if !blocked[lower_right] {
                    position_stack.push(sand);
                    sand.x += 1;
                    sand.y = blocked_y;
                } else {
                    // We can't fall further. Come to rest.
                    num_sand += 1;
                    blocked[to_idx(sand.x, blocked_y - 1)] = true;
                    continue 'outer;
                }
            } else {
                break 'outer;
            }
        }
    }

    num_sand
}

#[allow(unused)]
fn draw_map(lines: &[Line], sand: &[bool], end_y: u16, x_start: u16, x_end: u16) {
    let x_range = (x_end - x_start) as usize;
    let mut map: Vec<char> = sand.iter().map(|b| if *b { 'o' } else { '.' }).collect();

    for line in lines {
        if line.start.y == line.end.y {
            for x in line.x_range() {
                map[line.start.y as usize * x_range + (x - x_start) as usize] = '#';
            }
        } else {
            for y in line.y_range() {
                map[y as usize * x_range + (line.start.x - x_start) as usize] = '#';
            }
        }
    }

    for row in map.chunks_exact(x_range) {
        row.iter().for_each(|c| eprint!("{c}"));
        eprintln!();
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

        let lines = parse(&data).unwrap();
        let expected = 24;
        let actual = do_fall(&lines);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let mut lines = parse(&data).unwrap();
        insert_floor(&mut lines);
        let expected = 93;
        let actual = do_fall(&lines);

        assert_eq!(expected, actual);
    }
}
