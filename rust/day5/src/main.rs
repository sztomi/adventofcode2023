use common::{get_lines, Result};
use itertools::Itertools;
use std::collections::BTreeMap;
use rayon::prelude::*;

#[derive(Debug)]
struct Entry(u64, u64, u64); // dst, src, len
#[derive(Debug)]
struct BullshitMap {
    entries: BTreeMap<u64, Entry>,
}

impl Entry {
    fn get(&self, x: u64) -> u64 {
        if x < self.1 || x > self.1 + self.2 {
            x
        }
        else {
            self.0 + (x - self.1)
        }
    }
}

impl BullshitMap {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }
    fn get(&self, x: u64) -> u64 {
        self.entries
            .range(..=x)
            .next_back()
            .map(|(_, e)| e.get(x))
            .unwrap_or(x)
    }
}

fn parse_maps(lines: &[String]) -> Vec<BullshitMap> {
    let mut maps = Vec::new();
    let mut current_map = BullshitMap::new();

    for line in lines[2..].iter() {
        if line.trim().is_empty() {
            if !current_map.entries.is_empty() {
                maps.push(current_map);
                current_map = BullshitMap::new();
            }
        }
        else if !line.contains("map:") {
            let parts: Vec<u64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if parts.len() == 3 {
                let entry = Entry(parts[0], parts[1], parts[2]);
                current_map.entries.insert(parts[1], entry);
            }
        }
    }
    if !current_map.entries.is_empty() {
        maps.push(current_map);
    }

    maps
}

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day5.txt")?;
    let seeds = lines[0]
        .split(&[':', ' '])
        .filter_map(|s| s.parse::<u64>().ok())
        .collect_vec();

    let maps = parse_maps(&lines);
    let part1: u64 = seeds
        .iter()
        .map(|s| maps.iter().fold(*s, |acc, m| m.get(acc)))
        .min().unwrap();
    println!("Part1: {}", part1);

    let part2 = seeds
        .iter()
        .tuples::<(_, _)>()
        .par_bridge()
        .flat_map(|(s, len)| (*s..s + len))
        .map(|s| maps.iter().fold(s, |acc, m| m.get(acc)))
        .min().unwrap();
    println!("Part2: {}", part2);

    Ok(())
}
