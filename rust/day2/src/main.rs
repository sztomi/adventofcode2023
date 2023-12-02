use common::{get_lines, Result};

#[derive(Clone, Copy)]
struct Round(usize, usize, usize); // bgr

impl Round {
    fn is_valid(&self) -> bool {
        self.0 <= 14 && self.1 <= 13 && self.2 <= 12
    }
    fn power(&self) -> usize {
        self.0 * self.1 * self.2
    }
}

fn parse_game(line: &str) -> Vec<Round> {
    let st = line.find(':').unwrap() + 2;
    line[st..]
        .split(';')
        .map(|round| {
            let mut rs = round
                .split(',')
                .map(|t| {
                    let take: Vec<_> = t.trim().split(' ').map(|x| x.trim()).collect();
                    (take[0].parse::<usize>().unwrap(), take[1])
                })
                .collect::<Vec<(_, _)>>();
            rs.sort_by(|a, b| a.1.cmp(b.1));
            rs
        })
        .map(|r| match r.as_slice() {
            [(b, "blue")] => Round(*b, 0, 0),
            [(g, "green")] => Round(0, *g, 0),
            [(r, "red")] => Round(0, 0, *r),
            [(b, "blue"), (g, "green")] => Round(*b, *g, 0),
            [(b, "blue"), (r, "red")] => Round(*b, 0, *r),
            [(g, "green"), (r, "red")] => Round(0, *g, *r),
            [(b, "blue"), (g, "green"), (r, "red")] => Round(*b, *g, *r),
            _ => panic!("at the disco"),
        })
        .collect()
}

fn main() -> Result<()> {
    let lines = get_lines("../inputs/day2.txt")?;
    let part1: usize = lines
        .iter()
        .enumerate()
        .map(|(i, line)| (i + 1, parse_game(line)))
        .filter(|(_, game)| game.iter().all(|&r| r.is_valid()))
        .map(|(num, _)| num)
        .sum();
    println!("Part 1: {}", part1);

    let part2: usize = lines
        .iter()
        .enumerate()
        .map(|(i, line)| (i + 1, parse_game(line)))
        .map(|g| {
            Round(
                g.1.iter().max_by_key(|r| r.0).unwrap().0,
                g.1.iter().max_by_key(|r| r.1).unwrap().1,
                g.1.iter().max_by_key(|r| r.2).unwrap().2,
            )
            .power()
        })
        .sum();
    println!("Part 2: {}", part2);
    Ok(())
}
