use std::collections::HashMap;
use common::{get_lines, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day8.txt")?;
    let d: Vec<_> = lines[0]
        .chars()
        .map(|c| {
            if c == 'L' {
                1
            }
            else {
                2
            }
        })
        .collect();
    let m = lines[2..]
        .iter()
        .map(|line| {
            line.split(&[' ', '=', '(', ')', ','])
                .filter(|e| !e.is_empty())
                .collect_vec()
        })
        .collect_vec();
    let pm: HashMap<_, _> = m.iter().enumerate().map(|(i, v)| (v[0], i)).collect();

    let mut i = pm["AAA"];
    let mut steps = 0;
    while m[i][0] != "ZZZ" {
        i = pm[m[i][d[steps % d.len()]]];
        steps += 1;
    }
    println!("Part 1: {}", steps);

    let part2 = pm.keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| {
            let mut i = pm[k];
            let mut steps = 0;
            while !m[i][0].ends_with('Z') {
                i = pm[m[i][d[steps % d.len()]]];
                steps += 1;
            }
            steps
        })
        .fold(1, num::integer::lcm);
    println!("Part 2: {}", part2);

    Ok(())
}
