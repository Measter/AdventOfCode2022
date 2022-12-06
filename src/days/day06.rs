use aoc_lib::{misc::ArrWindows, Bench, BenchResult, Day, NoError};

// 11:40
// 11:57

pub const DAY: Day = Day {
    day: 6,
    name: "Tuning Trouble",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(solve::<4>(input)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(solve::<14>(input)))
}

fn solve<const N: usize>(data: &str) -> usize {
    ArrWindows::<_, N>::new(data.as_bytes())
        .enumerate()
        .find(|(_, bytes)| {
            assert!(bytes.iter().all(|b| b.is_ascii_lowercase()));
            bytes
                .iter()
                .map(|b| 1 << ((b - b'a') as u32))
                .fold(0u32, |acc, b| acc | b)
                .count_ones() as usize
                == N
        })
        .unwrap()
        .0
        + N
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

        let tests = data.lines().map(|l| {
            let (test, expected) = l.split_once('-').unwrap();
            let (expected, _) = expected.split_once(',').unwrap();
            (test, expected.parse::<usize>().unwrap())
        });

        for (test, expected) in tests {
            let actual = solve::<4>(test);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let tests = data.lines().map(|l| {
            let (test, expected) = l.split_once('-').unwrap();
            let (_, expected) = expected.split_once(',').unwrap();
            (test, expected.parse::<usize>().unwrap())
        });

        for (test, expected) in tests {
            let actual = solve::<14>(test);
            assert_eq!(expected, actual);
        }
    }
}
