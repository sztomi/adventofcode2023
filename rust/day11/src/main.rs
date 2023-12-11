use common::num::between;
use common::{get_lines, Point, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day11.txt")?;

    let empty_rows = (0..lines.len())
        .filter(|&i| lines[i].chars().all(|c| c == '.'))
        .collect_vec();
    let empty_cols = (0..lines[0].len())
        .filter(|&i| lines.iter().all(|l| l.chars().nth(i).unwrap() == '.'))
        .collect_vec();
    let galaxies = (0..lines.len())
        .cartesian_product(0..lines[0].len())
        .filter(|(x, y)| lines[*x].chars().nth(*y).unwrap() == '#')
        .map(|(x, y)| Point(x as i64, y as i64))
        .collect_vec();

    let ans = |f: i64| {
        galaxies
            .iter()
            .combinations(2)
            .map(|g| {
                let xr = empty_rows
                    .iter()
                    .filter(|&r| between(g[0].0, g[1].0, *r as i64))
                    .count() as i64
                    * (f - 1);
                let xc = empty_cols
                    .iter()
                    .filter(|&c| between(g[0].1, g[1].1, *c as i64))
                    .count() as i64
                    * (f - 1);
                let d = (Point(g[0].0 + xr, g[0].1 + xc) - Point(g[1].0 + xr, g[1].1 + xc)).abs();
                d.0 + d.1 + xr + xc
            })
            .sum()
    };
    let part1: i64 = ans(2);
    println!("Part 1: {}", part1);

    let part2: i64 = ans(1_000_000);
    println!("Part 2: {}", part2);
    Ok(())
}
