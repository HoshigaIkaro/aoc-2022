use super::Day;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Air,
    Sand,
    Rock,
}

struct Board {
    grid: Vec<Tile>,
    count: usize,
}

impl Board {
    fn new() -> Self {
        Self {
            grid: vec![Tile::Air; WIDTH * HEIGHT],
            count: 0,
        }
    }

    fn is_occupied(&self, (x, y): (usize, usize)) -> bool {
        let tile = self.grid[y * WIDTH + x];
        tile == Tile::Rock || tile == Tile::Sand
    }

    fn set_rock(&mut self, (x, y): (usize, usize)) {
        self.grid[y * WIDTH + x] = Tile::Rock;
    }

    fn set_sand(&mut self, (x, y): (usize, usize)) {
        self.count += 1;
        self.grid[y * WIDTH + x] = Tile::Sand;
    }
}

pub struct Day14;

impl Day for Day14 {
    fn part_1(&self, input: &str) -> String {
        let (mut board, depth) = parse_input(input);
        let mut to_drop: Vec<(usize, usize)> = std::iter::once((500, 0)).collect();
        while let Some(current) = to_drop.pop() {
            if current.1 >= depth {
                break;
            }

            // straight down;
            let new = (current.0, current.1 + 1);
            if !board.is_occupied(new) {
                to_drop.push(current);
                to_drop.push(new);
                continue;
            }

            // lower left
            let new = (current.0 - 1, current.1 + 1);
            if !board.is_occupied(new) {
                to_drop.push(current);
                to_drop.push(new);
                continue;
            }

            // lower right
            let new = (current.0 + 1, current.1 + 1);
            if !board.is_occupied(new) {
                to_drop.push(current);
                to_drop.push(new);
                continue;
            }

            // no moves left
            board.set_sand(current);
        }
        board.count.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let (mut board, depth) = parse_input(input);
        let depth = depth + 2;
        let mut to_drop: Vec<(usize, usize)> = std::iter::once((500, 0)).collect();
        while let Some(current) = to_drop.pop() {
            if current.1 == depth - 1 {
                board.set_sand(current);
                continue;
            }

            // straight down;
            let new = (current.0, current.1 + 1);
            if !board.is_occupied(new) {
                to_drop.push(current);
                to_drop.push(new);
                continue;
            }

            // lower left
            let new = (current.0 - 1, current.1 + 1);
            if !board.is_occupied(new) {
                to_drop.push(current);
                to_drop.push(new);
                continue;
            }

            // lower right
            let new = (current.0 + 1, current.1 + 1);
            if !board.is_occupied(new) {
                to_drop.push(current);
                to_drop.push(new);
                continue;
            }

            board.set_sand(current);
        }
        board.count.to_string()
    }
}

fn parse_input(input: &str) -> (Board, usize) {
    let mut board = Board::new();
    let mut depth = 0;
    for line in input.lines() {
        let mut lines = line.split(" -> ").map(|p| {
            let (x, y) = p.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        });
        let mut current = lines.next().unwrap();
        for new in lines {
            depth = depth.max(new.1);
            board.set_rock(current);
            while current != new {
                if current.0 < new.0 {
                    current.0 += 1;
                } else if current.0 > new.0 {
                    current.0 -= 1;
                }
                if current.1 < new.1 {
                    current.1 += 1;
                } else if current.1 > new.1 {
                    current.1 -= 1;
                }
                board.set_rock(current);
            }
        }
    }
    (board, depth)
}
