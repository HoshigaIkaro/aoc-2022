use std::{collections::BinaryHeap, fmt::Display};

#[cfg(feature = "visualize")]
use crossterm::{cursor, execute, terminal, ExecutableCommand};
#[cfg(feature = "visualize")]
use std::io::{stdout, Write};

use super::Day;

const MAX_STATES: usize = 50;

type Point = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Direction::Up => "^",
            Direction::Right => ">",
            Direction::Down => "v",
            Direction::Left => "<",
        };
        write!(f, "{out}")
    }
}

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    point: Point,
    direction: Direction,
}

impl Blizzard {
    fn new(point: Point, direction: Direction) -> Self {
        Self { point, direction }
    }
}

#[derive(Debug)]
struct Valley {
    blizzards: Vec<Blizzard>,
    /// width: x-axis
    width: usize,
    /// height/length: y-axis
    height: usize,
}

impl Valley {
    fn new(input: &str) -> Self {
        let mut blizzards = Vec::new();
        let mut height = 0;
        let mut total_tiles = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                match tile {
                    '^' => blizzards.push(Blizzard::new((x, y), Direction::Up)),
                    '>' => blizzards.push(Blizzard::new((x, y), Direction::Right)),
                    'v' => blizzards.push(Blizzard::new((x, y), Direction::Down)),
                    '<' => blizzards.push(Blizzard::new((x, y), Direction::Left)),
                    _ => (),
                }
            }
            total_tiles += line.len();
            height += 1;
        }
        let width = total_tiles / height;
        Self {
            blizzards,
            width,
            height,
        }
    }

    fn simulate_next(&mut self) {
        for blizzard in self.blizzards.iter_mut() {
            match blizzard.direction {
                Direction::Up => {
                    blizzard.point.1 -= 1;
                    if blizzard.point.1 == 0 {
                        blizzard.point.1 = self.height - 2;
                    }
                }
                Direction::Right => {
                    blizzard.point.0 += 1;
                    if blizzard.point.0 == self.width - 1 {
                        blizzard.point.0 = 1;
                    }
                }
                Direction::Down => {
                    blizzard.point.1 += 1;
                    if blizzard.point.1 == self.height - 1 {
                        blizzard.point.1 = 1;
                    }
                }
                Direction::Left => {
                    blizzard.point.0 -= 1;
                    if blizzard.point.0 == 0 {
                        blizzard.point.0 = self.width - 2;
                    }
                }
            }
        }
    }

    fn get_surrounding_points(&self, (x, y): Point) -> Vec<Point> {
        if y == 0 && x == 1 {
            // start
            vec![(1, 1)]
        } else if y == self.height - 1 && x == self.width - 2 {
            // end
            vec![(self.width - 2, self.height - 2)]
        } else {
            // in the valley
            let mut points = Vec::new();
            if y > 1 || x == 1 {
                points.push((x, y - 1));
            }
            if y < self.height - 2 || x == self.width - 2 {
                points.push((x, y + 1));
            }
            if x > 1 {
                points.push((x - 1, y));
            }
            if x < self.width - 2 {
                points.push((x + 1, y));
            }
            points
        }
    }

    /// Called after blizzards move
    fn valid_moves_at(&self, point: Point) -> Vec<Point> {
        self.get_surrounding_points(point)
            .into_iter()
            .chain(vec![point])
            .filter(|point| self.blizzards.iter().all(|b| b.point != *point))
            .collect()
    }

    fn start(&self) -> Point {
        (1, 0)
    }

    fn end(&self) -> Point {
        (self.width - 2, self.height - 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    point: Point,
    target: Point,
}

impl State {
    fn new(point: Point, target: Point) -> Self {
        Self { point, target }
    }

    fn min_minutes_remaining(&self) -> usize {
        self.point.0.abs_diff(self.target.0) + self.point.1.abs_diff(self.target.1)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .min_minutes_remaining()
            .cmp(&self.min_minutes_remaining())
    }
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.width {
            let tile = if x == 1 { "." } else { "#" };
            write!(f, "{tile}")?;
        }
        writeln!(f)?;
        for y in 1..self.height - 1 {
            write!(f, "#")?;
            for x in 1..self.width - 1 {
                let occupying: Vec<&Blizzard> = self
                    .blizzards
                    .iter()
                    .filter(|b| b.point == (x, y))
                    .collect();
                let tile = if occupying.is_empty() {
                    String::from(".")
                } else if let [blizzard] = occupying.as_slice() {
                    blizzard.direction.to_string()
                } else {
                    occupying.len().to_string()
                };
                write!(f, "{tile}")?;
            }
            writeln!(f, "#")?;
        }
        for x in 0..self.width {
            let tile = if x == self.width - 2 { "." } else { "#" };
            write!(f, "{tile}")?;
        }
        writeln!(f)?;
        Ok(())
    }
}

#[cfg(not(feature = "visualize"))]
fn traverse(start: Point, target: Point, valley: &mut Valley) -> usize {
    let mut minutes = 0;
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State::new(start, target));
    'outer: loop {
        valley.simulate_next();
        let mut best_states = Vec::new();
        while best_states.len() < MAX_STATES && !queue.is_empty() {
            let state = queue.pop().unwrap();
            if state.target == state.point {
                break 'outer;
            }
            if best_states.contains(&state) {
                continue;
            }
            best_states.push(state);
        }
        if best_states.is_empty() {
            break;
        }
        queue.clear();
        for state in best_states {
            for new_point in valley.valid_moves_at(state.point) {
                let mut state = state;
                state.point = new_point;
                queue.push(state);
            }
        }
        minutes += 1;
    }
    minutes
}

pub struct Day24;

impl Day for Day24 {
    fn part_1(&self, input: &str) -> String {
        #[cfg(feature = "visualize")]
        return String::new();
        let mut valley = Valley::new(input);
        traverse(valley.start(), valley.end(), &mut valley).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        #[cfg(feature = "visualize")]
        {
            let mut stdout = stdout();
            execute!(
                stdout,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveToColumn(0),
                cursor::MoveToRow(0),
                cursor::Hide
            )
            .unwrap();
        }

        let mut valley = Valley::new(input);
        let there = traverse(valley.start(), valley.end(), &mut valley);
        let back = 1 + traverse(valley.end(), valley.start(), &mut valley);
        let there_again = 1 + traverse(valley.start(), valley.end(), &mut valley);
        let total = there + back + there_again;
        total.to_string()
    }
}

#[cfg(feature = "visualize")]
fn display_all(valley: &Valley, occupied: &[Point]) {
    use owo_colors::OwoColorize;

    let s = valley.to_string();
    for (y, line) in s.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            if occupied.contains(&(x, y)) {
                print!("{}", "█".fg_rgb::<186, 187, 241>());
            } else {
                print!("{tile}");
            }
        }
        println!();
    }
}

#[cfg(feature = "visualize")]
fn traverse(start: Point, target: Point, valley: &mut Valley) -> usize {
    let mut stdout = stdout();
    let mut minutes = 0;
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State::new(start, target));
    'outer: loop {
        execute!(stdout, cursor::MoveToColumn(0), cursor::MoveToRow(0)).unwrap();
        display_all(&valley, &queue.iter().map(|s| s.point).collect::<Vec<_>>());
        valley.simulate_next();
        let mut best_states = Vec::new();
        while best_states.len() < MAX_STATES && !queue.is_empty() {
            let state = queue.pop().unwrap();
            if state.target == state.point {
                break 'outer;
            }
            if best_states.contains(&state) {
                continue;
            }
            best_states.push(state);
        }
        if best_states.is_empty() {
            break;
        }
        queue.clear();
        for state in best_states {
            for new_point in valley.valid_moves_at(state.point) {
                let mut state = state;
                state.point = new_point;
                queue.push(state);
            }
        }
        minutes += 1;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    execute!(stdout, cursor::MoveToColumn(0), cursor::MoveToRow(0)).unwrap();
    display_all(&valley, &[target]);
    std::thread::sleep(std::time::Duration::from_millis(500));
    minutes
}
