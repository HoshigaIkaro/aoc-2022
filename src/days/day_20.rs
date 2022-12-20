use super::Day;

const KEY: isize = 811_589_153;

pub struct Day20;

impl Day for Day20 {
    fn part_1(&self, input: &str) -> String {
        let numbers = input
            .lines()
            .map(|line| line.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let max = numbers.len();
        let mut new = (0..).take(max).collect::<Vec<_>>();
        for (index, original) in numbers.iter().copied().enumerate() {
            let start = new.iter().position(|i| *i == index).unwrap();
            let element = new.remove(start);
            let end = (start as isize + original).rem_euclid(new.len() as isize) as usize;
            new.insert(end, element);
        }
        let original_index = numbers.iter().position(|i| *i == 0).unwrap();
        let index = new.iter().position(|i| *i == original_index).unwrap();
        [1000, 2000, 3000]
            .into_iter()
            .map(|i| (i + index) % max)
            .map(|i| {
                let n = numbers[new[i]];
                dbg!(n);
                n
            })
            .sum::<isize>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let numbers = input
            .lines()
            .map(|line| line.parse::<isize>().unwrap() * KEY)
            .collect::<Vec<_>>();
        let max = numbers.len();
        let mut new = (0..).take(max).collect::<Vec<_>>();
        for _ in 0..10 {
            for (index, original) in numbers.iter().copied().enumerate() {
                let start = new.iter().position(|i| *i == index).unwrap();
                let element = new.remove(start);
                let end = (start as isize + original).rem_euclid(new.len() as isize) as usize;
                new.insert(end, element);
            }
        }
        let original_index = numbers.iter().position(|i| *i == 0).unwrap();
        let index = new.iter().position(|i| *i == original_index).unwrap();
        [1000, 2000, 3000]
            .into_iter()
            .map(|i| (i + index) % max)
            .map(|i| {
                let n = numbers[new[i]];
                dbg!(n);
                n
            })
            .sum::<isize>()
            .to_string()
    }
}
