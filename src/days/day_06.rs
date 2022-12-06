use super::Day;

pub struct Day06;

impl Day for Day06 {
    fn part_1(&self, input: &str) -> String {
        {
            input
                .chars()
                .collect::<Vec<char>>()
                .windows(4)
                .enumerate()
                .find(|(i, window)| {
                    let mut s = Vec::with_capacity(4);
                    for c in window.iter() {
                        if s.contains(c) {
                            return false;
                        }
                        s.push(*c);
                    }
                    true
                })
                .unwrap()
                .0
                + 4
        }
        .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        {
            input
                .chars()
                .collect::<Vec<char>>()
                .windows(14)
                .enumerate()
                .find(|(i, window)| {
                    let mut s = Vec::with_capacity(14);
                    for c in window.iter() {
                        if s.contains(c) {
                            return false;
                        }
                        s.push(*c);
                    }
                    true
                })
                .unwrap()
                .0
                + 14
        }
        .to_string()
    }
}
