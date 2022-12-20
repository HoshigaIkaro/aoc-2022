use std::{
    collections::{BinaryHeap, VecDeque},
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

    fn obsidian_ore_cost(&self) -> usize {
        self.obsidian.0 + self.obsidian.1 * self.clay
    }

    fn geode_ore_cost(&self) -> usize {
        self.geode.0 + self.geode.1 * self.obsidian_ore_cost()
    }

    fn clay_ore_cost(&self) -> usize {
        self.clay
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    // ████████████████████████████████████████████████████████████████████████
    fn can_buy_geode(&self) -> bool {
        self.pack.ore >= self.costs.geode.0 && self.pack.obsidian >= self.costs.geode.1
    }
    fn can_buy_obsidian(&self) -> bool {
        self.pack.ore >= self.costs.obsidian.0 && self.pack.clay >= self.costs.obsidian.1
    }
    fn can_buy_clay(&self) -> bool {
        self.pack.ore >= self.costs.clay
    }
    fn can_buy_ore(&self) -> bool {
        self.pack.ore >= self.costs.ore
    }
    // ████████████████████████████████████████████████████████████████████████

    fn advance(&mut self) -> Vec<Self> {
        let mut states = Vec::new();

        if self.can_buy_geode() {
            let mut state = self.clone();
            state.pack.ore -= self.costs.geode.0;
            state.pack.obsidian -= self.costs.geode.1;
            state.action = Action::Geode;
            states.push(state);
        }
        if self.can_buy_obsidian() && self.rates.obsidian < self.costs.geode.1 {
            let mut state = self.clone();
            state.pack.ore -= self.costs.obsidian.0;
            state.pack.clay -= self.costs.obsidian.1;
            state.action = Action::Obsidian;
            states.push(state);
        }
        if self.can_buy_clay() && self.rates.clay < self.costs.obsidian.1 {
            let mut state = self.clone();
            state.pack.ore -= self.costs.clay;
            state.action = Action::Clay;
            states.push(state);
        }
        if self.can_buy_ore() && self.rates.ore < self.max_ore_cost() {
            let mut state = self.clone();
            state.pack.ore -= self.costs.ore;
            state.action = Action::Ore;
            states.push(state);
        }

        if states.len() != 4 {
            states.push(self.clone());
        }

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

    fn minutes_left(&self) -> usize {
        R - self.minutes
    }

    fn possible_geode(&self) -> usize {
        self.pack.geode + self.rates.geode * self.minutes_left()
    }
    fn possible_obsidian(&self) -> usize {
        self.pack.obsidian + self.rates.obsidian * self.minutes_left()
    }
    fn possible_clay(&self) -> usize {
        self.pack.clay + self.rates.clay * self.minutes_left()
    }
    fn possible_ore(&self) -> usize {
        self.pack.ore + self.rates.ore * self.minutes_left()
    }

    fn score(&self) -> usize {
        self.possible_ore()
            + self.possible_clay() * self.costs.clay_ore_cost()
            + self.possible_obsidian() * self.costs.obsidian_ore_cost()
            + self.possible_geode() * self.costs.geode_ore_cost()
    }
}

impl<const R: usize> PartialOrd for Blueprint<R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const R: usize> Ord for Blueprint<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

pub struct Day19;

impl Day for Day19 {
    fn part_1(&self, input: &str) -> String {
        let blueprints = parse_blueprints::<24>(input);
        let total: usize = blueprints
            .into_par_iter()
            .map(|blueprint| {
                let mut queue = BinaryHeap::new();
                queue.push(blueprint);
                let max_geode = AtomicUsize::new(0);
                loop {
                    if queue.is_empty() {
                        break;
                    }
                    let mut new = Vec::new();
                    for _ in 0..queue.len().min(500) {
                        let state = queue.pop().unwrap();
                        new.push(state);
                    }
                    queue.clear();
                    let new = new
                        .into_par_iter()
                        .flat_map(|mut state| state.advance())
                        .into_par_iter()
                        .filter_map(|state| {
                            if state.minutes < 24 {
                                Some(state)
                            } else {
                                max_geode.fetch_max(state.pack.geode, Ordering::Relaxed);
                                None
                            }
                        });
                    queue.par_extend(new);
                }
                let quality = blueprint.id * max_geode.load(Ordering::Acquire);
                quality
            })
            .sum();
        total.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let blueprints = parse_blueprints::<32>(input);
        let total: usize = blueprints
            .into_iter()
            .take(3)
            .map(|blueprint| {
                let mut queue = BinaryHeap::new();
                queue.push(blueprint);
                let max_geode = AtomicUsize::new(0);
                loop {
                    if queue.is_empty() {
                        break;
                    }
                    let mut new = Vec::new();
                    for _ in 0..queue.len().min(100) {
                        let state = queue.pop().unwrap();
                        new.push(state);
                    }
                    queue.clear();
                    let new = new
                        .into_par_iter()
                        .flat_map(|mut state| state.advance())
                        .into_par_iter()
                        .filter_map(|state| {
                            if state.minutes < 32 {
                                Some(state)
                            } else {
                                max_geode.fetch_max(state.pack.geode, Ordering::Relaxed);
                                None
                            }
                        });
                    queue.par_extend(new);
                }
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
