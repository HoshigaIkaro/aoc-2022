use std::collections::{HashMap, HashSet};

use super::Day;

pub struct Day07;

impl Day for Day07 {
    fn part_1(&self, input: &str) -> String {
        let mut dirs: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut files: HashMap<&str, Vec<usize>> = HashMap::new();
        let mut dir_stack = vec!["/"];

        for line in input.lines() {
            if line.starts_with("$ l") {
                continue;
            } else if line.starts_with("$ c") {
                match line.trim_start_matches("$ cd ") {
                    ".." => {
                        dir_stack.pop();
                    }
                    "/" => dir_stack.truncate(1),
                    dir => dir_stack.push(dir),
                }
            } else if line.starts_with("d") {
                let dir = line.trim_start_matches("dir ");
                dirs.entry(dir_stack.last().unwrap()).or_default().push(dir);
                dirs.insert(dir, Vec::new());
                files.insert(dir, Vec::new());
            } else {
                files
                    .entry(dir_stack.last().unwrap())
                    .or_default()
                    .push(line.split_whitespace().next().unwrap().parse().unwrap())
            }
        }

        let mut total = 0;
        for dir in dirs.keys() {
            let sum = size(dir, &dirs, &files);

            if sum <= 100000 {
                total += sum;
            }
        }

        total.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

fn size(dir: &str, dirs: &HashMap<&str, Vec<&str>>, files: &HashMap<&str, Vec<usize>>) -> usize {
    files.get(dir).unwrap().iter().sum::<usize>()
        + dirs
            .get(dir)
            .unwrap()
            .iter()
            .map(|new_dir| size(new_dir, dirs, files))
            .sum::<usize>()
}
