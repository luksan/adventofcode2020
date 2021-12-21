use itertools::Itertools;
use std::fmt::{Debug, Formatter};

type LineType = Vec<u8>;
type Input = (Vec<u8>, Image);

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    let mut lines = line_source.into_iter();
    let alg = lines
        .next()
        .unwrap()
        .as_ref()
        .bytes()
        .map(|b| (b == b'#') as u8)
        .collect_vec();
    assert_eq!(alg.len(), 512);
    lines.next(); // blank
    let img = lines.map(parse).collect_vec();
    (alg, Image(img, 0))
}

fn parse<S: AsRef<str>>(s: S) -> LineType {
    let mut x = 0;
    let mut line = vec![0; 0];
    line.extend(
        s.as_ref()
            .bytes()
            .chain(".".bytes().cycle().take(102))
            .map(|c| {
                x = (x << 1 | (c == b'#') as u8) & 0b111;
                x
            }),
    );
    line
}

#[derive(Clone)]
struct Image(Vec<Vec<u8>>, usize);

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for c in row.iter() {
                if *c & 0b010 != 0 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Image {
    fn enhance(&self, alg: &[u8]) -> Self {
        let mut new = Vec::with_capacity(self.0.len() + 2);
        let inf = self.1;
        let inf_addr = inf << 6 | inf << 3 | inf;
        let new_inf = if alg[inf_addr] == 0 { 0 } else { 0b111 };
        for row in 0..(self.0.len() + 2) as i32 {
            let mut x = new_inf;
            let mut new_line = Vec::new();
            for col in 0..self.0[0].len() {
                x = (x << 1 | alg[self.bits_at(row - 1, col)]) & 0b111;
                new_line.push(x);
            }
            new.push(new_line);
        }

        Self(new, new_inf as usize)
    }

    fn bits_at(&self, row: i32, col: usize) -> usize {
        let mut ret = 0usize;
        for r in row - 1..row + 2 {
            if r < 0 || r >= self.0.len() as i32 {
                ret = ret << 3 | self.1;
                continue;
            }
            let r = r as usize;
            ret = ret << 3 | self.0[r][col as usize] as usize
        }
        ret
    }

    fn count_ones(&self) -> usize {
        self.0
            .iter()
            .flat_map(|line| line.iter().copied())
            .map(|p| (p >> 1 & 1) as usize)
            .sum()
    }
}

fn part1(input: &Input) -> usize {
    let (alg, img) = input;
    img.enhance(alg).enhance(alg).count_ones()
}

fn part2(input: &Input) -> usize {
    let (alg, img) = input;
    let mut img = img.enhance(alg);
    for _ in 1..50 {
        img = img.enhance(alg);
    }
    img.count_ones()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 5884);
    assert_eq!(part2(&d), 19043);
}

#[test]
fn test_data() {
    let data = // Example data
"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 35);
    assert_eq!(part2(&d), 3351);
}
