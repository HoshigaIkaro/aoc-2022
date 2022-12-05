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
            let mut v = Vec::new();
            for _ in 0..num {
                v.push(cmap.get_mut(origin - 1).unwrap().pop().unwrap());
            }
            cmap.get_mut(target - 1)
                .unwrap()
                .extend(v.into_iter().rev());
        }
        cmap.iter().map(|v| v.last().unwrap()).collect()
    }
}

fn get_crates(input: &str) -> Vec<Vec<char>> {
    let mut cmap: Vec<Vec<char>> = vec![Vec::new(); 9];
    for line in input.lines() {
        if line.contains('1') {
            break;
        }
        let line = line.replace(" [", "@[");
        let line = line.replace("] ", "]@");
        let line = line.replace("    ", "   @");
        for (i, c) in line.split('@').enumerate() {
            if c.starts_with('[') {
                cmap[i].insert(0, c.chars().nth(1).unwrap());
            }
        }
    }
    cmap
}

fn parse_instruction(line: &str) -> (usize, usize, usize) {
    let mut s = line.split_whitespace();
    s.next();
    let num = s.next().unwrap().parse().unwrap();
    s.next();
    let origin = s.next().unwrap().parse().unwrap();
    s.next();
    let target = s.next().unwrap().parse().unwrap();
    (num, origin, target)
}
