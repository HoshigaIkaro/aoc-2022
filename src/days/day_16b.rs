use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use super::Day;

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: usize,
    connected: Vec<&'a str>,
}

type Valves<'a> = HashMap<&'a str, Valve<'a>>;

fn parse_valves(input: &str) -> Valves {
    let mut valves = HashMap::new();
    for line in input.lines() {
        let line = line.strip_prefix("Valve ").unwrap();
        let (valve_name, rest) = line.split_once(" has flow rate=").unwrap();
        let (flow_rate, connected) = rest.split_once("; ").unwrap();
        let connected = connected
            .trim_start_matches("tunnels lead to valve")
            .trim_start_matches("tunnel leads to valve")
            .trim_start_matches('s')
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

#[derive(Debug)]
struct Connection {
    flow_rate: usize,
    distance: usize,
}

type Connections<'a> = HashMap<&'a str, HashMap<&'a str, Connection>>;
type Start<'a> = HashMap<&'a str, Connection>;

/// Calculates the distance between the non-zero valves.
/// Removes all valves with zero flow rate.
fn simplify_valves(valves: Valves) -> (Start, Connections) {
    let mut connections: Connections = HashMap::new();
    for source in valves.keys() {
        // only want useful valves
        if valves[source].flow_rate != 0 || *source == "AA" {
            // calculate the distance for each target
            for target in valves.keys() {
                // except for targets with zero flow rate
                if valves[target].flow_rate != 0 && target != source {
                    connections
                        .entry(source)
                        .or_default()
                        .insert(target, find_connection(source, target, &valves));
                }
            }
        }
    }
    let start = connections.remove("AA").unwrap();
    (start, connections)
}

fn find_connection<'a>(
    source: &'a str,
    target: &'a str,
    valves: &HashMap<&'a str, Valve<'a>>,
) -> Connection {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut distances: HashMap<&str, usize> = std::iter::once((source, 0)).collect();
    let mut queue: VecDeque<&str> = std::iter::once(source).collect();
    while let Some(current) = queue.pop_front() {
        if current == target {
            break;
        }

        for possible in &valves[current].connected {
            if !visited.insert(possible) {
                continue;
            }
            distances.insert(possible, distances[&current] + 1);
            queue.push_back(possible);
        }
    }
    Connection {
        flow_rate: valves[&target].flow_rate,
        distance: distances[&target],
    }
}

fn parse_simplified_valves(input: &str) -> (Start, Connections) {
    let valves = parse_valves(input);
    simplify_valves(valves)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
struct State<'a> {
    current_valve: &'a str,
    elapsed_minutes: usize,
    pressure: usize,
    flow_rate: usize,
    remaining: Vec<&'a str>,
}

impl<'a> State<'a> {
    fn new(current_valve: &'a str, remaining: Vec<&'a str>, elapsed_minutes: usize) -> Self {
        Self {
            current_valve,
            elapsed_minutes,
            pressure: 0,
            flow_rate: 0,
            remaining,
        }
    }

    fn minutes_left(&self) -> usize {
        30 - self.elapsed_minutes
    }

    fn final_pressure(&self) -> usize {
        self.pressure + self.flow_rate * self.minutes_left()
    }

    fn max_possible_pressure(&self, connections: &'a Connections<'a>) -> usize {
        self.minutes_left()
            * self
                .remaining
                .iter()
                .filter(|v| **v != self.current_valve)
                .map(|valve| connections[self.current_valve][valve].flow_rate)
                .sum::<usize>()
    }

    fn score(&self, connections: &Connections) -> usize {
        self.final_pressure() + self.max_possible_pressure(connections)
    }
}
impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = self.final_pressure();
        let right = other.final_pressure();
        left.cmp(&right)
    }
}

fn traverse<'a>(connections: &'a Connections<'a>, start: Start) -> State<'a> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    for (i, v) in connections.keys().enumerate() {
        let mut remaining = connections.keys().copied().collect::<Vec<_>>();
        remaining.remove(i);
        let state = State::new(*v, remaining, start[v].distance);
        queue.push(state);
    }

    let mut best_pressure = 0;
    let mut scores = [0; 31];
    let mut best_state = State::default();
    while let Some(state) = queue.pop() {
        // movement unavailable
        if state.remaining.is_empty() || state.elapsed_minutes == 30 {
            let pressure = state.final_pressure();
            if pressure > best_pressure {
                best_state = state;
            }
            continue;
        }

        let mut checked = false;
        for (i, new_valve) in state.remaining.iter().enumerate() {
            let mut state = state.clone();
            let connection = &connections[state.current_valve][new_valve];
            if state.elapsed_minutes + connection.distance >= 30 {
                if !checked {
                    let final_presure = state.final_pressure();
                    if final_presure > best_pressure {
                        best_pressure = final_presure;
                        best_state = state;
                    }
                    checked = true
                }
                continue;
            }
            // move to the new value
            state.current_valve = new_valve;
            state.elapsed_minutes += connection.distance + 1;
            state.pressure += state.flow_rate * (connection.distance) + 1;
            // turn the valve
            state.remaining.remove(i);
            state.flow_rate += connection.flow_rate;

            // stop this path early if there is already a better-scoring path at this point in time
            let best_score = scores.get_mut(state.elapsed_minutes).unwrap();
            let score = state.score(connections);
            if score >= *best_score {
                *best_score = (*best_score).max(score);
                queue.push(state);
            }
        }
    }
    best_state
}

pub struct Day16;
impl Day for Day16 {
    fn part_1(&self, input: &str) -> String {
        let (start, connections) = parse_simplified_valves(input);
        traverse(&connections, start).final_pressure().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}
