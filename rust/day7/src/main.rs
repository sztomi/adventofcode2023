use common::{get_lines, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::zip;

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    jkind: bool,
}

const RANKS: &str = "23456789TJQKA";
const JRANKS: &str = "J23456789TQKA";

impl Hand {
    fn new(s: &str) -> Self {
        Self {
            cards: s.to_string(),
            jkind: false,
        }
    }

    fn jnew(s: &str) -> Self {
        Self {
            cards: s.to_string(),
            jkind: true,
        }
    }

    fn ranks(&self) -> &str {
        if self.jkind {
            JRANKS
        }
        else {
            RANKS
        }
    }

    fn strength(&self) -> i64 {
        let u = self.cards.chars().collect::<HashSet<_>>().len();
        let hc = self
            .cards
            .chars()
            .map(|c| self.cards.matches(c).count())
            .max()
            .unwrap();
        let s = match (hc, u) {
            (5, _) => 6,
            (4, _) => 5,
            (3, 2) => 4,
            (3, 3) => 3,
            (2, 3) => 2,
            (2, 4) => 1,
            (1, _) => 0,
            _ => unreachable!(),
        };
        if !self.jkind {
            s
        }
        else {
            let j = self.cards.matches('J').count();
            match (s, j) {
                (s, 0) => s,
                (4..=6, _) | (3, 2) => 6,
                (3, _) | (2, 2) => 5,
                (2, 1) => 4,
                (1, _) => 3,
                (0, 1) => 1,
                (s, _) => s,
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.chars().sorted().collect_vec() == other.cards.chars().sorted().collect_vec()
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_strength = self.strength();
        let other_strength = other.strength();
        if self_strength != other_strength {
            self_strength.cmp(&other_strength)
        }
        else {
            let ranks = self.ranks();
            zip(self.cards.chars(), other.cards.chars())
                .map(|(l, r)| ranks.find(l).unwrap().cmp(&ranks.find(r).unwrap()))
                .find(|&o| o != std::cmp::Ordering::Equal)
                .unwrap()
        }
    }
}

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day7.txt")?;
    let part1: usize = lines
        .iter()
        .filter_map(|line| {
            line.split_whitespace()
                .collect_tuple()
                .map(|(l, r)| (Hand::new(l), r.parse::<usize>().unwrap()))
        })
        .sorted_by_key(|(h, _)| h.clone())
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .sum();
    println!("Part 1: {}", part1);
    let part2: usize = lines
        .iter()
        .filter_map(|line| {
            line.split_whitespace()
                .collect_tuple()
                .map(|(l, r)| (Hand::jnew(l), r.parse::<usize>().unwrap()))
        })
        .sorted_by_key(|(h, _)| h.clone())
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .sum();
    println!("Part 2: {}", part2);

    Ok(())
}
