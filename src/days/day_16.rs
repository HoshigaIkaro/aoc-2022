use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    hash::Hash,
};

use super::Day;

#[derive(Debug, PartialEq)]
struct Valve<'a> {
    flow_rate: usize,
    connected: Vec<&'a str>,
    open: bool,
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
            open: false,
        };
        valves.insert(valve_name, valve);
    }
    valves
}

#[derive(Debug)]
struct Connection {
    flow_rate: usize,
    distance: usize,
}
type ReducedMap<'a> = HashMap<&'a str, HashMap<&'a str, Connection>>;

/// Calculates the distance from each non-zero valve to another.
///
/// Valves with a flow rate of 0 will not be included in the calculations,
/// unless it is "AA", the starting position.
fn get_reduced_map<'a>(valves: &HashMap<&'a str, Valve<'a>>) -> ReducedMap<'a> {
    let mut full: HashMap<&str, HashMap<&str, Connection>> = HashMap::with_capacity(valves.len());
    for source in valves.keys() {
        // skips non-zero roots unless it is the start
        if valves[source].flow_rate == 0 && *source != "AA" {
            continue;
        }
        // calculate the distance for each target
        for target in valves.keys() {
            // except for targets with zero flow rate, the start, and the same valve as source
            if valves[target].flow_rate == 0 || target == source || *target == "AA" {
                continue;
            }
            full.entry(source)
                .or_default()
                .insert(target, find_connection(source, target, &valves));
        }
    }
    full
}


/// Finds the connection from `source` to `target` in the `values` map.
///
/// The current implementation searches breadth first with a `VecDeque`.
fn find_connection<'a>(
    source: &'a str,
    target: &'a str,
    valves: &HashMap<&'a str, Valve<'a>>,
) -> Connection {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut distance: HashMap<&str, usize> = std::iter::once((source, 0)).collect();
    let mut queue: VecDeque<&str> = std::iter::once(source).collect();
    while let Some(current) = queue.pop_front() {
        if current == target {
            break;
        }

        for possible in &valves[current].connected {
            if !visited.insert(possible) {
                continue; // already visited
            }
            distance.insert(possible, distance[&current] + 1);
            queue.push_back(possible);
        }
    }
    Connection {
        flow_rate: valves[&target].flow_rate,
        distance: distance[&target],
    }
}

pub struct Day16;

impl Day for Day16 {
    fn part_1(&self, input: &str) -> String {
        let valves = parse_valves(input);
        let basic_map = get_reduced_map(&valves);
        // dbg!(&basic_map["AA"]);
        traverse(basic_map).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
struct State<'a> {
    current_valve: &'a str,
    elapsed_minutes: usize,
    pressure: usize,
    flow_rate: usize,
    opened_valves: Vec<&'a str>,
}

impl<'a> State<'a> {
    fn new(current: &'a str) -> Self {
        Self {
            current_valve: current,
            elapsed_minutes: 0,
            pressure: 0,
            flow_rate: 0,
            opened_valves: Vec::new(),
        }
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = self.pressure + self.flow_rate * (30 - self.elapsed_minutes);
        let right = other.pressure + other.flow_rate * (30 - other.elapsed_minutes);
        left.cmp(&right)
    }
}

/// Finds the path with the most pressure after 30 minutes and returns the value.
///
/// Works only on the example input.
fn traverse<'a>(valves: ReducedMap<'a>) -> usize {
    let max_valves = valves.len() - 1; // remove count for "AA"
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State::new("AA"));

    let mut best = 0;
    let mut best_state = State::default();
    while let Some(state) = queue.pop() {
        // movement unavailable
        if state.opened_valves.len() == max_valves || state.elapsed_minutes == 30 {
            let pressure = state.pressure + state.flow_rate * (30 - state.elapsed_minutes);
            if pressure > best {
                best = pressure;
                best_state = state;
            }
            continue;
        }

        for &possible in valves.keys().filter(|new| {
            !state.opened_valves.contains(*new) && **new != "AA" && **new != state.current_valve
        }) {
            let mut state = state.clone();
            let connection = &valves[state.current_valve][possible];
            if state.elapsed_minutes + connection.distance > 30 {
                state.pressure += state.flow_rate * (30 - state.elapsed_minutes);
                state.elapsed_minutes = 30;
                queue.push(state);
                continue;
            }
            // move to the new valve
            state.current_valve = possible;
            state.elapsed_minutes += connection.distance;
            state.pressure += state.flow_rate * connection.distance;
            if state.elapsed_minutes == 30 {
                queue.push(state);
                continue;
            }
            // turn the valve
            state.elapsed_minutes += 1;
            state.pressure += state.flow_rate;
            state.opened_valves.push(possible);
            state.flow_rate += connection.flow_rate;

            queue.push(state);
        }
    }
    dbg!(&best_state);
    best_state.pressure + best_state.flow_rate * (30 - best_state.elapsed_minutes)
}

fn traverse_alpha<'a>(
    valves: &mut HashMap<&'a str, Valve<'a>>,
    full: &HashMap<&'a str, HashMap<&str, Connection>>,
    mut open: HashSet<&'a str>,
    current: &'a str,
    mut pressure: usize,
    mut total_flow: usize,
    mut minutes: usize,
) -> usize {
    // println!("{}", current);
    pressure += total_flow;
    if open.len() == 0 {
        // dbg!(minutes);
        let remaining_minutes = 30 - minutes;
        pressure + total_flow * remaining_minutes.saturating_sub(1)
    } else {
        let max_distance = open
            .iter()
            .filter(|c| full[current][*c].distance + minutes <= 30)
            .map(|op| full[current][op].distance)
            .max()
            .unwrap();
        let best = open
            .iter()
            .max_by_key(|c| {
                let conn = &full[current][*c];
                let mut pressure = 0;
                // move to place
                pressure += total_flow * conn.distance;
                // turn the valve
                pressure += total_flow;
                // finish waiting
                pressure += (total_flow + conn.flow_rate) * (max_distance + 1 - conn.distance + 1);
                pressure
            })
            .unwrap();
        dbg!(best);
        let conn = &full[current][*best];
        if conn.distance + minutes >= 30 {
            dbg!('a');
        }
        minutes += conn.distance + 1;
        pressure += total_flow * conn.distance;
        total_flow += conn.flow_rate;
        let mut open = open.clone();
        open.remove(best);

        traverse_alpha(valves, full, open, best, pressure, total_flow, minutes)
    }
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
                connected: vec!["DD", "II", "BB"],
                open: false
            }
        )
    }
}
