use nom::{
    bytes::complete::take_while,
    character::complete::newline,
    combinator::{map_res, opt},
    multi::{fold_many1, many1},
    sequence::terminated,
    IResult,
};

use super::Day;

pub struct Day01;

impl Day for Day01 {
    fn part_1(&self, input: &str) -> String {
        fold_many1(terminated(parse_elf, opt(newline)), || 0, u32::max)(input)
            .unwrap()
            .1
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        fold_many1(
            terminated(parse_elf, opt(newline)),
            || [0; 3],
            |mut top, elf| {
                let index = get_min_index(top);
                let low = top.get_mut(index).unwrap();
                if elf > *low {
                    *low = elf;
                }
                top
            },
        )(input)
        .unwrap()
        .1
        .into_iter()
        .sum::<u32>()
        .to_string()
    }
}

fn get_min_index(top: [u32; 3]) -> usize {
    let mut lowest = u32::MAX;
    let mut index = 0;
    for (current, &calories) in top.iter().enumerate() {
        if calories < lowest {
            index = current;
            lowest = calories;
        }
    }
    index
}

fn is_valid_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_food(input: &str) -> IResult<&str, u32> {
    terminated(
        map_res(take_while(is_valid_digit), lexical::parse),
        opt(newline),
    )(input)
}

fn parse_elf(input: &str) -> IResult<&str, u32> {
    fold_many1(parse_food, || 0, |acc, food: u32| acc + food)(input)
}

fn parse_elves(input: &str) -> IResult<&str, Vec<u32>> {
    many1(terminated(parse_elf, opt(newline)))(input)
}

pub fn parse_input(input: &str) -> Vec<u32> {
    parse_elves(input).unwrap().1
}

pub fn part_1(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

pub fn part_2(input: &[u32]) -> u32 {
    input
        .iter()
        .fold([0; 3], |mut top, &elf| {
            let index = get_min_index(top);
            let low = top.get_mut(index).unwrap();
            if elf > *low {
                *low = elf;
            }
            top
        })
        .into_iter()
        .sum::<u32>()
}


#[cfg(test)]
mod day_01_tests {
    use super::*;

    #[test]
    fn food_parses() {
        let input = "1000";
        let (_, calories) = parse_food(input).unwrap();
        assert_eq!(calories, 1000);
    }

    #[test]
    fn elf_parses() {
        let input = "1000\n2000\n3000\n";
        let (_, calories) = parse_elf(input).unwrap();
        assert_eq!(calories, 6000);
    }

    #[test]
    fn elves_parses() {
        let input = "1000\n2000\n3000\n\n1000\n2000\n3000\n";
        let (_, elves) = parse_elves(input).unwrap();
        assert_eq!(elves, vec![6000, 6000]);
    }
}
