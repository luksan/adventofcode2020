use itertools::Itertools;
use std::fmt::{Debug, Formatter};

type LineType = u128;
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
    (alg, Image(img))
}

fn parse<S: AsRef<str>>(s: S) -> LineType {
    s.as_ref()
        .bytes()
        .fold(0u128, |line, c| line << 1 | ((c == b'#') as u128))
        << 10 // shit 10 to pad right side
}

#[derive(Clone)]
struct Image(Vec<u128>);

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for c in 0..128 {
                if *row >> (127 - c) & 1 == 1 {
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
    fn enhance(&self, alg: &[u8], inf: usize) -> Self {
        let mut new = vec![0u128; self.0.len() + 4];
        for row in 0..new.len() {
            for bit in (0..128).rev() {
                new[row] = new[row] << 1 | alg[self.bits_at(row as i32 - 2, bit, inf)] as u128
            }
        }
        Self(new)
    }

    fn bits_at(&self, row: i32, bit: u32, inf: usize) -> usize {
        let mut ret = 0usize;
        for r in row - 1..row + 2 {
            if r < 0 || r >= self.0.len() as i32 {
                ret = ret << 3 | inf;
                continue;
            }
            let r = r as usize;
            if bit > 0 {
                ret = ret << 3 | (self.0[r] >> (bit - 1) & 0b111) as usize;
                if bit == 127 && inf > 0 {
                    ret |= 0b100;
                }
            } else {
                ret = ret << 3 | (self.0[r] as usize & 0b011) << 1 | inf & 0b001
            }
        }
        ret
    }
}

fn part1(input: &Input) -> u32 {
    let (alg, img) = input;
    let e1 = img.enhance(alg, 0);
    let e2 = e1.enhance(alg, 0b111);
    println!("{:?}", img);
    println!("--");
    println!("{:?}", e1);
    println!("--");
    println!("{:?}", e2);

    e2.0.iter().fold(0, |cnt, row| cnt + row.count_ones())
}

fn part2(_input: &Input) -> usize {
    0
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 5884);
    assert_eq!(part2(&d), 1);
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
