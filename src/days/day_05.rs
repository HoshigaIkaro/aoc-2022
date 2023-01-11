use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, newline, none_of, one_of, u32},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use super::Day;

pub struct Day05;

impl Day for Day05 {
    fn part_1(&self, input: &str) -> String {
        let (crates, instructions) = input.split_once("\n\n").unwrap();
        let mut cmap = get_crates(crates);
        for line in instructions.lines() {
            let (num, origin, target) = parse_instruction(line);
            for _ in 0..num {
                let v = cmap.get_mut(origin - 1).unwrap().pop().unwrap();
                cmap.get_mut(target - 1).unwrap().push(v);
            }
        }
        cmap.iter().map(|v| v.last().unwrap()).collect()
    }

    fn part_2(&self, input: &str) -> String {
        let (crates, instructions) = input.split_once("\n\n").unwrap();
        let mut cmap = get_crates(crates);
        for line in instructions.lines() {
            let (num, origin, target) = parse_instruction(line);
            let v = cmap.get_mut(origin - 1).unwrap();
            let b = v.split_off(v.len() - num);
            cmap.get_mut(target - 1).unwrap().extend(b.into_iter());
        }
        cmap.iter().map(|v| v.last().unwrap()).collect()
    }
}

fn get_crates(input: &str) -> Vec<Vec<char>> {
    let (left, right) = input.rsplit_once('\n').unwrap();
    let num_crates = lexical::parse(right.split_whitespace().last().unwrap()).unwrap();
    let mut cmap: Vec<Vec<char>> = vec![Vec::new(); num_crates];
    for line in left.lines() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                cmap[i].insert(0, c);
            }
        }
    }
    cmap
}

fn parse_instruction(line: &str) -> (usize, usize, usize) {
    let mut s = line.split_whitespace();
    s.next();
    let num = lexical::parse(s.next().unwrap()).unwrap();
    s.next();
    let origin = lexical::parse(s.next().unwrap()).unwrap();
    s.next();
    let target = lexical::parse(s.next().unwrap()).unwrap();
    (num, origin, target)
}

fn parse_crate_row(input: &str) -> IResult<&str, Vec<char>> {
    terminated(
        separated_list1(
            char(' '),
            delimited(anychar, none_of("12345667890"), anychar),
        ),
        newline,
    )(input)
}

fn parse_crates(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (rest, crates) = many1(parse_crate_row)(input)?;
    let (rest, number) = terminated(
        separated_list1(
            char(' '),
            delimited(anychar, one_of("12345667890"), anychar),
        ),
        newline,
    )(rest)?;
    let number_of_crates = number.len();
    let mut crate_map: Vec<Vec<char>> = Vec::with_capacity(number_of_crates);
    for stack in 0..number_of_crates {
        crate_map.push(
            crates
                .iter()
                .map(|row| row[stack])
                .filter(|c| *c != ' ')
                .rev()
                .collect(),
        );
    }
    Ok((rest, crate_map))
}

type Instruction = (usize, usize, usize);

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(u32, |s| s as usize)(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(terminated(
        tuple((
            preceded(tag("move "), parse_usize),
            preceded(tag(" from "), parse_usize),
            preceded(tag(" to "), parse_usize),
        )),
        newline,
    ))(input)
}

type CrateMap = Vec<Vec<char>>;
type Instructions = Vec<Instruction>;

pub fn parse_input(input: &str) -> (CrateMap, Instructions) {
    separated_pair(parse_crates, newline, parse_instructions)(input)
        .unwrap()
        .1
}

pub fn part_1((crate_map, instructions): &(CrateMap, Instructions)) -> String {
    let mut crate_map = crate_map.clone();
    for (num, origin, target) in instructions {
        for _ in 0..*num {
            let v = crate_map.get_mut(origin - 1).unwrap().pop().unwrap();
            crate_map.get_mut(target - 1).unwrap().push(v);
        }
    }
    crate_map.iter().map(|v| v.last().unwrap()).collect()
}

pub fn part_2((crate_map, instructions): &(CrateMap, Instructions)) -> String {
    let mut crate_map = crate_map.to_owned();
    for (num, origin, target) in instructions {
        let v = crate_map.get_mut(origin - 1).unwrap();
        let b = v.split_off(v.len() - num);
        crate_map.get_mut(target - 1).unwrap().extend(b.into_iter());
    }
    crate_map.iter().map(|v| v.last().unwrap()).collect()
}

pub fn run(input: &str) -> (String, String) {
    let parsed = parse_input(input);
    (part_1(&parsed), part_2(&parsed))
}
