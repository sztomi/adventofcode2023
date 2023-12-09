#![feature(iter_map_windows)]
use common::{get_lines, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day9.txt")?;

    let vals = lines
        .iter()
        .map(|s| {
            s.split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec()
        })
        .map(|line| {
            let mut acc = vec![line.clone()];
            let mut i = 0;
            loop {
                let d = acc[i].iter().map_windows(|[a, b]| *b - *a).collect_vec();
                acc.push(d.clone());
                if d.iter().all(|&n| n == 0) {
                    break;
                }
                i = acc.len() - 1;
            }
            acc
        })
        .collect_vec();

    let ends = vals
        .iter()
        .map(|diffs| diffs.iter().map(|dv| *dv.last().unwrap()).collect_vec())
        .collect_vec();

    let part1 = ends
        .iter()
        .map(|ev| {
            let mut xev = ev.clone();
            for i in (1..xev.len()).rev() {
                xev[i - 1] += xev[i];
            }
            xev[0]
        })
        .sum::<i64>();
    println!("Part 1: {}", part1);

    let firsts = vals
        .iter()
        .map(|diffs| diffs.iter().map(|dv| *dv.first().unwrap()).collect_vec())
        .collect_vec();

    let part2 = firsts
        .iter()
        .map(|sv| {
            let mut xsv = sv.clone();
            for i in (1..xsv.len()).rev() {
                xsv[i - 1] -= xsv[i];
            }
            xsv[0]
        })
        .sum::<i64>();
    println!("Part 2: {}", part2);

    Ok(())
}
