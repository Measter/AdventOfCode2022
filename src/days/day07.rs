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

#[derive(Debug, Clone, Copy, Hash)]
struct EntryId(usize);

#[derive(Debug)]
enum FileSystemEntryKind {
    File { size: usize },
    Directory { children: Vec<EntryId> },
}

impl FileSystemEntryKind {
    fn file(size: usize) -> Self {
        FileSystemEntryKind::File { size }
    }

    fn dir() -> Self {
        FileSystemEntryKind::Directory {
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct FileSystemEntry<'a> {
    id: EntryId,
    name: &'a str,
    parent: Option<EntryId>,
    kind: FileSystemEntryKind,
}

impl<'a> FileSystemEntry<'a> {
    fn size(&self, fs: &FileSystem) -> usize {
        match &self.kind {
            FileSystemEntryKind::File { size } => *size,
            FileSystemEntryKind::Directory { children } => {
                children.iter().map(|&ch| fs.get_entry(ch).size(fs)).sum()
            }
        }
    }

    fn is_file(&self) -> bool {
        matches!(&self.kind, FileSystemEntryKind::File { .. })
    }

    fn is_directory(&self) -> bool {
        matches!(&self.kind, FileSystemEntryKind::Directory { .. })
    }
}

#[derive(Debug)]
struct FileSystem<'a> {
    root: EntryId,
    entries: Vec<FileSystemEntry<'a>>,
}

impl<'a> FileSystem<'a> {
    fn new() -> Self {
        Self {
            root: EntryId(0),
            entries: vec![FileSystemEntry {
                id: EntryId(0),
                name: "/",
                parent: None,
                kind: FileSystemEntryKind::Directory {
                    children: Vec::new(),
                },
            }],
        }
    }

    fn new_entry(
        &mut self,
        name: &'a str,
        parent: EntryId,
        kind: FileSystemEntryKind,
    ) -> &mut FileSystemEntry<'a> {
        let next_id = EntryId(self.entries.len());
        match &mut self.entries[parent.0].kind {
            FileSystemEntryKind::File { .. } => panic!("tried to add child to file"),
            FileSystemEntryKind::Directory { children } => children.push(next_id),
        }

        let new_dir = FileSystemEntry {
            parent: Some(parent),
            name,
            id: next_id,
            kind,
        };

        self.entries.push(new_dir);
        &mut self.entries[next_id.0]
    }

    fn get_children(&self, entry: EntryId) -> &[EntryId] {
        match &self.entries[entry.0].kind {
            FileSystemEntryKind::File { .. } => &[],
            FileSystemEntryKind::Directory { children } => children,
        }
    }

    fn entries(&self) -> impl Iterator<Item = &FileSystemEntry<'a>> {
        self.entries.iter()
    }

    fn get_entry(&self, id: EntryId) -> &FileSystemEntry<'a> {
        &self.entries[id.0]
    }

    fn root(&self) -> EntryId {
        self.root
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

                let entry_kind = match kind {
                    "$" => break, // End of LS listing.
                    "dir" => FileSystemEntryKind::dir(),
                    _ if kind.bytes().all(|b| b.is_ascii_digit()) => {
                        let exists = fs
                            .get_children(cur_dir_id)
                            .iter()
                            .map(|&id| fs.get_entry(id))
                            .filter(|entry| entry.is_file())
                            .any(|entry| entry.name == name);
                        if exists {
                            lines.next();
                            continue;
                        }

                        FileSystemEntryKind::file(kind.parse().unwrap())
                    }
                    _ => {
                        return Err(eyre!("Unexpected input: {:?}", line));
                    }
                };

                fs.new_entry(name, cur_dir_id, entry_kind);
                lines.next();
            }
        } else if let Some(dir_name) = line.strip_prefix("$ cd ") {
            if dir_name == "/" {
                cur_dir_id = fs.root();
            } else if dir_name == ".." {
                let cur_dir = fs.get_entry(cur_dir_id);
                cur_dir_id = if let Some(cd) = cur_dir.parent {
                    cd
                } else {
                    return Err(eyre!("Invalid directory travarsal"));
                };
            } else {
                let is_known_dir = fs
                    .get_children(cur_dir_id)
                    .iter()
                    .map(|&ci| fs.get_entry(ci))
                    .filter(|ci| ci.is_directory())
                    .find(|ci| ci.name == dir_name);

                cur_dir_id = if let Some(new_id) = is_known_dir {
                    new_id.id
                } else {
                    fs.new_entry(dir_name, cur_dir_id, FileSystemEntryKind::dir())
                        .id
                };
            }
        } else {
            return Err(eyre!("Unexpected input: {:?}", line));
        }
    }

    Ok(fs)
}

fn part1(fs: &FileSystem) -> usize {
    fs.entries()
        .filter(|e| e.is_directory())
        .map(|d| d.size(fs))
        .filter(|&d| d <= 100000)
        .sum()
}

fn part2(fs: &FileSystem) -> usize {
    const TOTAL_FS_SIZE: usize = 70_000_000;
    const NEEDED_SPACE: usize = 30_000_000;

    let used_space = fs.get_entry(fs.root()).size(fs);
    let free_space = TOTAL_FS_SIZE - used_space;

    let space_to_free = NEEDED_SPACE - free_space;

    fs.entries()
        .filter(|e| e.is_directory())
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
