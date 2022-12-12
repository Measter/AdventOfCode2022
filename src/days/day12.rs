use std::collections::BinaryHeap;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

// 10:21
// 11:39

pub const DAY: Day = Day {
    day: 12,
    name: "Hill Climbing Algorithm",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(path_search_part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(path_search_part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    fn to_idx(self, width: usize) -> usize {
        self.y as usize * width + self.x as usize
    }

    fn neighbours(self) -> [Point; 4] {
        let Point { x, y } = self;
        [
            Point::new(x, y.wrapping_sub(1)),
            Point::new(x.wrapping_sub(1), y),
            Point::new(x + 1, y),
            Point::new(x, y + 1),
        ]
    }

    fn cost_to(self, to: Point) -> u16 {
        let (max_x, min_x) = (self.x.max(to.x), self.x.min(to.x));
        let (max_y, min_y) = (self.y.max(to.y), self.y.min(to.y));

        (max_x - min_x) as u16 + (max_y - min_y) as u16
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

impl Map {
    fn contains(&self, point: Point) -> bool {
        (0..self.width as u8).contains(&point.x) && (0..self.height as u8).contains(&point.y)
    }

    fn can_traverse(&self, from: Point, to: Point) -> bool {
        if !self.contains(from) || !self.contains(to) {
            return false;
        }

        let from_tile = self.tiles[from.to_idx(self.width)];
        let to_tile = self.tiles[to.to_idx(self.width)];

        to_tile < from_tile || to_tile - from_tile <= 1
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct State {
    heuristic_cost: u16,
    cost: u16,
    pos: Point,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic_cost == other.heuristic_cost
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heuristic_cost.cmp(&self.heuristic_cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Result<Map> {
    let mut tiles = Vec::new();
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    let mut width = 0;
    let mut height = 0;

    for byte in input.trim().bytes() {
        match byte {
            b'a'..=b'z' => tiles.push(byte - b'a'),
            b'\r' => continue,
            b'\n' => {
                width = 0;
                height += 1;
                continue;
            }
            b'S' => {
                tiles.push(0);
                start = Point::new(width as u8, height as u8);
            }
            b'E' => {
                tiles.push(25);
                end = Point::new(width as u8, height as u8);
            }
            _ => return Err(eyre!("Invalid character: `{}`", byte as char)),
        }
        width += 1;
    }
    height += 1;

    assert_eq!(width * height, tiles.len(),);

    Ok(Map {
        tiles,
        width,
        height,
        start,
        end,
    })
}

fn path_search_part1(map: &Map) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut dist = vec![u16::MAX; map.width * map.height];
    let mut prev = vec![Point::new(255, 255); map.width * map.height];

    dist[map.start.to_idx(map.width)] = 0;
    queue.push(State {
        heuristic_cost: 0,
        cost: 0,
        pos: map.start,
    });

    while let Some(next) = queue.pop() {
        for neighbour in next.pos.neighbours() {
            if !map.contains(neighbour) {
                continue;
            }
            if !map.can_traverse(next.pos, neighbour) {
                continue;
            }
            let total_cost = next.cost + 1;
            if neighbour == map.end {
                prev[map.end.to_idx(map.width)] = next.pos;
                break;
            }
            let nidx = neighbour.to_idx(map.width);
            if total_cost < dist[nidx] {
                dist[nidx] = total_cost;
                prev[nidx] = next.pos;
                queue.push(State {
                    heuristic_cost: total_cost + neighbour.cost_to(map.end),
                    cost: total_cost,
                    pos: neighbour,
                });
            }
        }
    }

    let mut steps = 0;
    let mut cur_pos = map.end;
    while let Some(prev) = prev.get(cur_pos.to_idx(map.width)) {
        steps += 1;
        cur_pos = *prev;
    }

    steps - 1
}

fn path_search_part2(map: &Map) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut dist = vec![u16::MAX; map.width * map.height];
    let mut prev = vec![Point::new(255, 255); map.width * map.height];

    dist[map.end.to_idx(map.width)] = 0;
    queue.push(State {
        heuristic_cost: 0,
        cost: 0,
        pos: map.end,
    });

    let mut last_tile = map.start;

    while let Some(next) = queue.pop() {
        for neighbour in next.pos.neighbours() {
            if !map.contains(neighbour) {
                continue;
            }
            if !map.can_traverse(neighbour, next.pos) {
                continue;
            }
            let total_cost = next.cost + 1;
            let nidx = neighbour.to_idx(map.width);

            if total_cost < dist[nidx] {
                dist[nidx] = total_cost;
                prev[nidx] = next.pos;
                queue.push(State {
                    heuristic_cost: total_cost,
                    cost: total_cost,
                    pos: neighbour,
                });
            }
            if map.tiles[nidx] == 0 && dist[nidx] < dist[last_tile.to_idx(map.width)] {
                prev[nidx] = next.pos;
                last_tile = neighbour;
            }
        }
    }

    let mut steps = 0;
    let mut cur_pos = last_tile;
    while let Some(prev) = prev.get(cur_pos.to_idx(map.width)) {
        steps += 1;
        cur_pos = *prev;
    }

    steps - 1
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

        let map = parse(&data).unwrap();
        let expected = 31;
        let actual = path_search_part1(&map);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let map = parse(&data).unwrap();
        let expected = 29;
        let actual = path_search_part2(&map);

        assert_eq!(expected, actual);
    }
}
