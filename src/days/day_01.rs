use super::Day;

pub struct Day01;

impl Day for Day01 {
    fn part_1(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines().fold(0, |acc, food| {
                    let food: usize = lexical::parse(food).unwrap();
                    acc + food
                })
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines().fold(0, |acc, food| {
                    let food: usize = lexical::parse(food).unwrap();
                    acc + food
                })
            })
            .fold([0; 3], |mut top, elf| {
                if let Some(index) = get_top_index(elf, top) {
                    top[index] = elf;
                }
                top
            })
            .into_iter()
            .sum::<usize>()
            .to_string()
    }
}

fn get_top_index(new: usize, top: [usize; 3]) -> Option<usize> {
    for index in 0..3 {
        if new > top[index] {
            return Some(index);
        }
    }
    None
}
