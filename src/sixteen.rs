use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::util::{BoxError, Result};

#[test]
fn one() {
    let valves = read_input().unwrap();
    let mut to_visit = VecDeque::new();
    to_visit.push_back(&valves["AA"]);
    let mut visited = HashSet::new();

    while let Some(valve) = to_visit.pop_front() {
        for other in &valve.tunnels_to {
            if !visited.contains(other) {
                to_visit.push_back(&valves[other]);
                visited.insert(other.clone());
            }
        }
    }
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels_to: Vec<String>,
}

impl FromStr for Valve {
    type Err = BoxError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.strip_prefix("Valve ").unwrap();
        let (name, s) = s.split_once(" ").unwrap();
        let s = s.strip_prefix("has flow rate=").unwrap();
        dbg!(s);
        let (rate_str, s) = s.split_once(";").unwrap();
        let (_, s) = s[17..].split_once(" ").unwrap();
        let tunnels_to = s.split(", ").map(String::from).collect();

        Ok(Valve {
            name: name.to_string(),
            flow_rate: rate_str.parse()?,
            tunnels_to,
        })
    }
}

fn read_input() -> Result<HashMap<String, Valve>> {
    let lines = BufReader::new(File::open("input/16.in")?).lines();

    let mut m: HashMap<String, Valve> = HashMap::new();
    for valve in lines.map(|line| line?.parse()) {
        let valve: Valve = valve?;
        let name: String = valve.name.clone();
        m.insert(name, valve);
    }

    Ok(m)
}
