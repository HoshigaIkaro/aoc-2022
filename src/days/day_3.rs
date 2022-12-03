use super::Day;

pub struct Day3;

impl Day for Day3 {
    fn part_1(&self, input: &str) -> String {
        input
            .lines()
            .map(|bag| {
                let (a, b) = bag.split_at(bag.len() / 2);
                let same = a.chars().find(|c| b.contains(*c)).unwrap();
                priority(same)
            })
            .sum::<u32>()
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
            .sum::<u32>()
            .to_string()
    }
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u8 - b'a' + 1,
        'A'..='Z' => c as u8 - b'A' + 27,
        _ => unreachable!(),
    }
    .into()
}
