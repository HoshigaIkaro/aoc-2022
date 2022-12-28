use super::Day;

pub struct Day02;

impl Day for Day02 {
    fn part_1(&self, input: &str) -> String {
        input
            .lines()
            .map(|s| match s {
                // Opponent: Rock
                "A X" => 4, // 1 + 3
                "A Y" => 8, // 2 + 6
                "A Z" => 3, // 3 + 0
                // Opponent: Paper
                "B X" => 1, // 1 + 0
                "B Y" => 5, // 2 + 3
                "B Z" => 9, // 3 + 6
                // Opponent: Scissors
                "C X" => 7, // 1 + 6
                "C Y" => 2, // 2 + 0
                "C Z" => 6, // 3 + 3
                _ => unreachable!(),
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        input
            .lines()
            .map(|s| match s {
                // Opponent: Rock
                "A X" => 3, // 3 + 0
                "A Y" => 4, // 1 + 3
                "A Z" => 8, // 2 + 6
                // Opponent: Paper
                "B X" => 1, // 1 + 0
                "B Y" => 5, // 2 + 3
                "B Z" => 9, // 3 + 6
                // Opponent: Scissors
                "C X" => 2, // 2 + 0
                "C Y" => 6, // 3 + 3
                "C Z" => 7, // 1 + 6
                _ => unreachable!(),
            })
            .sum::<usize>()
            .to_string()
    }
}

pub fn run(input: &str) -> (u32, u32) {
    let parsed = parse_input(input);
    (part_1(&parsed), part_2(&parsed))
}

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|s| match *s {
            // Opponent: Rock
            "A X" => 4, // 1 + 3
            "A Y" => 8, // 2 + 6
            "A Z" => 3, // 3 + 0
            // Opponent: Paper
            "B X" => 1, // 1 + 0
            "B Y" => 5, // 2 + 3
            "B Z" => 9, // 3 + 6
            // Opponent: Scissors
            "C X" => 7, // 1 + 6
            "C Y" => 2, // 2 + 0
            "C Z" => 6, // 3 + 3
            _ => unreachable!(),
        })
        .sum::<u32>()
}

pub fn part_2(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|s| match *s {
            // Opponent: Rock
            "A X" => 3, // 3 + 0
            "A Y" => 4, // 1 + 3
            "A Z" => 8, // 2 + 6
            // Opponent: Paper
            "B X" => 1, // 1 + 0
            "B Y" => 5, // 2 + 3
            "B Z" => 9, // 3 + 6
            // Opponent: Scissors
            "C X" => 2, // 2 + 0
            "C Y" => 6, // 3 + 3
            "C Z" => 7, // 1 + 6
            _ => unreachable!(),
        })
        .sum::<u32>()
}
