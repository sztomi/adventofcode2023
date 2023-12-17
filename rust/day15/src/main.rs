use std::collections::HashMap;

use common::{get_chars, Result};
use itertools::Itertools;

fn hash(s: &str) -> u64 {
    s.chars().fold(0, |acc, c| (acc + c as u64) * 17) % 256
}

fn main() -> Result<()> {
    let data = get_chars("../inputs/day15.txt")?;
    let data = data.split(|c| [',', '\n'].contains(c)).collect_vec();
    let part1 = data.iter().fold(0, |acc, s| {
        acc + s.iter().fold(0, |acc, c| (acc + *c as u64) * 17) % 256
    });
    println!("Part 1: {}", part1);

    let data = data
        .iter()
        .map(|s| {
            s.split(|c| ['=', '-'].contains(c))
                .map(String::from_iter)
                .collect_vec()
        })
        .map(|v| (v[0].clone(), v[1].parse::<u64>().ok()))
        .collect_vec();

    let mut hm = HashMap::new();

    for e in data.iter() {
        match e {
            (l, Some(fl)) => {
                let v = hm.entry(hash(l)).or_insert(Vec::new());
                if let Some(it) = v.iter_mut().find(|(ll, _)| ll == l) {
                    it.1 = fl;
                }
                else {
                    v.push((l.to_string(), fl));
                }
            }
            (l, None) => {
                let v = hm.entry(hash(l)).or_insert(Vec::new());
                if let Some(i) = v.iter_mut().position(|(ll, _)| ll == l) {
                    v.remove(i);
                }
            }
        }
    }
    let part2 = hm
        .iter()
        .flat_map(|(_, v)| v.iter().map(|e| e.0.clone()).collect_vec())
        .map(|l| {
            let b = hash(&l);
            let v = hm.get(&b).unwrap();
            let lp = v.iter().position(|(ll, _)| ll == &l).unwrap();
            let fl = v[lp].1;
            (1 + b) * (lp as u64 + 1) * fl
        })
        .sum::<u64>();
    println!("Part 2: {}", part2);

    Ok(())
}
