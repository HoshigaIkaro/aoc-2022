use super::Day;

#[derive(Debug)]
enum Op {
    Noop,
    Add(isize),
}

pub struct Day10;

impl Day for Day10 {
    fn part_1(&self, input: &str) -> String {
        let mut cycle = 0;
        let mut x = 1;
        let mut ops = input.lines().map(|line| match line {
            "noop" => Op::Noop,
            _ => Op::Add(line.split(' ').skip(1).next().unwrap().parse().unwrap()),
        });
        let mut out: isize = 0;
        let mut counter = 0;
        let mut buf = None;
        loop {
            // println!("B | {} {} {} {counter}", cycle, x, cycle * x);
            if counter > 0 {
                counter -= 1;
            } else if let Some(v) = buf {
                x += v;
                buf = None;
                counter = 0;
                match ops.next() {
                    Some(op) => match op {
                        Op::Noop => (),
                        Op::Add(v) => {
                            buf = Some(v);
                            counter = 1
                        }
                    },
                    None => break,
                }
            } else {
                match ops.next() {
                    Some(op) => match op {
                        Op::Noop => (),
                        Op::Add(v) => {
                            buf = Some(v);
                            counter = 1
                        }
                    },
                    None => break,
                }
            }
            // println!("A | {} {} {} {counter}\n", cycle, x, cycle * x);
            cycle += 1;

            if (cycle - 20) % 40 == 0 {
                out += cycle as isize * x;
            }
        }
        println!("{} {} {}", cycle, x, cycle * x);
        out.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut x = 1;
        let mut ops = input.lines().map(|line| match line {
            "noop" => Op::Noop,
            _ => Op::Add(line.split(' ').skip(1).next().unwrap().parse().unwrap()),
        });
        let mut counter = 0;
        let mut buf = None;
        for _ in 0..6 {
            for cycle in 0..40 {
                // println!("B | {} {} {} {counter}", cycle, x, cycle * x);
                if counter > 0 {
                    counter -= 1;
                } else if let Some(v) = buf {
                    x += v;
                    buf = None;
                    counter = 0;
                    match ops.next() {
                        Some(op) => match op {
                            Op::Noop => (),
                            Op::Add(v) => {
                                buf = Some(v);
                                counter = 1
                            }
                        },
                        None => break,
                    }
                } else {
                    match ops.next() {
                        Some(op) => match op {
                            Op::Noop => (),
                            Op::Add(v) => {
                                buf = Some(v);
                                counter = 1
                            }
                        },
                        None => break,
                    }
                }
                // println!("A | {} {} {} {counter}\n", cycle, x, cycle * x);

                if x.abs_diff(cycle) <= 1 {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
        todo!()
    }
}
