use super::Day;

pub struct Day1;

impl Day for Day1 {
    fn part_1(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|food| food.parse::<u32>().unwrap())
                    .sum::<u32>()
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
                    .map(|food| food.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .collect::<Vec<u32>>();
        elves.sort();
        elves.iter().rev().take(3).sum::<u32>().to_string()
    }
}
