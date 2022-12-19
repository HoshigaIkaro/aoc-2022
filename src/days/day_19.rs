use std::{
    collections::{HashSet, VecDeque},
    sync::atomic::{AtomicUsize, Ordering}, io::Write,
};

use rayon::prelude::*;

use super::Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Costs {
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
}

impl Costs {
    fn new(ore: usize, clay: usize, obsidian: (usize, usize), geode: (usize, usize)) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rates {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Rates {
    fn new() -> Self {
        Self {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
struct Pack {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Action {
    Ore(usize),
    Clay(usize),
    Obsidian(usize),
    Geode(usize),
    Wait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Blueprint {
    id: usize,
    costs: Costs,
    rates: Rates,
    pack: Pack,
    minutes: usize,
    action: Action,
}

impl Blueprint {
    fn new(
        id: usize,
        ore_cost: usize,
        clay_cost: usize,
        obsidian_cost: (usize, usize),
        geode_cost: (usize, usize),
    ) -> Self {
        Self {
            id,
            costs: Costs::new(ore_cost, clay_cost, obsidian_cost, geode_cost),
            rates: Rates::new(),
            pack: Pack::default(),
            minutes: 0,
            action: Action::Wait,
        }
    }

    fn add(&mut self) {
        self.pack.ore += self.rates.ore;
        self.pack.clay += self.rates.clay;
        self.pack.obsidian += self.rates.obsidian;
        self.pack.geode += self.rates.geode;
    }

    fn advance(&mut self) -> Vec<Self> {
        let mut states = Vec::new();
        states.push(self.clone());
        if self.pack.ore >= self.costs.geode.0 && self.pack.obsidian >= self.costs.geode.1 {
            let mut state = self.clone();
            state.pack.ore -= self.costs.geode.0;
            state.pack.obsidian -= self.costs.geode.1;
            state.action = Action::Geode(1);
            states.push(state);
        }
        if self.pack.ore >= self.costs.obsidian.0 && self.pack.clay >= self.costs.obsidian.1 {
            let mut state = self.clone();
            state.pack.ore -= self.costs.obsidian.0;
            state.pack.clay -= self.costs.obsidian.1;
            state.action = Action::Obsidian(1);
            states.push(state);
        }
        if self.pack.ore >= self.costs.clay
            && (self.rates.clay / self.rates.ore < self.costs.obsidian.1 / self.costs.obsidian.0)
        {
            let mut state = self.clone();
            state.pack.ore -= self.costs.clay;
            state.action = Action::Clay(1);
            states.push(state);
        }
        if self.pack.ore >= 4 {
            let mut state = self.clone();
            state.pack.ore -= self.costs.ore;
            state.action = Action::Ore(1);
            states.push(state);
        }

        states
            .iter_mut()
            .map(|state| {
                state.add();
                match state.action {
                    Action::Ore(_) => state.rates.ore += 1,
                    Action::Clay(_) => state.rates.clay += 1,
                    Action::Obsidian(_) => state.rates.obsidian += 1,
                    Action::Geode(_) => state.rates.geode += 1,
                    Action::Wait => (),
                }
                state.action = Action::Wait;
                state.minutes += 1;
                *state
            })
            .collect()
    }

    fn score(&self) -> usize {
        (self.rates.ore + self.rates.clay * 4 + self.rates.obsidian * 32 + self.rates.geode * 64)
            + (24 - self.minutes)
                * (self.rates.ore
                    + self.rates.clay * 4
                    + self.rates.obsidian * 16
                    + self.rates.geode * 64)
    }
}

pub struct Day19;

impl Day for Day19 {
    fn part_1(&self, input: &str) -> String {
        let bl = Blueprint::new(1, 4, 2, (3, 14), (2, 7));
        let mut queue = VecDeque::new();
        queue.push_back(bl);
        let mut max_geode = 0;
        let mut best = [0; 24];
        let mut i = 0;
        let mut log_file = std::fs::File::create("log.log").unwrap();
        loop {
            dbg!(i);
            if queue.is_empty() {
                break;
            }
            let mut sub = Vec::new();
            for state in queue.drain(0..) {
                if state.score() < best[state.minutes] {
                    continue;
                }
                best[state.minutes] = best[state.minutes].max(state.score());
                sub.push(state);
            }
            let new: Vec<Blueprint> = sub
                .par_iter_mut()
                .flat_map(|state| {
                    if state.minutes == 24 {
                        vec![*state]
                    } else {
                        state.advance()
                    }
                })
                .collect();
            for state in new {
                max_geode = state.pack.geode.max(max_geode);
                // if state.rates.geode == 1 {
                //     dbg!(state.score());
                // }
                if state.minutes < 24 {
                    queue.push_back(state);
                } else {
                    writeln!(log_file, "{:?}", state).unwrap();
                }
            }
            i += 1;
        }
        dbg!(best);
        max_geode.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}
