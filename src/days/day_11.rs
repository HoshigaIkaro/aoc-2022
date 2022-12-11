use std::collections::{BTreeMap, VecDeque};

use super::Day;

#[derive(Debug, Clone)]
enum OpValue {
    Old,
    Lit(usize),
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
}
impl Op {
    fn new(op: &str) -> Self {
        match op {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    left: OpValue,
    op: Op,
    right: OpValue,
}
impl Operation {
    fn new(left: OpValue, op: Op, right: OpValue) -> Self {
        Self { left, op, right }
    }

    fn evaluate(&self, old: usize) -> usize {
        let left = match self.left {
            OpValue::Old => old,
            OpValue::Lit(val) => val,
        };
        let right = match self.right {
            OpValue::Old => old,
            OpValue::Lit(val) => val,
        };
        match self.op {
            Op::Add => left + right,
            Op::Mul => left * right,
        }
    }
}

enum Divisor {
    Three,
    Custom(usize),
}

#[derive(Debug, Clone)]
struct Monkey {
    number: usize,
    items: VecDeque<usize>,
    operation: Operation,
    divisor: usize,
    true_target: usize,
    false_target: usize,
    inspected: usize,
}

impl Monkey {
    fn new(
        number: usize,
        items: VecDeque<usize>,
        operation: Operation,
        divisor: usize,
        true_target: usize,
        false_target: usize,
    ) -> Self {
        Self {
            number,
            items,
            operation,
            divisor,
            true_target,
            false_target,
            inspected: 0,
        }
    }

    fn inspect(&mut self, thrown: &mut BTreeMap<usize, VecDeque<usize>>, bored: &Divisor) {
        while let Some(item) = self.items.pop_front() {
            let new = self.operation.evaluate(item);
            let new = match bored {
                Divisor::Three => new / 3,
                Divisor::Custom(d) => new % d,
            };
            if new % self.divisor == 0 {
                thrown.entry(self.true_target).or_default().push_back(new);
            } else {
                thrown.entry(self.false_target).or_default().push_back(new);
            }
            self.inspected += 1;
        }
    }
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey: {}: {:?}", self.number, self.items)
    }
}

pub struct Day11;

impl Day for Day11 {
    fn part_1(&self, input: &str) -> String {
        let mut monkeys = parse_monkeys(input);
        let mut thrown: BTreeMap<usize, VecDeque<usize>> = BTreeMap::new();
        for _round in 0..20 {
            for monkey in &mut monkeys {
                let received = thrown.entry(monkey.number).or_default();
                monkey.items.extend(received.drain(0..));
                monkey.inspect(&mut thrown, &Divisor::Three);
            }
        }
        let mut monkeys = monkeys.into_iter().collect::<Vec<_>>();
        monkeys.sort_by_key(|monkey| monkey.inspected);
        monkeys
            .into_iter()
            .rev()
            .take(2)
            .map(|monkey| monkey.inspected as u128)
            .product::<u128>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut monkeys = parse_monkeys(input);
        let gcd = monkeys
            .iter()
            .map(|m| m.divisor)
            .fold(None, |acc, new| match acc {
                None => Some(new),
                Some(old) => Some(gcd(new, old)),
            })
            .unwrap();
        let lcm = monkeys.iter().map(|m| m.divisor).product::<usize>() / gcd;
        let mut thrown: BTreeMap<usize, VecDeque<usize>> = BTreeMap::new();
        for _round in 0..10000 {
            for monkey in &mut monkeys {
                let received = thrown.entry(monkey.number).or_default();
                monkey.items.extend(received.drain(0..));
                monkey.inspect(&mut thrown, &Divisor::Custom(lcm));
            }
        }
        monkeys.sort_by_key(|monkey| monkey.inspected);
        monkeys
            .into_iter()
            .rev()
            .take(2)
            .map(|monkey| monkey.inspected)
            .product::<usize>()
            .to_string()
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|lines| {
            let mut lines = lines.lines();
            let number: usize = lines
                .next()
                .unwrap()
                .trim_start_matches("Monkey ")
                .trim_end_matches(':')
                .parse()
                .unwrap();

            let items: VecDeque<usize> = lines
                .next()
                .unwrap()
                .trim_start_matches("  Starting items: ")
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect();

            let mut operation = lines
                .next()
                .unwrap()
                .trim_start_matches("  Operation: new = ")
                .split_ascii_whitespace();
            let left = operation
                .next()
                .unwrap()
                .parse::<usize>()
                .map_or(OpValue::Old, OpValue::Lit);
            let op = Op::new(operation.next().unwrap());
            let right = operation
                .next()
                .unwrap()
                .parse::<usize>()
                .map_or(OpValue::Old, OpValue::Lit);
            let operation = Operation::new(left, op, right);

            let divisor: usize = lines
                .next()
                .unwrap()
                .trim_start_matches("  Test: divisible by ")
                .parse()
                .unwrap();

            let true_target: usize = lines
                .next()
                .unwrap()
                .trim_start_matches("    If true: throw to monkey ")
                .parse()
                .unwrap();

            let false_target: usize = lines
                .next()
                .unwrap()
                .trim_start_matches("    If false: throw to monkey ")
                .parse()
                .unwrap();

            Monkey::new(number, items, operation, divisor, true_target, false_target)
        })
        .collect()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod day_11_tests {
    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(5, 10), 5);
        assert_eq!(gcd(5, 6), 1);
    }
}
