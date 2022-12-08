use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

// 10:16
// 11:26

pub const DAY: Day = Day {
    day: 8,
    name: "Treetop Tree House",
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

struct Grid {
    size: usize,
    trees: Vec<i8>,
}

fn parse(input: &str) -> Result<Grid> {
    let width = input.as_bytes().iter().position(|&d| d == b'\n').unwrap();

    let trees = input
        .as_bytes()
        .iter()
        .filter(|b| b.is_ascii_digit())
        .map(|d| (d - b'0') as i8)
        .collect();

    Ok(Grid { size: width, trees })
}

fn part1(trees: &Grid) -> usize {
    // We know the outside trees are always visible.
    let mut visibility = vec![false; trees.trees.len()];

    let mut check = |idx: usize, max: &mut i8| {
        let tree_height = trees.trees[idx];
        if tree_height > *max {
            visibility[idx] = true;
            *max = tree_height;
        }
    };

    for y in 0..trees.size {
        let mut max_height_left = -1;
        let mut max_height_right = -1;
        let mut max_height_top = -1;
        let mut max_height_bottom = -1;
        for x in 0..trees.size {
            // Left > Right
            let idx = y * trees.size + x;
            check(idx, &mut max_height_left);

            // Right > Left
            let idx = y * trees.size + (trees.size - x - 1);
            check(idx, &mut max_height_right);

            // Top > Bottom
            let idx = x * trees.size + y;
            check(idx, &mut max_height_top);

            // Bottom > Top
            let idx = (trees.size - x - 1) * trees.size + y;
            check(idx, &mut max_height_bottom);
        }
    }

    visibility.into_iter().filter(|v| *v).count()
}

fn part2(trees: &Grid) -> usize {
    let mut max_score = 0;

    for (y, row) in trees.trees.chunks_exact(trees.size).enumerate() {
        for (x, &tree) in row.iter().enumerate() {
            // Do the search leftwards.
            let mut left_score = x;
            for new_x in (0..x).rev() {
                let idx = y * trees.size + new_x;
                if trees.trees[idx] >= tree {
                    left_score = x - new_x;
                    break;
                }
            }

            // Rightwards search.
            let mut right_score = trees.size - x - 1;
            for new_x in x + 1..trees.size {
                let idx = y * trees.size + new_x;
                if trees.trees[idx] >= tree {
                    right_score = new_x - x;
                    break;
                }
            }

            // Upwards search.
            let mut up_score = y;
            for new_y in (0..y).rev() {
                let idx = new_y * trees.size + x;
                if trees.trees[idx] >= tree {
                    up_score = y - new_y;
                    break;
                }
            }

            // Downwards search.
            let mut down_score = trees.size - y - 1;
            for new_y in y + 1..trees.size {
                let idx = new_y * trees.size + x;
                if trees.trees[idx] >= tree {
                    down_score = new_y - y;
                    break;
                }
            }

            max_score = max_score.max(left_score * right_score * up_score * down_score);
        }
    }

    max_score
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

        let trees = parse(&data).unwrap();
        let expected = 21;
        let actual = part1(&trees);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let trees = parse(&data).unwrap();
        let expected = 8;
        let actual = part2(&trees);

        assert_eq!(expected, actual);
    }
}
