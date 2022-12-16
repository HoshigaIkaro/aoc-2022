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
            .trim_start_matches('s')
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
                .insert(target, find_connection(source, target, valves));
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
        traverse(&basic_map).to_string()
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
    remaining: Vec<&'a str>,
}

impl<'a> State<'a> {
    fn new(current: &'a str, valves: &'a ReducedMap) -> Self {
        let remaining = valves
            .keys()
            .copied()
            .filter(|v| *v != "AA" && *v != current)
            .collect();
        // dbg!(&remaining, current);
        Self {
            current_valve: current,
            elapsed_minutes: 0,
            pressure: 0,
            flow_rate: 0,
            remaining,
        }
    }

    fn calculate_final_pressure(&self) -> usize {
        self.pressure + self.flow_rate * (30 - self.elapsed_minutes)
    }

    /// Calculates the score used in pruning.
    ///
    /// The score is the sum of
    ///
    /// - the current pressure
    /// - the pressure gained from the current flow rate
    /// - the pressure gained from all remaining points
    ///
    /// during the remaining minutes.
    fn calculate_score(&self, valves: &ReducedMap) -> usize {
        let minutes_left = 30 - self.elapsed_minutes;
        self.pressure
            + minutes_left * self.flow_rate
            + minutes_left
                * self
                    .remaining
                    .iter()
                    .filter(|v| **v != self.current_valve)
                    .map(|valve| valves[self.current_valve][valve].flow_rate)
                    .sum::<usize>()
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = self.calculate_final_pressure();
        let right = other.calculate_final_pressure();
        left.cmp(&right)
    }
}

/// Finds the path with the most pressure after 30 minutes and returns the value.
///
/// Works only on the example input.
fn traverse(valves: &ReducedMap) -> usize {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State::new("AA", &valves));

    let mut best = 0;
    let mut best_array = [0; 31];
    let mut best_state = State::default();
    while let Some(state) = queue.pop() {
        // movement unavailable
        if state.remaining.is_empty() || state.elapsed_minutes == 30 {
            let score = state.calculate_score(&valves);
            if score > best {
                best = score;
                best_array[state.elapsed_minutes] = score;
                best_state = state;
            }
            continue;
        }

        for (i, possible) in state.remaining.iter().enumerate() {
            let mut state = state.clone();
            let connection = &valves[state.current_valve][possible];
            if state.elapsed_minutes + connection.distance > 30 {
                state.pressure += state.flow_rate * (30 - state.elapsed_minutes);
                state.elapsed_minutes = 30;
                let best = best_array.get_mut(state.elapsed_minutes).unwrap();
                let score = state.calculate_score(&valves);
                if score >= *best {
                    *best = (*best).max(score);
                    queue.push(state);
                }
                continue;
            }
            // move to the new valve
            state.current_valve = possible;
            state.elapsed_minutes += connection.distance;
            state.pressure += state.flow_rate * connection.distance;
            if state.elapsed_minutes == 30 {
                let best = best_array.get_mut(state.elapsed_minutes).unwrap();
                let score = state.calculate_score(&valves);
                if score >= *best {
                    *best = (*best).max(score);
                    queue.push(state);
                }
                continue;
            }
            // turn the valve
            state.elapsed_minutes += 1;
            state.pressure += state.flow_rate;
            state.remaining.remove(i);
            state.flow_rate += connection.flow_rate;
            let best = best_array.get_mut(state.elapsed_minutes).unwrap();
            let score = state.calculate_score(&valves);
            if score >= *best {
                *best = (*best).max(score);
                queue.push(state);
            }
        }
    }
    best_state.pressure + best_state.flow_rate * (30 - best_state.elapsed_minutes)
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
