use super::Day;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

struct Board(Vec<bool>);

impl Board {
    fn new() -> Self {
        Self(vec![false; WIDTH * HEIGHT])
    }

    fn is_occupied(&self, (x, y): (usize, usize)) -> bool {
        self.0[y * WIDTH + x]
    }

    fn set(&mut self, (x, y): (usize, usize)) {
        self.0[y * WIDTH + x] = true;
    }
}

pub struct Day14;

impl Day for Day14 {
    fn part_1(&self, input: &str) -> String {
        let (mut board, depth) = parse_input(input);
        let mut count = 0;
        'outer: loop {
            let mut current = (500, 0);

            loop {
                // println!("{current:?}");
                if current.1 >= depth {
                    break 'outer;
                }

                // straight down;
                let new = (current.0, current.1 + 1);
                if !board.is_occupied(new) {
                    current = new;
                    continue;
                }

                // lower left
                let new = (current.0 - 1, current.1 + 1);
                if !board.is_occupied(new) {
                    current = new;
                    continue;
                }

                // lower right
                let new = (current.0 + 1, current.1 + 1);
                if !board.is_occupied(new) {
                    current = new;
                    continue;
                }

                // no moves left
                board.set(current);
                break;
            }

            count += 1;
        }
        count.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let (mut board, depth) = parse_input(input);
        let depth = depth + 2;
        let mut count = 0;
        'outer: loop {
            let mut current = (500, 0);

            loop {
                if current.1 == depth - 1 {
                    board.set(current);
                    break;
                }

                // straight down;
                let new = (current.0, current.1 + 1);
                if !board.is_occupied(new) {
                    current = new;
                    continue;
                }

                // lower left
                let new = (current.0 - 1, current.1 + 1);
                if !board.is_occupied(new) {
                    current = new;
                    continue;
                }

                // lower right
                let new = (current.0 + 1, current.1 + 1);
                if !board.is_occupied(new) {
                    current = new;
                    continue;
                }
                // no moves left
                if current == (500, 0) {
                    count += 1;
                    break 'outer;
                }
                board.set(current);
                break;
            }
            count += 1;
        }
        count.to_string()
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
            board.set(current);
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
                board.set(current);
            }
        }
    }
    (board, depth)
}
