use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::Day;

type Dirs = HashMap<u64, Vec<u64>>;
type Files = HashMap<u64, Vec<u64>>;

pub struct Day07;

impl Day for Day07 {
    fn part_1(&self, input: &str) -> String {
        let (dirs, files) = parse(input);
        dirs.keys()
            .filter_map(|con| {
                let size = size(*con, &dirs, &files);
                if size <= 100_000 {
                    Some(size)
                } else {
                    None
                }
            })
            .sum::<u64>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let (dirs, files) = parse(input);
        let total_size: u64 = files.values().flatten().sum();
        let remaining = 30_000_000 - (70_000_000 - total_size);
        dirs.keys()
            .filter_map(|con| {
                let size = size(*con, &dirs, &files);
                if size >= remaining {
                    Some(size)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
            .to_string()
    }
}

fn parse(input: &str) -> (Dirs, Files) {
    let mut dirs: Dirs = HashMap::new();
    let mut files: Files = HashMap::new();
    let mut dir_stack: Vec<&str> = vec!["/"];

    let mut context: u64 = 0;
    for line in input.lines() {
        let mut line = line.split_whitespace();
        match line.next().unwrap() {
            // Command
            "$" => match line.next().unwrap() {
                "cd" => match line.next().unwrap() {
                    "/" => dir_stack.truncate(1),
                    ".." => {
                        dir_stack.pop();
                    }
                    dir => dir_stack.push(dir),
                },
                "ls" => {
                    let mut hasher = DefaultHasher::new();
                    dir_stack.hash(&mut hasher);
                    context = hasher.finish();
                }
                _ => unreachable!(),
            },
            // ls output
            item => match item {
                "dir" => {
                    let dir = line.next().unwrap();
                    let mut hasher = DefaultHasher::new();
                    let mut new = dir_stack.clone();
                    new.push(dir);
                    new.hash(&mut hasher);
                    let hash = hasher.finish();
                    dirs.entry(context).or_default().push(hash);
                    dirs.insert(hash, Vec::new());
                }
                size => files
                    .entry(context)
                    .or_default()
                    .push(size.parse().unwrap()),
            },
        }
    }
    (dirs, files)
}

fn size(context: u64, dirs: &Dirs, files: &Files) -> u64 {
    files.get(&context).map_or(0, |v| v.iter().sum::<u64>())
        + dirs
            .get(&context)
            .unwrap()
            .iter()
            .map(|new_context| size(*new_context, dirs, files))
            .sum::<u64>()
}
