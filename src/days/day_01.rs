use super::Day;

pub struct Day01;

impl Day for Day01 {
    fn part_1(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|food| food.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut elves = input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|food| food.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .collect::<Vec<usize>>();
        elves.sort();
        elves.iter().rev().take(3).sum::<usize>().to_string()
    }
}
