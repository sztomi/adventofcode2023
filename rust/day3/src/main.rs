use common::{get_lines, Point, Result};

#[derive(Debug)]
struct Number(i64, Vec<Point>);

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day3.txt")?;

    let numbers: Vec<Number> = lines
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, line)| {
            let mut ps: Vec<Point> = Vec::new();
            let mut n = 0;
            for (x, c) in line.chars().enumerate() {
                match c.to_digit(10) {
                    Some(digit) => {
                        ps.push(Point(x as i64, y as i64));
                        n = n * 10 + digit as i64;
                    }
                    None => {
                        if !ps.is_empty() {
                            acc.push(Number(n, ps.clone()));
                            ps.clear();
                            n = 0;
                        }
                    }
                }
            }
            if !ps.is_empty() {
                acc.push(Number(n, ps));
            }
            acc
        });

    let part1: i64 = numbers
        .iter()
        .filter(|n| {
            n.1.iter().any(|p| {
                p.around()
                    .iter()
                    .filter_map(|p| {
                        lines
                            .get(p.1 as usize)
                            .and_then(|line| line.chars().nth(p.0 as usize))
                    })
                    .any(|c| c != '.' && !c.is_ascii_digit())
            })
        })
        .map(|n| n.0)
        .sum();
    println!("Part1: {}", part1);

    let part2: i64 = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '*')
                .map(move |(x, _)| Point(x as i64, y as i64))
        })
        .filter_map(|s| {
            let mut ns = numbers
                .iter()
                .filter(|n| n.1.iter().any(|p| p.around().contains(&s)))
                .map(|n| n.0)
                .collect::<Vec<i64>>();
            if ns.len() == 2 {
                ns.sort();
                Some(ns)
            }
            else {
                None
            }
        })
        .map(|ns| ns[0] * ns[1])
        .sum();
    println!("Part2: {}", part2);

    Ok(())
}
