use nom::{
    character::{self, complete::newline},
    combinator::opt,
    multi::{fold_many1, many1},
    sequence::terminated,
    IResult,
};

use super::Day;

pub struct Day01;

impl Day for Day01 {
    fn part_1(&self, input: &str) -> String {
        let (_, elves) = parse_elves(input).unwrap();
        elves.into_iter().max().unwrap().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let (_, elves) = parse_elves(input).unwrap();
        elves
            .into_iter()
            .fold([0; 3], |mut top, elf| {
                let index = get_min_index(top);
                let low = top.get_mut(index).unwrap();
                if elf > *low {
                    *low = elf;
                }
                top
            })
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

#[allow(dead_code)]
fn parse_food(input: &str) -> IResult<&str, u32> {
    terminated(character::complete::u32, opt(newline))(input)
}

#[allow(dead_code)]
fn parse_elf(input: &str) -> IResult<&str, u32> {
    fold_many1(
        terminated(character::complete::u32, opt(newline)),
        || 0,
        |acc, food| acc + food,
    )(input)
}

fn parse_elves(input: &str) -> IResult<&str, Vec<u32>> {
    many1(terminated(
        fold_many1(
            terminated(character::complete::u32, opt(newline)),
            || 0,
            |acc, food| acc + food,
        ),
        opt(newline),
    ))(input)
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
