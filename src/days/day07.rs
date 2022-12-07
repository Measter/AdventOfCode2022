use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

// 12:01
// 13:26

pub const DAY: Day = Day {
    day: 7,
    name: "No Space Left On Device",
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

#[derive(Debug)]
struct FileSystem<'a> {
    root: DirectoryID,
    directories: Vec<Directory<'a>>,
}

impl<'a> FileSystem<'a> {
    fn new() -> Self {
        Self {
            root: DirectoryID(0),
            directories: vec![Directory {
                id: DirectoryID(0),
                name: "/",
                parent: None,
                files: Vec::new(),
                sub_directories: Vec::new(),
            }],
        }
    }

    fn new_directory(&mut self, name: &'a str, parent: Option<DirectoryID>) -> &mut Directory<'a> {
        let next_id = DirectoryID(self.directories.len());
        let new_dir = Directory {
            parent,
            name,
            id: next_id,
            files: Vec::new(),
            sub_directories: Vec::new(),
        };

        self.directories.push(new_dir);
        &mut self.directories[next_id.0]
    }

    fn directories(&self) -> impl Iterator<Item = &Directory<'a>> {
        self.directories.iter()
    }

    fn get_dir(&self, id: DirectoryID) -> &Directory<'a> {
        &self.directories[id.0]
    }

    fn get_mut_dir(&mut self, id: DirectoryID) -> &mut Directory<'a> {
        &mut self.directories[id.0]
    }

    fn root(&self) -> DirectoryID {
        self.root
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct DirectoryID(usize);

#[derive(Debug)]
struct Directory<'a> {
    id: DirectoryID,
    name: &'a str,
    parent: Option<DirectoryID>,
    files: Vec<(usize, &'a str)>,
    sub_directories: Vec<DirectoryID>,
}

impl<'a> Directory<'a> {
    fn size(&self, fs: &FileSystem) -> usize {
        self.files.iter().map(|(s, _)| s).sum::<usize>()
            + self
                .sub_directories
                .iter()
                .map(|sd| fs.get_dir(*sd).size(fs))
                .sum::<usize>()
    }

    fn add_file(&mut self, name: &'a str, size: usize) {
        self.files.push((size, name));
    }

    fn add_dir(&mut self, id: DirectoryID) {
        self.sub_directories.push(id);
    }

    fn sub_directories(&self) -> &[DirectoryID] {
        self.sub_directories.as_ref()
    }
}

fn parse(input: &str) -> Result<FileSystem> {
    let mut lines = input.lines().peekable();
    let mut fs = FileSystem::new();
    let mut cur_dir_id = fs.root();

    while let Some(line) = lines.next() {
        if line == "$ ls" {
            loop {
                let Some(next) = lines.peek().copied() else { break };
                let Some((kind, name)) = next.split_once(' ') else {
                    return Err(eyre!("Unexpected input: {:?}", next));
                };

                match kind {
                    "$" => break, // End of LS listing.
                    "dir" => {
                        let sub_dir_id = fs.new_directory(name, Some(cur_dir_id)).id;
                        let cur_dir = fs.get_mut_dir(cur_dir_id);
                        cur_dir.add_dir(sub_dir_id);

                        lines.next();
                    }
                    _ if kind.bytes().all(|b| b.is_ascii_digit()) => {
                        let cur_dir = fs.get_mut_dir(cur_dir_id);
                        cur_dir.add_file(name, kind.parse().unwrap());

                        lines.next();
                    }
                    _ => {
                        return Err(eyre!("Unexpected input: {:?}", line));
                    }
                }
            }
        } else if let Some(dir_name) = line.strip_prefix("$ cd ") {
            if dir_name == "/" {
                cur_dir_id = fs.root();
            } else if dir_name == ".." {
                let cur_dir = fs.get_dir(cur_dir_id);
                cur_dir_id = if let Some(cd) = cur_dir.parent {
                    cd
                } else {
                    return Err(eyre!("Invalid directory travarsal"));
                };
            } else {
                let cur_dir = fs.get_dir(cur_dir_id);
                let is_known_dir = cur_dir
                    .sub_directories()
                    .iter()
                    .find(|&&id| fs.get_dir(id).name == dir_name);

                cur_dir_id = if let Some(&new_id) = is_known_dir {
                    new_id
                } else {
                    let new_id = fs.new_directory(dir_name, Some(cur_dir_id));
                    new_id.id
                };
            }
        } else {
            return Err(eyre!("Unexpected input: {:?}", line));
        }
    }

    Ok(fs)
}

fn part1(fs: &FileSystem) -> usize {
    fs.directories()
        .map(|d| d.size(fs))
        .filter(|&d| d <= 100000)
        .sum()
}

fn part2(fs: &FileSystem) -> usize {
    const TOTAL_FS_SIZE: usize = 70_000_000;
    const NEEDED_SPACE: usize = 30_000_000;

    let used_space = fs.get_dir(fs.root()).size(fs);
    let free_space = TOTAL_FS_SIZE - used_space;

    let space_to_free = NEEDED_SPACE - free_space;

    fs.directories()
        .map(|d| d.size(fs))
        .filter(|&s| s >= space_to_free)
        .min()
        .unwrap()
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

        let tree = parse(&data).unwrap();
        let expected = 95437;
        let actual = part1(&tree);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let tree = parse(&data).unwrap();
        let expected = 24_933_642;
        let actual = part2(&tree);

        assert_eq!(expected, actual);
    }
}
