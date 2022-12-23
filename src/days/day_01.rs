use super::Day;

pub struct Day01;

impl Day for Day01 {
    fn part_1(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|food| {
                        let food: usize = lexical::parse(food).unwrap();
                        food
                    })
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
                    .map(|food| {
                        let food: usize = lexical::parse(food).unwrap();
                        food
                    })
                    .sum::<usize>()
            })
            .collect::<Vec<usize>>();
        elves.sort_unstable();
        elves.iter().rev().take(3).sum::<usize>().to_string()
    }
}
