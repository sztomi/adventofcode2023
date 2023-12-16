#![feature(iter_repeat_n)]
#![allow(unstable_name_collisions)]

use std::{collections::HashMap, time::Instant};

use itertools::Itertools;

use common::{get_char_map, Dir, Point, Result};
use terny::iff;

// row, col
// North, West, South, East
const DIRS: [Dir; 4] = [Dir(-1, 0), Dir(0, -1), Dir(1, 0), Dir(0, 1)];

fn tilt(map: &mut [Vec<char>], dir: &Dir) -> usize {
    let rocks = map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(j, c)| iff!(*c == 'O' ? Some(Point(i as i64, j as i64)) : None))
                .collect_vec()
        })
        .sorted_by_key(|p| p * -dir) // <3
        .collect_vec();

    let (w, h) = (map.len() as i64, map[0].len() as i64);
    for p in rocks.iter() {
        let mut x = p.0 as usize;
        let mut y = p.1 as usize;
        map[x][y] = '.';

        let mut np = *p;
        loop {
            np += dir;
            if np.0 < 0 || np.0 >= h || np.1 < 0 || np.1 >= w {
                break;
            }
            if map[np.0 as usize][np.1 as usize] != '.' {
                break;
            }
            x = np.0 as usize;
            y = np.1 as usize;
        }
        map[x][y] = 'O';
    }

    map.iter()
        .enumerate()
        .map(|(i, l)| {
            l.iter()
                .filter(|c| **c == 'O')
                .map(|_| h as usize - i)
                .sum::<usize>()
        })
        .sum()
}

fn main() -> Result<()> {
    let mut map = get_char_map("../inputs/day14.txt")?;

    let start = Instant::now();
    let part1 = tilt(&mut map, &Dir(-1, 0));
    println!("Part1: {}", part1);
    let part1_time = start.elapsed();

    let mut map = get_char_map("../inputs/day14.txt")?;

    let start = Instant::now();
    let mut cm = HashMap::new();
    let mut vals = HashMap::new();
    let (mut seq_start, mut seq_end) = (0, 0);
    for i in 0..4_000_000_000 {
        let dir = DIRS[i % 4];
        let v = tilt(&mut map, &dir);
        if i % 3 == 0 {
            if cm.contains_key(&map) {
                seq_start = *cm.get(&map).unwrap();
                seq_end = i;
                break;
            }
            cm.insert(map.clone(), i);
            vals.insert(i, v);
        }
    }
    let rl = seq_end - seq_start;
    let x = seq_start + (4_000_000_000 - seq_end) % rl - 1;
    let part2 = vals[&x];
    println!("Part2: {}", part2);
    let part2_time = start.elapsed();

    println!("Times:");
    println!("  Part1: {:?}", part1_time);
    println!("  Part2: {:?}", part2_time);

    Ok(())
}
