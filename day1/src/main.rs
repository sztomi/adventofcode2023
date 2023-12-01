#![feature(let_chains)]

use common::{get_lines, Result};

fn main() -> Result<()> {
    let lines = get_lines("day1/input.txt")?;

    let isnum = |c: &char| c.is_ascii_digit();

    let tens: u32 = lines
        .iter()
        .map(|line| line.chars().find(isnum).unwrap().to_digit(10).unwrap())
        .sum();
    let ones: u32 = lines
        .iter()
        .map(|line| {
            line.chars()
                .rev()
                .find(isnum)
                .unwrap()
                .to_digit(10)
                .unwrap()
        })
        .sum();
    let part1 = tens * 10 + ones;
    println!("part1: {}", part1);

    let words: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let tens: u32 = lines
        .iter()
        .map(|line| {
            let (pos, num) = line
                .chars()
                .enumerate()
                .find(|(_, c)| c.is_ascii_digit())
                .map(|(pos, c)| (pos as u32, c.to_digit(10).unwrap()))
                .unwrap_or((u32::MAX, 0));
            let (wpos, wnum) = words
                .iter()
                .enumerate()
                .filter_map(|(idx, word)| line.find(word).map(|pos| (pos as u32, idx as u32 + 1)))
                .min_by_key(|(pos, val)| (*pos, *val))
                .unwrap_or((u32::MAX, 0));
            if pos < wpos {
                num
            }
            else {
                wnum
            }
        })
        .sum();

    let ones: u32 = lines
        .iter()
        .map(|line| {
            let n = line
                .chars()
                .rev()
                .enumerate()
                .find(|(_, c)| c.is_ascii_digit())
                .map(|(pos, c)| ((line.len() - pos - 1) as u32, c.to_digit(10).unwrap()));

            let w = words
                .iter()
                .enumerate()
                .flat_map(|(idx, word)| {
                    line.match_indices(word)
                        .map(move |(pos, _)| (pos as u32, idx as u32 + 1))
                })
                .max_by_key(|(pos, _)| *pos);

            if let Some((pos, num)) = n && let Some((wpos, wnum)) = w {
                if wpos > pos {
                    wnum
                } else {
                    num
                }
            } else if let Some((_, num)) = n {
                num
            } else if let Some((_, wnum)) = w {
                wnum
            } else {
                panic!("at the disco")
            }
        })
        .sum();

    println!("part2: {}", tens * 10 + ones);

    Ok(())
}
