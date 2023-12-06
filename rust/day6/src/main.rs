use std::iter::zip;
use std::time::Instant;
use common::{get_lines, Result};

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day6.txt")?;

    let start = Instant::now();
    let times: Vec<i64> = lines[0].split(&[':', ' ']).filter_map(|s| s.parse().ok()).collect();
    let dsts: Vec<i64> = lines[1].split(&[':', ' ']).filter_map(|s| s.parse().ok()).collect();
    let t: i64 = lines[0].chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse()?;
    let d: i64 = lines[1].chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse()?;
    let parse_time = start.elapsed();

    let s = |t: f64, d: f64| {
        let det = (t.powi(2) - 4.0 * d).sqrt();
        (((-t + det) / 2.0).ceil() - ((-t - det) / 2.0).floor() - 1.0) as i64
    };

    let part1: i64 = zip(times, dsts)
        .map(|(t, d)| s(t as f64, d as f64))
        .product();
    println!("Part 1: {}", part1);
    let part1_time = start.elapsed() - parse_time;

    let part2 = s(t as f64, d as f64);
    println!("Part 2: {}", part2);
    let part2_time = start.elapsed() - part1_time;

    println!("Times:");
    println!("  Parse: {:?}", parse_time);
    println!("  Part 1: {:?}", part1_time);
    println!("  Part 2: {:?}", part2_time);
    println!("  Total:  {:?}", parse_time + part1_time + part2_time);

    Ok(())
}