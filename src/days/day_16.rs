use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use super::Day;

#[derive(Debug, PartialEq)]
struct Valve<'a> {
    flow_rate: usize,
    connected: Vec<&'a str>,
}

fn traverse<'a>(
    valves: &HashMap<&str, Valve>,
    mut opened: HashSet<&'a str>,
    current: &'a str,
    mut total_flow: usize,
    mut pressure: usize,
    mut minute: usize,
) -> usize {
    minute += 1;
    pressure += total_flow;
    if minute == 30 {
        println!("{opened:?}");
        return pressure;
    }

    let current_valve = valves.get(current).unwrap();
    if current_valve.flow_rate == 0 || opened.contains(&current) {
        let mut possible: Vec<&str> = valves[&current]
            .connected
            .iter()
            .map(|v| *v)
            .collect();
        possible.sort_by_key(|v| Reverse(valves[v].flow_rate));
        // println!("{possible:?}");
        let mut m = 0;
        for valve in possible {
            m = m.max(traverse(valves, opened.clone(), valve, total_flow, pressure, minute));
        }
        m
    } else {
        let mut possible: Vec<&str> = valves[&current]
            .connected
            .iter()
            .filter(|v| !opened.contains(**v))
            .map(|v| *v)
            .collect();
        possible.sort_by_key(|v| Reverse(valves[v].flow_rate));
        if let Some(&better) = possible
            .iter()
            .find(|v| valves.get(*v).unwrap().flow_rate > current_valve.flow_rate)
        {
            traverse(valves, opened, better, total_flow, pressure, minute)
        } else {
            opened.insert(current);
            total_flow += current_valve.flow_rate;
            traverse(valves, opened, current, total_flow, pressure, minute)
        }
    }
}

pub struct Day16;

impl Day for Day16 {
    fn part_1(&self, input: &str) -> String {
        let valves = parse_valves(input);
        let opened = HashSet::new();
        let m = traverse(&valves, opened, "AA", 0, 0, 0);
        dbg!(m);
        todo!()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

fn parse_valves(input: &str) -> HashMap<&str, Valve> {
    let mut valves = HashMap::new();
    for line in input.lines() {
        let line = line.strip_prefix("Valve ").unwrap();
        let (valve_name, rest) = line.split_once(" has flow rate=").unwrap();
        let (flow_rate, connected) = rest.split_once("; ").unwrap();
        let connected = connected
            .trim_start_matches("tunnels lead to valve")
            .trim_start_matches("tunnel leads to valve")
            .trim_start_matches("s")
            .trim_start();
        let flow_rate = flow_rate.parse().unwrap();
        let connected: Vec<&str> = connected.split(", ").collect();
        let valve = Valve {
            flow_rate,
            connected,
        };
        valves.insert(valve_name, valve);
    }
    valves
}

#[cfg(test)]
mod day_16_tests {
    use super::*;

    #[test]
    fn parse_successful() {
        let valves = parse_valves("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB");
        assert_eq!(valves.len(), 1);
        assert_eq!(
            valves["AA"],
            Valve {
                flow_rate: 0,
                connected: vec!["DD", "II", "BB"]
            }
        )
    }
}
