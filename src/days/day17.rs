use std::collections::HashSet;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};
use derive_more::Add;

// 16:43
pub const DAY: Day = Day {
    day: 17,
    name: "Template",
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

#[derive(Debug, Clone, Copy, Add, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    const DOWN: Self = Self::new(0, -1);
    const LEFT: Self = Self::new(-1, 0);
    const RIGHT: Self = Self::new(1, 0);

    const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Wind {
    Left,
    Right,
}

impl Wind {
    fn to_rel_point(self) -> Point {
        match self {
            Wind::Left => Point::LEFT,
            Wind::Right => Point::RIGHT,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Flat,
    Cross,
    Angle,
    Vertical,
    Block,
}

impl Shape {
    const COL_FLAT: &[Point] = &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
    ];
    const COL_CROSS: &[Point] = &[
        Point::new(1, 2),
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(1, 0),
    ];
    const COL_ANGLE: &[Point] = &[
        Point::new(2, 2),
        Point::new(2, 1),
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
    ];
    const COL_VERTICAL: &[Point] = &[
        Point::new(0, 3),
        Point::new(0, 2),
        Point::new(0, 1),
        Point::new(0, 0),
    ];
    const COL_BLOCK: &[Point] = &[
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(0, 0),
        Point::new(1, 0),
    ];

    fn next(self) -> Self {
        use Shape::*;
        match self {
            Flat => Cross,
            Cross => Angle,
            Angle => Vertical,
            Vertical => Block,
            Block => Flat,
        }
    }

    fn collision_coords(self) -> &'static [Point] {
        match self {
            Shape::Flat => Self::COL_FLAT,
            Shape::Cross => Self::COL_CROSS,
            Shape::Angle => Self::COL_ANGLE,
            Shape::Vertical => Self::COL_VERTICAL,
            Shape::Block => Self::COL_BLOCK,
        }
    }
}

fn parse(input: &str) -> Result<Vec<Wind>> {
    input
        .trim()
        .bytes()
        .map(|b| match b {
            b'<' => Ok(Wind::Left),
            b'>' => Ok(Wind::Right),
            _ => Err(eyre!("invalid character")),
        })
        .collect()
}

const LEFT_WALL: i16 = -1;
const RIGHT_WALL: i16 = 7;
const FLOOR: i16 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CollisionCheck {
    Clear,
    Blocked,
}

fn collision_check(set_points: &HashSet<Point>, pos: Point) -> CollisionCheck {
    if pos.x == LEFT_WALL || pos.x == RIGHT_WALL || pos.y == FLOOR || set_points.contains(&pos) {
        CollisionCheck::Blocked
    } else {
        CollisionCheck::Clear
    }
}

fn part1(wind: &[Wind]) -> i16 {
    let mut cur_shape = Shape::Block;
    let mut shapes = std::iter::from_fn(move || {
        cur_shape = cur_shape.next();
        Some(cur_shape)
    });
    let mut jets = wind.iter().copied().cycle();

    let mut set_points = HashSet::new();
    let mut spawn_height = 4;
    let mut max_y = 0;

    for _ in 0..2022 {
        let mut cur_position = Point::new(2, spawn_height);
        let shape = shapes.next().unwrap();

        for push_dir in jets.by_ref() {
            let next_position = cur_position + push_dir.to_rel_point();
            if shape
                .collision_coords()
                .iter()
                .map(|&p| p + next_position)
                .all(|p| collision_check(&set_points, p) == CollisionCheck::Clear)
            {
                cur_position = next_position;
            }

            let next_position = cur_position + Point::DOWN;

            if shape
                .collision_coords()
                .iter()
                .map(|&p| p + next_position)
                .all(|p| collision_check(&set_points, p) == CollisionCheck::Clear)
            {
                cur_position = next_position;
                continue;
            }

            for &pos in shape.collision_coords() {
                set_points.insert(pos + cur_position);
            }
            max_y = max_y.max(shape.collision_coords()[0].y + cur_position.y);
            spawn_height = max_y + 4;

            break;
        }
    }

    max_y
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

        let wind = parse(&data).unwrap();
        let expected = 3068;
        let actual = part1(&wind);

        assert_eq!(expected, actual);
    }
}
