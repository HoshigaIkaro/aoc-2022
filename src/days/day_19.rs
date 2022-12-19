use std::{
    collections::{HashSet, VecDeque},
    sync::atomic::{AtomicUsize, Ordering},
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
        if self.pack.ore >= self.costs.clay {
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
}

pub struct Day19;

impl Day for Day19 {
    fn part_1(&self, input: &str) -> String {
        let mut bl = Blueprint::new(1, 4, 2, (3, 14), (2, 7));
        let mut queue = VecDeque::new();
        queue.push_back(bl);
        let max = AtomicUsize::new(0);
        let mut best = [0; 25];
        loop {
            if queue.is_empty() {
                break;
            }
            // dbg!(queue.len());
            let mut sub = Vec::new();
            for state in queue.drain(0..) {
                if state.rates.geode < best[state.minutes] {
                    continue;
                }
                best[state.minutes] = best[state.minutes].max(state.pack.geode);
                sub.push(state);
            }
            // dbg!(sub.len());
            let new: Vec<Blueprint> = sub
                .par_iter_mut()
                .flat_map(|state| {
                    if best[state.minutes] > state.pack.geode {
                        return Vec::new();
                    }
                    if state.minutes == 24 {
                        dbg!(&state);
                        max.fetch_max(state.pack.geode, Ordering::Relaxed);
                        return Vec::new();
                    }
                    let new = state.advance();
                    new
                })
                .collect();
            for state in new {
                queue.push_back(state);
            }
        }
        // dbg!(bl);
        max.load(Ordering::Acquire).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}
