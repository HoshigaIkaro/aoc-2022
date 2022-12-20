use std::{
    collections::VecDeque,
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
struct Blueprint<const R: usize> {
    id: usize,
    costs: Costs,
    rates: Rates,
    pack: Pack,
    minutes: usize,
    action: Action,
}

impl<const R: usize> Blueprint<R> {
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
        // println!("{:?}", &self);

        if self.pack.ore >= self.costs.geode.0 && self.pack.obsidian >= self.costs.geode.1
        // && self.minutes > R / 2
        {
            // dbg!(self.minutes);
            let mut state = self.clone();
            state.pack.ore -= self.costs.geode.0;
            state.pack.obsidian -= self.costs.geode.1;
            state.action = Action::Geode;
            states.push(state);
        }
        if self.pack.ore >= self.costs.obsidian.0
            && self.pack.clay >= self.costs.obsidian.1
            && self.rates.obsidian < self.costs.geode.1
        {
            let mut state = self.clone();
            state.pack.ore -= self.costs.obsidian.0;
            state.pack.clay -= self.costs.obsidian.1;
            state.action = Action::Obsidian;
            states.push(state);
        }
        if self.pack.ore >= self.costs.clay && self.rates.clay < self.costs.obsidian.1
        // && self.minutes < R / 2 + 6
        {
            let mut state = self.clone();
            state.pack.ore -= self.costs.clay;
            state.action = Action::Clay;
            states.push(state);
        }
        if self.pack.ore >= self.costs.ore && self.rates.ore < self.max_ore_cost() {
            let mut state = self.clone();
            state.pack.ore -= self.costs.ore;
            state.action = Action::Ore;
            states.push(state);
        }

        if self.pack.ore < 5 || states.is_empty() {
            states.push(self.clone());
        }
        // println!("{:?}", states);

        states
            .into_iter()
            .map(|mut state| {
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
                state
            })
            .collect()
    }

    fn max_ore_cost(&self) -> usize {
        self.costs.ore.max(
            self.costs
                .clay
                .max(self.costs.obsidian.0.max(self.costs.geode.0)),
        )
    }

    fn score(&self) -> usize {
        // (self.rates.ore + self.rates.clay * 4 +
        self.pack.ore
            + (self.pack.clay + self.pack.obsidian * self.costs.obsidian.1) * self.costs.clay
            + self.pack.obsidian * self.costs.obsidian.0
            + self.pack.geode * self.costs.geode.0
        // self.pack.geode + self.rates.geode * (R - self.minutes)
    }

    fn score_obsidian(&self) -> usize {
        self.pack.obsidian + self.rates.obsidian * (R - self.minutes)
    }
}

pub struct Day19;

impl Day for Day19 {
    fn part_1(&self, input: &str) -> String {
        // return "".to_string();
        let blueprints = parse_blueprints::<24>(input);
        let total: usize = blueprints
            .into_par_iter()
            .map(|blueprint| {
                println!(
                    "On blueprint {} -----------------------------------",
                    blueprint.id
                );
                let mut queue = VecDeque::new();
                queue.push_back(blueprint);
                let max_geode = AtomicUsize::new(0);
                let best = AtomicUsize::new(0);
                let best_obsidian = AtomicUsize::new(0);
                // let mut log_file = std::fs::File::create("log.log").unwrap();
                loop {
                    // dbg!(i);
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
                                if state.minutes < 24 / 2 + 2
                                    && state.score_obsidian()
                                        >= best_obsidian.load(Ordering::Relaxed)
                                {
                                    best_obsidian.store(state.score_obsidian(), Ordering::Relaxed);
                                    Some(state)
                                } else if state.score() >= best.load(Ordering::Relaxed) {
                                    best.store(state.score(), Ordering::Relaxed);
                                    Some(state)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        });
                    queue.par_extend(new);
                }
                dbg!(best, &max_geode);
                let quality = blueprint.id * max_geode.load(Ordering::Acquire);
                quality
            })
            .sum();
        total.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        // todo!();
        let blueprints = parse_blueprints::<32>(input);
        let total: usize = blueprints
            .into_iter()
            .take(3)
            .map(|blueprint| {
                println!(
                    "On blueprint {} -----------------------------------",
                    blueprint.id
                );
                let mut queue = VecDeque::new();
                queue.push_back(blueprint);
                let max_geode = AtomicUsize::new(0);
                let best = AtomicUsize::new(0);
                let best_obsidian = AtomicUsize::new(0);
                // let mut log_file = std::fs::File::create("log.log").unwrap();
                loop {
                    // dbg!(i);
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
                            if state.minutes < 32 {
                                if state.minutes < 32 / 2
                                    && state.score_obsidian()
                                        >= best_obsidian.load(Ordering::Relaxed)
                                {
                                    best_obsidian.store(state.score_obsidian(), Ordering::Relaxed);
                                    Some(state)
                                } else if state.score() >= best.load(Ordering::Relaxed) {
                                    best.store(state.score(), Ordering::Relaxed);
                                    Some(state)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        });
                    queue.par_extend(new);
                }
                dbg!(best, &max_geode);
                max_geode.load(Ordering::Acquire)
            })
            .product();
        total.to_string()
    }
}

fn parse_blueprints<const R: usize>(input: &str) -> Vec<Blueprint<R>> {
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
            // dbg!(clay);
            let obsidian = (numbers.next().unwrap(), numbers.next().unwrap());
            // dbg!(obsidian);
            let geode = (numbers.next().unwrap(), numbers.next().unwrap());
            Blueprint::new(id, ore, clay, obsidian, geode)
        })
        .collect()
}
