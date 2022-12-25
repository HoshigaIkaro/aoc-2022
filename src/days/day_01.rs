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
                let index = get_min_index(top);
                let low = top.get_mut(index).unwrap();
                if elf > *low {
                    *low = elf;
                }
                top
            })
            .into_iter()
            .sum::<usize>()
            .to_string()
    }
}

fn get_min_index(top: [usize; 3]) -> usize {
    let mut lowest = usize::MAX;
    let mut index = 0;
    for (current, &calories) in top.iter().enumerate() {
        if calories < lowest {
            index = current;
            lowest = calories;
        }
    }
    index
}
