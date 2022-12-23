use std::ops::Add;

use rustc_hash::FxHashMap;

use super::Day;

type Id = usize;

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
    fn deltas_to_check(&self) -> [Point; 3] {
        match self {
            Direction::North => [Point(0, -1), Point(1, -1), Point(-1, -1)],
            Direction::South => [Point(0, 1), Point(1, 1), Point(-1, 1)],
            Direction::West => [Point(-1, 0), Point(-1, 1), Point(-1, -1)],
            Direction::East => [Point(1, 0), Point(1, 1), Point(1, -1)],
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
    id: usize,
    position: Point,
    directions: [Direction; 4],
}

impl Elf {
    fn new(id: Id, position: Point) -> Self {
        Self {
            id,
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
            Point(0, 1),   // S
            Point(1, 1),   // SE
            Point(1, 0),   // E
            Point(1, -1),  // NE
            Point(0, -1),  // N
            Point(-1, -1), // NW
            Point(-1, 0),  // W
            Point(-1, 1),  // SW
        ]
    }

    fn any_elves_around(&self, elves: &FxHashMap<Id, Elf>) -> bool {
        self.deltas_around().into_iter().any(|delta| {
            let new = self.position + delta;
            elves.values().any(|elf| elf.position == new)
        })
    }

    fn propose(&self, elves: &FxHashMap<Id, Elf>, proposed: &mut FxHashMap<Point, Vec<Id>>, start_index:usize) {
        if self.any_elves_around(elves) {
            for index in 0..4 {
                let new_index = (start_index + index) % 4;
                let direction = self.directions[new_index];
                if direction.deltas_to_check().into_iter().all(|delta| {
                    let new = self.position + delta;
                    elves.values().all(|elf| elf.position != new)
                }) {
                    // can propose
                    let new = direction.new_point(self.position);
                    proposed.entry(new).or_default().push(self.id);
                    break;
                }
            }
        }
    }
}

pub struct Day23;

impl Day for Day23 {
    fn part_1(&self, input: &str) -> String {
        let mut elves = parse_elves(input);
        for round in 0..10 {
            // first half
            let mut proposed = FxHashMap::default();
            for elf in elves.values() {
                elf.propose(&elves, &mut proposed, round % 4);
            }

            // second half
            for (point, elf_ids) in proposed {
                if let [elf_id] = elf_ids.as_slice() {
                    elves.get_mut(elf_id).unwrap().position = point
                }
            }
        }
        // display_elves(&elves);
        let min_x = elves.values().map(|elf| elf.position.0).min().unwrap();
        let max_x = elves.values().map(|elf| elf.position.0).max().unwrap();
        let min_y = elves.values().map(|elf| elf.position.1).min().unwrap();
        let max_y = elves.values().map(|elf| elf.position.1).max().unwrap();

        let width = min_x.abs_diff(max_x) + 1;
        let length = min_y.abs_diff(max_y) + 1;
        let area = width * length;

        let empty = area - elves.len();
        empty.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut elves = parse_elves(input);
        let mut round = 1;
        let first = loop {
            // first half
            let mut proposed = FxHashMap::default();
            for elf in elves.values() {
                elf.propose(&elves, &mut proposed,round % 4);
            }

            if proposed.is_empty() {
                break round;
            }

            // second half
            for (point, elf_ids) in proposed {
                if let [elf_id] = elf_ids.as_slice() {
                    elves.get_mut(elf_id).unwrap().position = point
                }
            }
            round += 1;
        };
        first.to_string()
    }
}

fn parse_elves(input: &str) -> FxHashMap<Id, Elf> {
    let mut elves = FxHashMap::default();
    let mut id = 0;
    for (y, row) in input.lines().enumerate() {
        for (x, tile) in row.chars().enumerate() {
            if tile == '#' {
                let elf = Elf::new(id, Point(x as isize, y as isize));
                elves.insert(id, elf);
                id += 1;
            }
        }
    }
    elves
}

fn display_elves(elves: &FxHashMap<Id, Elf>) {
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
