use nom::{
    character::complete::{char, newline, u32},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

use super::Day;

pub struct Day04;

impl Day for Day04 {
    fn part_1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(',').unwrap();
                (to_pair(left), to_pair(right))
            })
            .filter(|(left, right)| {
                (left.0 >= right.0 && left.1 <= right.1) || (right.0 >= left.0 && right.1 <= left.1)
            })
            .count()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(',').unwrap();
                (to_pair(left), to_pair(right))
            })
            .filter(|(left, right)| left.0 <= right.1 && right.0 <= left.1)
            .count()
            .to_string()
    }
}

fn to_pair(input: &str) -> (usize, usize) {
    let (left, right) = input.split_once('-').unwrap();
    let left: usize = lexical::parse(left).unwrap();
    let right: usize = lexical::parse(right).unwrap();
    (left, right)
}

type Sections = (u32, u32);
type Pair = (Sections, Sections);

fn parse_sections(input: &str) -> IResult<&str, Sections> {
    separated_pair(u32, char('-'), u32)(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    separated_pair(parse_sections, char(','), parse_sections)(input)
}

pub fn parse_input(input: &str) -> Vec<Pair> {
    many1(terminated(parse_pair, newline))(input).unwrap().1
}

pub fn part_1(input: &[Pair]) -> usize {
    input
        .iter()
        .filter(|(left, right)| {
            (left.0 >= right.0 && left.1 <= right.1) || (right.0 >= left.0 && right.1 <= left.1)
        })
        .count()
}

pub fn part_2(input: &[Pair]) -> usize {
    input
        .iter()
        .filter(|(left, right)| left.0 <= right.1 && right.0 <= left.1)
        .count()
}

pub fn run(input: &str) -> (usize, usize) {
    let parsed = parse_input(input);
    (part_1(&parsed), part_2(&parsed))
}
