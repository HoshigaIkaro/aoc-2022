use std::{
    collections::{HashSet, VecDeque},
    io::Write,
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
    Ore,
    Clay,
    Obsidian,
    Geode,
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
        if self.minutes != 24 {
            if self.pack.ore >= self.costs.geode.0 && self.pack.obsidian >= self.costs.geode.1 {
                let mut state = self.clone();
                state.pack.ore -= self.costs.geode.0;
                state.pack.obsidian -= self.costs.geode.1;
                state.action = Action::Geode;
                states.push(state);
            }
            if self.pack.ore >= self.costs.obsidian.0
                && self.pack.clay >= self.costs.obsidian.1
                && (self.rates.obsidian / self.rates.ore < self.costs.geode.1 / self.costs.geode.0)
                && self.rates.obsidian < self.costs.geode.1
            // && self.rates.obsidian < self.costs.geode.1
            {
                let mut state = self.clone();
                state.pack.ore -= self.costs.obsidian.0;
                state.pack.clay -= self.costs.obsidian.1;
                state.action = Action::Obsidian;
                states.push(state);
            }
            if self.pack.ore >= self.costs.clay
                && (self.rates.clay / self.rates.ore
                    < self.costs.obsidian.1 / self.costs.obsidian.0)
                && self.rates.clay < self.costs.obsidian.1
            {
                let mut state = self.clone();
                state.pack.ore -= self.costs.clay;
                state.action = Action::Clay;
                states.push(state);
            }
            if self.pack.ore >= self.costs.ore && self.rates.ore < 4 {
                let mut state = self.clone();
                state.pack.ore -= self.costs.ore;
                state.action = Action::Ore;
                states.push(state);
            }
        }
        states
            .par_iter_mut()
            .map(|state| {
                state.add();
                match state.action {
                    Action::Ore => state.rates.ore += 1,
                    Action::Clay => state.rates.clay += 1,
                    Action::Obsidian => state.rates.obsidian += 1,
                    Action::Geode => state.rates.geode += 1,
                    Action::Wait => (),
                }
                state.action = Action::Wait;
                state.minutes += 1;
                *state
            })
            .collect()
    }

    fn score(&self) -> usize {
        // (self.rates.ore + self.rates.clay * 4 +
        self.pack.geode + self.rates.geode * (24 - self.minutes)
    }
}

pub struct Day19;

impl Day for Day19 {
    fn part_1(&self, input: &str) -> String {
        let blueprints = parse_blueprints(input);
        let total: usize = blueprints
            .into_iter()
            .map(|blueprint| {
                println!(
                    "On blueprint {} -----------------------------------",
                    blueprint.id
                );
                let mut queue = VecDeque::new();
                queue.push_back(blueprint);
                let max_geode = AtomicUsize::new(0);
                let best = AtomicUsize::new(0);
                let mut i = 0;
                // let mut log_file = std::fs::File::create("log.log").unwrap();
                loop {
                    dbg!(i);
                    dbg!(queue.len());
                    if queue.is_empty() {
                        break;
                    }
                    let new = queue.split_off(0);
                    let new = new
                        .into_par_iter()
                        .flat_map(|mut state| state.advance())
                        .into_par_iter()
                        .filter_map(|state| {
                            max_geode.fetch_max(state.pack.geode, Ordering::Relaxed);
                            if state.minutes < 24 {
                                let c_best = best.load(Ordering::Relaxed);
                                let score = state.score();
                                if score > c_best {
                                    best.store(state.score(), Ordering::Relaxed);
                                    Some(state)
                                } else if c_best == score {
                                    Some(state)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        });
                    queue.par_extend(new);

                    i += 1;
                }
                dbg!(best, &max_geode);
                let quality = blueprint.id * max_geode.load(Ordering::Acquire);
                quality
            })
            .sum();
        total.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let (id, rest) = line
                .strip_prefix("Blueprint ")
                .unwrap()
                .split_once(": ")
                .unwrap();
            let id = id.parse().unwrap();
            let mut numbers = rest
                .split_whitespace()
                .filter(|c| c.chars().all(|d| d.is_digit(10)))
                .map(|s| usize::from_str_radix(s, 10).unwrap());
            let ore = numbers.next().unwrap();
            let clay = numbers.next().unwrap();
            dbg!(clay);
            let obsidian = (numbers.next().unwrap(), numbers.next().unwrap());
            dbg!(obsidian);
            let geode = (numbers.next().unwrap(), numbers.next().unwrap());
            Blueprint::new(id, ore, clay, obsidian, geode)
        })
        .collect()
}
