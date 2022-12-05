use std::collections::{BTreeMap, VecDeque};

use super::Day;

pub struct Day05;

impl Day for Day05 {
    fn part_1(&self, input: &str) -> String {
        let (crates, instructions) = input.split_once("\n\n").unwrap();
        let mut cmap: BTreeMap<usize, VecDeque<char>> = BTreeMap::new();
        for line in crates.lines() {
            if line.contains("1") {
                break;
            }
            let line = line.replace(" [", "@[");
            let line = line.replace("] ", "]@");
            let line = line.replace("    ", "   @");
            for (i, c) in line.split('@').enumerate() {
                if c.chars().any(|c| c.is_alphabetic()) {
                    cmap.entry(i + 1)
                        .or_default()
                        .push_front(c.chars().skip(1).next().unwrap());
                }
            }
        }
        for i in instructions.lines() {
            let s = i.split_whitespace().collect::<Vec<_>>();
            let num = s[1].parse::<u32>().unwrap();
            let origin = s[3].parse::<usize>().unwrap();
            let target = s[5].parse::<usize>().unwrap();
            for _ in 0..num {
                let v = cmap.get_mut(&origin).unwrap().pop_back().unwrap();
                cmap.get_mut(&target).unwrap().push_back(v);
            }
        }
        cmap.values().map(|v| v.back().unwrap()).collect()
    }

    fn part_2(&self, input: &str) -> String {
        let (crates, instructions) = input.split_once("\n\n").unwrap();
        let mut cmap: BTreeMap<usize, VecDeque<char>> = BTreeMap::new();
        for line in crates.lines() {
            if line.contains("1") {
                break;
            }
            let line = line.replace(" [", "@[");
            let line = line.replace("] ", "]@");
            let line = line.replace("    ", "   @");
            for (i, c) in line.split('@').enumerate() {
                if c.chars().any(|c| c.is_alphabetic()) {
                    cmap.entry(i + 1)
                        .or_default()
                        .push_front(c.chars().skip(1).next().unwrap());
                }
            }
        }
        for i in instructions.lines() {
            let s = i.split_whitespace().collect::<Vec<_>>();
            let num = s[1].parse::<u32>().unwrap();
            let origin = s[3].parse::<usize>().unwrap();
            let target = s[5].parse::<usize>().unwrap();
            let mut v = Vec::new();
            for _ in 0..num {
                v.push(cmap.get_mut(&origin).unwrap().pop_back().unwrap());
            }
            cmap.get_mut(&target).unwrap().extend(v.into_iter().rev());
        }
        cmap.values().map(|v| v.back().unwrap()).collect()
    }
}
