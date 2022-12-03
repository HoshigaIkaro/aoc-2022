use super::Day;

pub struct Day3;

impl Day for Day3 {
    fn part_1(&self, input: &str) -> String {
        let mut total: usize = 0;
        for bag in input.lines() {
            let (a, b) = bag.split_at(bag.len() / 2);
            let same = a.chars().find(|c| b.contains(*c)).unwrap();
            total += priority(same);
        }
        total.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut total = 0;
        for group in input.lines().collect::<Vec<_>>().chunks_exact(3) {
            let badge = group[0]
                .chars()
                .find(|c| group[1].contains(*c) && group[2].contains(*c))
                .unwrap();
            total += priority(badge);
        }
        total.to_string()
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
