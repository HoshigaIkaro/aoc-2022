use super::Day;

pub struct Day03;

impl Day for Day03 {
    fn part_1(&self, input: &str) -> String {
        input
            .lines()
            .map(|bag| {
                let (a, b) = bag.split_at(bag.len() / 2);
                let same = a.chars().find(|c| b.contains(*c)).unwrap();
                priority(same)
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        input
            .lines()
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(|group| {
                let badge = group[0]
                    .chars()
                    .find(|c| group[1].contains(*c) && group[2].contains(*c))
                    .unwrap();
                priority(badge)
            })
            .sum::<usize>()
            .to_string()
    }
}

fn priority(c: char) -> usize {
    match c {
        'a'..='z' => c as u8 - b'a' + 1,
        'A'..='Z' => c as u8 - b'A' + 27,
        _ => unreachable!(),
    }
    .into()
}

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|bag| {
            let (a, b) = bag.split_at(bag.len() / 2);
            let same = a.chars().find(|c| b.contains(*c)).unwrap();
            priority(same)
        })
        .sum()
}

pub fn part_2(input: &[&str]) -> usize {
    input
        .chunks_exact(3)
        .map(|group| {
            let badge = group[0]
                .chars()
                .find(|c| group[1].contains(*c) && group[2].contains(*c))
                .unwrap();
            priority(badge)
        })
        .sum()
}

pub fn run(input: &str) -> (usize, usize) {
    let parsed = parse_input(input);
    (part_1(&parsed), part_2(&parsed))
}