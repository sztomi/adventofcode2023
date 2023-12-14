#![feature(iter_map_windows)]

use common::{get_lines, Result};
use itertools::Itertools;
use std::cmp::min;
use std::collections::HashMap;

fn is_vert_mirror(line: &[char], i: usize, j: usize, r: usize, e: usize) -> bool {
    let max_r = min(line.len() - j - 1, i);
    line[i - r] == line[j + r] && (r + 1 > max_r || is_vert_mirror(line, i, j, r + 1, e))
}

fn get_mirrors(map: &[Vec<char>], e: usize) -> Option<(usize, usize)> {
    let mut mirrors = HashMap::new();
    for line in map.iter() {
        for (i, j) in (0..line.len()).tuple_windows() {
            if is_vert_mirror(line, i, j, 0, e) {
                *mirrors.entry((i, j)).or_insert(0) += 1;
            }
        }
    }

    let mirrors = mirrors
        .iter()
        .map(|((i, j), n)| ((*i, *j), *n))
        .sorted_by_key(|(_, n)| -(*n as i64))
        .collect_vec();

    mirrors.iter().find_map(|((i, j), n)| {
        if *n == map.len() - e {
            Some((*i, *j))
        }
        else {
            None
        }
    })
}

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day13.txt")?;

    let maps = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();
    let maps = maps.split(|l| l.is_empty()).collect_vec();

    let tmaps = maps
        .iter()
        .map(|m| {
            let mut tm = vec![];
            for i in 0..m[0].len() {
                tm.push(m.iter().cloned().map(|l| l[i]).collect_vec());
            }
            tm
        })
        .collect_vec();

    let mut vmirrors = HashMap::new();
    for (mi, map) in maps.iter().enumerate() {
        if let Some(p) = get_mirrors(map, 0) {
            vmirrors.insert(mi, p);
        }
    }
    let vl: usize = vmirrors.iter().map(|(_, (i, _))| i + 1).sum();

    let mut hmirrors = HashMap::new();
    for (mi, map) in tmaps.iter().enumerate() {
        if let Some(p) = get_mirrors(map, 0) {
            hmirrors.insert(mi, p);
        }
    }
    let hl: usize = hmirrors.iter().map(|(_, (i, _))| (i + 1) * 100).sum();

    let part1 = vl + hl;
    println!("Part 1: {}", part1);

    let mut vmirrors = HashMap::new();
    for (mi, map) in maps.iter().enumerate() {
        if let Some(p) = get_mirrors(map, 1) {
            vmirrors.insert(mi, p);
        }
    }
    let vl: usize = vmirrors.iter().map(|(_, (i, _))| i + 1).sum();

    let mut hmirrors = HashMap::new();
    for (mi, map) in tmaps.iter().enumerate() {
        if let Some(p) = get_mirrors(map, 1) {
            hmirrors.insert(mi, p);
        }
    }
    let hl: usize = hmirrors.iter().map(|(_, (i, _))| (i + 1) * 100).sum();

    let part2 = vl + hl;
    println!("Part 2: {}", part2);

    Ok(())
}
