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
    op: Op,
    right: OpValue,
}
impl Operation {
    fn new(op: Op, right: OpValue) -> Self {
        Self { op, right }
    }

    fn evaluate(&self, old: usize) -> usize {
        let right = match self.right {
            OpValue::Old => old,
            OpValue::Lit(val) => val,
        };
        match self.op {
            Op::Add => old + right,
            Op::Mul => old * right,
        }
    }
}

enum Manage {
    DivThree,
    ModLCM(usize),
}

#[derive(Debug, Clone)]
struct Monkey {
    number: usize,
    items: Vec<usize>,
    operation: Operation,
    divisor: usize,
    true_target: usize,
    false_target: usize,
    inspected: usize,
}

impl Monkey {
    fn new(
        number: usize,
        items: Vec<usize>,
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

    fn inspect(&mut self, thrown: &mut [Vec<usize>], bored: &Manage) {
        self.inspected += self.items.len();
        for item in self.items.drain(0..) {
            let new = self.operation.evaluate(item);
            let new = match bored {
                Manage::DivThree => new / 3,
                Manage::ModLCM(d) => new % d,
            };
            if new % self.divisor == 0 {
                thrown.get_mut(self.true_target).unwrap().push(new);
            } else {
                thrown.get_mut(self.false_target).unwrap().push(new);
            }
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
        simulate::<20>(&mut monkeys, &Manage::DivThree);
        monkeys.sort_by_key(|monkey| monkey.inspected);
        monkeys
            .into_iter()
            .rev()
            .take(2)
            .map(|monkey| monkey.inspected)
            .product::<usize>()
            .to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut monkeys = parse_monkeys(input);
        let lcm = monkeys.iter().map(|m| m.divisor).product::<usize>();
        simulate::<10_000>(&mut monkeys, &Manage::ModLCM(lcm));
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
                .strip_prefix("Monkey ")
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse()
                .unwrap();

            let items: Vec<usize> = lines
                .next()
                .unwrap()
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect();

            let mut operation = lines
                .next()
                .unwrap()
                .strip_prefix("  Operation: new = old ")
                .unwrap()
                .split_ascii_whitespace();
            let op = Op::new(operation.next().unwrap());
            let right = operation
                .next()
                .unwrap()
                .parse::<usize>()
                .map_or(OpValue::Old, OpValue::Lit);
            let operation = Operation::new(op, right);

            let divisor: usize = lines
                .next()
                .unwrap()
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();

            let true_target: usize = lines
                .next()
                .unwrap()
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            let false_target: usize = lines
                .next()
                .unwrap()
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            Monkey::new(number, items, operation, divisor, true_target, false_target)
        })
        .collect()
}

fn simulate<const R: usize>(monkeys: &mut [Monkey], manage: &Manage) {
    let mut thrown: Vec<Vec<usize>> = vec![Vec::new(); monkeys.len()];
    for _ in 0..R {
        for monkey in &mut *monkeys {
            let received = thrown.get_mut(monkey.number).unwrap();
            monkey.items.extend(received.drain(0..));
            monkey.inspect(&mut thrown, manage);
        }
    }
}
