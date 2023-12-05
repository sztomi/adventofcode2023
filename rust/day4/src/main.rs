use common::{get_lines, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    const TOTAL: usize = 35;
    let lines = get_lines("../inputs/day4.txt")?;
    let grade = |s: HashSet<_>| {
        if s.len() < TOTAL {
            2u64.pow(((TOTAL - s.len()) as i64 - 1) as u32)
        }
        else {
            0
        }
    };
    let scores = lines
        .iter()
        .map(|line| {
            line.split(&[' ', ':', '|'])
                .filter(|n| !n.is_empty())
                .skip(2)
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<HashSet<_>>()
        })
        .map(grade)
        .collect_vec();
    let part1: u64 = scores.iter().sum();
    println!("Part1: {}", part1);

    let cw = |s: u64| if s == 0 { 0 } else { s.ilog2() as usize + 1 };
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    struct CardId(usize);
    let wins: HashMap<CardId, _> = scores
        .iter()
        .enumerate()
        .map(|(i, s)| (CardId(i + 1), cw(*s)))
        .collect();

    let mut copies: HashMap<CardId, usize> = HashMap::new();
    scores.iter().enumerate().for_each(|(i, _score)| {
        let id = CardId(i + 1);
        let cps = *copies.get(&id).unwrap_or(&0);

        (0..cps+1).for_each(|_| {
            if let Some(&ws) = wins.get(&id) {
                if ws > 0 {
                    ((i + 1)..(i + ws + 1)).for_each(|j| {
                        let id = CardId(j + 1);
                        *copies.entry(id).or_insert(0) += 1;
                    });
                }
            }
        });
    });
    let part2 = copies.values().sum::<usize>() + scores.len();
    println!("Part2: {}", part2);

    Ok(())
}