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
    let pipe = "|-LJ7F";
    let junc = "LJ7F";
    let sdirs = [
        [Dir(0, 1), Dir(-1, 0)],  // L
        [Dir(-1, 0), Dir(0, -1)], // J
        [Dir(0, -1), Dir(1, 0)],  // 7
        [Dir(0, 1), Dir(1, 0)],   // F
    ];

    let at = |p: Point| lines[p.0 as usize].chars().nth(p.1 as usize).unwrap();

    let sd = |c: char, indir: Dir| {
        sdirs[junc.find(c).unwrap()]
            .iter()
            .find(|&d| *d != -indir)
            .unwrap()
    };

    let next = |p: Point, d: Option<Dir>| {
        let c = at(p);
        if let Some(i) = junc.find(c) {
            let d = sd(junc.chars().nth(i).unwrap(), d.unwrap());
            (p + d, Some(*d))
        }
        else if let Some(d) = d {
            (p + d, Some(d))
        }
        else {
            let np = *p
                .saround()
                .iter()
                .find(|np| pipe.contains(at(**np)))
                .unwrap();
            (np, Some((np - p).norm()))
        }
    };
    let mut p = s;
    let mut d = None;
    let mut cnt = 1;
    let mut lps = vec![];
    loop {
        lps.push(p);
        (p, d) = next(p, d);
        if at(p) == 'S' {
            break;
        }
        cnt += 1;
    }
    println!("Part 1: {}", cnt / 2);

    let mut nlines: Vec<String> = vec![];
    let mut ll: Vec<String> = vec![];
    for _ in &lines {
        let line = ".".repeat(lines[0].len());
        let nline = ".".repeat(lines[0].len() * 2);
        nlines.push(nline.clone());
        ll.push(line);
        nlines.push(String::from_iter(std::iter::repeat('.').take(nline.len())));
    }
    for &p in &lps {
        let c = at(p);
        nlines[p.0 as usize * 2]
            .replace_range(p.1 as usize * 2..p.1 as usize * 2 + 1, &c.to_string());
        ll[p.0 as usize].replace_range(p.1 as usize..p.1 as usize + 1, &c.to_string());
    }

    let oob = |p: Point, nlines: &Vec<String>| {
        p.0 < 0 || p.1 < 0 || p.0 >= nlines.len() as i64 || p.1 >= nlines[0].len() as i64
    };
    let atn =
        |p: Point, nlines: &Vec<String>| nlines[p.0 as usize].chars().nth(p.1 as usize).unwrap();
    for (x, y) in (0..nlines.len()).cartesian_product(0..nlines[0].len()) {
        if nlines[x].chars().nth(y).unwrap() != '.' {
            continue;
        }

        // Vertically
        let p1 = Point(x as i64 - 1, y as i64);
        let p2 = Point(x as i64 + 1, y as i64);
        if !oob(p1, &nlines)
            && !oob(p2, &nlines)
            && "|7FS".contains(atn(p1, &nlines))
            && "|JLS".contains(atn(p2, &nlines))
        {
            nlines[x].replace_range(y..y + 1, "|");
        }

        // Horizontally
        let p1 = Point(x as i64, y as i64 - 1);
        let p2 = Point(x as i64, y as i64 + 1);
        if !oob(p1, &nlines)
            && !oob(p2, &nlines)
            && "-LFS".contains(atn(p1, &nlines))
            && "-J7S".contains(atn(p2, &nlines))
        {
            nlines[x].replace_range(y..y + 1, "-");
        }
    }

    let mut work: Vec<Point> = (0..nlines.len())
        .cartesian_product(0..nlines[0].len())
        .filter(|&(x, y)| nlines[x].chars().nth(y).unwrap() == '.')
        .filter(|&(x, y)| x == 0 || y == 0 || x == nlines.len() - 1 || y == nlines[0].len() - 1)
        .map(|(x, y)| Point(x as i64, y as i64))
        .collect();

    let mut seen = std::collections::HashSet::new();
    while let Some(p) = work.pop() {
        if seen.contains(&p) {
            continue;
        }
        seen.insert(p);
        if atn(p, &nlines) == '.' {
            let x = p.0 as usize;
            let y = p.1 as usize;
            nlines[x].replace_range(y..y + 1, "O");
        }
        for np in p.saround() {
            if np.0 < 0 || np.1 < 0 || np.0 >= nlines.len() as i64 || np.1 >= nlines[0].len() as i64
            {
                continue;
            }
            if atn(np, &nlines) == '.' {
                work.push(np);
            }
        }
    }

    let part2: usize = (0..lines.len())
        .cartesian_product(0..lines[0].len())
        .filter(|&(x, y)| {
            ll[x].chars().nth(y).unwrap() == '.' && nlines[x * 2].chars().nth(y * 2).unwrap() == '.'
        })
        .count();
    println!("Part 2: {}", part2);

    Ok(())
}
