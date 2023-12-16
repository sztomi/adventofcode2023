use std::collections::HashMap;

use itertools::Itertools;

use common::{get_lines, Result};

fn count_shit(line: &str, blocks: &[usize]) -> usize {
    let mut dp = HashMap::new();

    fn fts(
        i: usize,
        bi: usize,
        len: usize,
        dp: &mut HashMap<(usize, usize, usize), usize>,
        l: &str,
        b: &[usize],
    ) -> usize {
        if dp.contains_key(&(i, bi, len)) {
            return dp[&(i, bi, len)];
        }

        let mut acc = 0;
        if i != l.len() {
            let c = l.chars().nth(i).unwrap();
            if c == '#' {
                acc += fts(i + 1, bi, len + 1, dp, l, b);
            }
            else if c == '.' {
                if len == 0 {
                    acc += fts(i + 1, bi, 0, dp, l, b);
                }
                else if bi < b.len() && len == b[bi] {
                    acc += fts(i + 1, bi + 1, 0, dp, l, b);
                }
            }
            else {
                acc += fts(i + 1, bi, len + 1, dp, l, b);
                if len == 0 {
                    acc += fts(i + 1, bi, 0, dp, l, b);
                }
                else if bi < b.len() && len == b[bi] {
                    acc += fts(i + 1, bi + 1, 0, dp, l, b);
                }
            }
        }
        // Line has ended or we have reached the last block
        else if (len == 0 && bi == b.len()) || (bi == b.len() - 1 && b[bi] == len)
        {
            return 1;
        }
        // Poop emoji
        else {
            return 0;
        }

        dp.insert((i, bi, len), acc);
        acc
    }

    fts(0, 0, 0, &mut dp, line, blocks)
}

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day12.txt")?;
    let records = lines
        .iter()
        .map(|s| {
            let mut s = s.split(' ');
            (
                s.next().unwrap(),
                s.next()
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let part1: usize = records
        .iter()
        .map(|(line, blocks)| count_shit(line, blocks))
        .sum();
    println!("Part1: {part1}");

    let expanded = records
        .iter()
        .map(|(l, blocks)| {
            (
                format!("{l}?{l}?{l}?{l}?{l}"),
                blocks
                    .iter()
                    .cloned()
                    .cycle()
                    .take(5 * blocks.len())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let part2: usize = expanded
        .iter()
        .map(|(line, blocks)| count_shit(line, blocks))
        .sum();
    println!("Part2: {part2}");

    Ok(())
}
