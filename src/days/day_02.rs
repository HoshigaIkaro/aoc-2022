use super::Day;

pub struct Day02;

impl Day for Day02 {
    fn part_1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(" ").unwrap();
                match a {
                    "A" => match b {
                        "X" => 1 + 3,
                        "Y" => 2 + 6,
                        "Z" => 3 + 0,
                        _ => unimplemented!(),
                    },
                    "B" => match b {
                        "X" => 1 + 0,
                        "Y" => 2 + 3,
                        "Z" => 3 + 6,
                        _ => unimplemented!(),
                    },
                    "C" => match b {
                        "X" => 1 + 6,
                        "Y" => 2 + 0,
                        "Z" => 3 + 3,
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(" ").unwrap();
                match a {
                    "A" => match b {
                        // rock
                        "X" => 3 + 0, // sciscors
                        "Y" => 1 + 3, // rock
                        "Z" => 2 + 6, // paper
                        _ => unimplemented!(),
                    },
                    "B" => match b {
                        // paper
                        "X" => 1 + 0, // rock
                        "Y" => 2 + 3, // paper
                        "Z" => 3 + 6, // scissors
                        _ => unimplemented!(),
                    },
                    "C" => match b {
                        // scissors
                        "X" => 2 + 0, // paper
                        "Y" => 3 + 3, // scissors
                        "Z" => 1 + 6, // rock
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                }
            })
            .sum::<u32>()
            .to_string()
    }
}
