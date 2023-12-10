use common::{get_lines, Dir, Point, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day10.txt")?;

    let s: Point = (0..lines.len())
        .cartesian_product(0..lines[0].len())
        .find(|&(x, y)| lines[x].chars().nth(y).unwrap() == 'S')
        .map(|(x, y)| (x as i64, y as i64))
        .unwrap()
        .into();

    // x = row, y = col
    let PIPE = "|-LJ7F";
    let JUNC = "LJ7F";
    let SDIRS = [
        [Dir(0, 1), Dir(-1, 0)],  // L
        [Dir(-1, 0), Dir(0, -1)], // J
        [Dir(0, -1), Dir(1, 0)],  // 7
        [Dir(0, 1), Dir(1, 0)],   // F
    ];

    let at = |p: Point| lines[p.0 as usize].chars().nth(p.1 as usize).unwrap();

    let sd = |c: char, indir: Dir| {
        SDIRS[JUNC.find(c).unwrap()]
            .iter()
            .find(|&d| *d != -indir)
            .unwrap()
    };

    let next = |p: Point, d: Option<Dir>| {
        let c = at(p);
        if let Some(i) = JUNC.find(c) {
            let d = sd(JUNC.chars().nth(i).unwrap(), d.unwrap());
            (p + d, Some(*d))
        }
        else if let Some(d) = d {
            (p + d, Some(d))
        }
        else {
            let np = *p
                .saround()
                .iter()
                .find(|np| PIPE.contains(at(**np)))
                .unwrap();
            (np, Some((np - p).norm()))
        }
    };
    let mut p = s;
    let mut d = None;
    let mut cnt = 1;
    loop {
        (p, d) = next(p, d);
        if at(p) == 'S' {
            break;
        }
        cnt += 1;
    }
    println!("Part 1: {}", cnt / 2);

    Ok(())
}
