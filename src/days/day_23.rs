use std::ops::Add;

use rayon::prelude::*;
use rustc_hash::FxHashMap;

use super::Day;

// Point(x, y)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    /// Gets the three indices to check.
    ///
    /// The bool array is assumed to be from west to east, north to south.
    fn indicies_to_check(&self) -> [usize; 3] {
        match self {
            Direction::North => [0, 1, 2],
            Direction::South => [5, 6, 7],
            Direction::West => [0, 3, 5],
            Direction::East => [2, 4, 7],
        }
    }

    fn new_point(&self, Point(x, y): Point) -> Point {
        match self {
            Direction::South => Point(x, y + 1),
            Direction::North => Point(x, y - 1),
            Direction::West => Point(x - 1, y),
            Direction::East => Point(x + 1, y),
        }
    }
}

struct Elf {
    position: Point,
    directions: [Direction; 4],
}

impl Elf {
    fn new(position: Point) -> Self {
        Self {
            position,
            directions: [
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
        }
    }

    const fn deltas_around(&self) -> [Point; 8] {
        [
            Point(-1, -1), // NW
            Point(0, -1),  // N
            Point(1, -1),  // NE
            Point(-1, 0),  // W
            Point(1, 0),   // E
            Point(-1, 1),  // SW
            Point(0, 1),   // S
            Point(1, 1),   // SE
        ]
    }

    /// Returns whether the eight surrounding tiles are occupied.
    ///
    /// The array in in the order \[NW, N, NE, W, E, SW, S, SE\]
    fn get_surrounding_elves(&self, elves: &[Elf]) -> [bool; 8] {
        self.deltas_around()
            .into_iter()
            .map(|delta| {
                let new = self.position + delta;
                elves.iter().any(|elf| elf.position == new)
            })
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap()
    }

    fn propose(&self, elves: &[Elf], start_index: usize) -> Option<Point> {
        let surrounding = self.get_surrounding_elves(elves);
        if surrounding.iter().any(|occupied| *occupied) {
            for index in 0..4 {
                let new_index = (start_index + index) % 4;
                let direction = self.directions[new_index];
                if direction.indicies_to_check().into_iter().all(|i| {
                    let occupied = surrounding[i];
                    !occupied
                }) {
                    // can propose
                    let new = direction.new_point(self.position);
                    return Some(new);
                }
            }
        }
        None
    }
}

#[derive(Debug)]
enum ProposalState {
    None,
    One(usize),
    Multiple,
}

fn generate_proposals(elves: &[Elf], round: usize) -> FxHashMap<Point, ProposalState> {
    elves
        .par_iter()
        .enumerate()
        .filter_map(|(id, elf)| {
            if let Some(new_point) = elf.propose(&elves, round % 4) {
                Some((new_point, id))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .fold(FxHashMap::default(), |mut proposed, (new_point, id)| {
            let state = proposed.entry(new_point).or_insert(ProposalState::None);
            match state {
                ProposalState::None => *state = ProposalState::One(id),
                ProposalState::One(_) => *state = ProposalState::Multiple,
                ProposalState::Multiple => (),
            }
            proposed
        })
}

fn apply_proposals(elves: &mut [Elf], proposals: &FxHashMap<Point, ProposalState>) -> bool {
    let mut checked = false;
    for (point, state) in proposals {
        if let ProposalState::One(id) = state {
            elves.get_mut(*id).unwrap().position = *point;
            checked = true;
        }
    }
    checked
}

pub struct Day23;

impl Day for Day23 {
    fn part_1(&self, input: &str) -> String {
        let mut elves = parse_elves(input);
        for round in 1..=10 {
            // first half
            let proposals = generate_proposals(&elves, round - 1);

            // second half
            apply_proposals(&mut elves, &proposals);
        }
        // display_elves(&elves);
        let min_x = elves.iter().map(|elf| elf.position.0).min().unwrap();
        let max_x = elves.iter().map(|elf| elf.position.0).max().unwrap();
        let min_y = elves.iter().map(|elf| elf.position.1).min().unwrap();
        let max_y = elves.iter().map(|elf| elf.position.1).max().unwrap();

        let width = min_x.abs_diff(max_x) + 1;
        let length = min_y.abs_diff(max_y) + 1;
        let area = width * length;

        let empty = area - elves.len();
        empty.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut elves = parse_elves(input);
        let mut round = 1;
        loop {
            // first half
            let proposals = generate_proposals(&elves, round - 1);

            // second half
            let moved = apply_proposals(&mut elves, &proposals);
            if !moved {
                break;
            }
            round += 1;
        }
        round.to_string()
    }
}

fn parse_elves(input: &str) -> Vec<Elf> {
    let mut elves = Vec::new();
    for (y, row) in input.lines().enumerate() {
        for (x, tile) in row.chars().enumerate() {
            if tile == '#' {
                let elf = Elf::new(Point(x as isize, y as isize));
                elves.push(elf)
            }
        }
    }
    elves
}

fn display_elves(elves: &FxHashMap<usize, Elf>) {
    let min_x = elves.values().map(|elf| elf.position.0).min().unwrap();
    let max_x = elves.values().map(|elf| elf.position.0).max().unwrap();
    let min_y = elves.values().map(|elf| elf.position.1).min().unwrap();
    let max_y = elves.values().map(|elf| elf.position.1).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.values().any(|elf| elf.position == Point(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
