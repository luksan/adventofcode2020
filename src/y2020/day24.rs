use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::str::FromStr;

const INPUT_FILE: &str = "data/2020/day24.txt";

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Vec<HexCoord> {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> HexCoord {
    s.as_ref().parse::<HexCoord>().unwrap()
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct HexCoord {
    x: i16,
    y: i16,
}

impl FromStr for HexCoord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            unreachable!("Nein!")
        }
        let mut x = 0;
        let mut y = 0;
        let mut chars = s.chars();
        while let Some(c1) = chars.next() {
            match c1 {
                'e' => {
                    x += 2;
                    continue;
                }
                'w' => {
                    x -= 2;
                    continue;
                }
                'n' => y += 1,
                's' => y -= 1,
                _ => unreachable!("Bad input"),
            }
            match chars.next().unwrap() {
                'e' => x += 1,
                'w' => x -= 1,
                _ => unreachable!("Bad 2nd char."),
            }
        }
        Ok(HexCoord { x, y })
    }
}

impl HexCoord {
    fn adjacent(&self) -> AdjIter {
        AdjIter::new(self)
    }
}

struct AdjIter {
    coords: [HexCoord; 6],
    next: usize,
}

impl AdjIter {
    fn new(center: &HexCoord) -> AdjIter {
        let coords = [(2, 0), (-2, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)]
            .iter()
            .map(|(dx, dy)| HexCoord {
                x: center.x + dx,
                y: center.y + dy,
            })
            .collect_vec();
        AdjIter {
            coords: coords.try_into().unwrap(),
            next: 0,
        }
    }
}

impl Iterator for AdjIter {
    type Item = HexCoord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.coords.len() {
            None
        } else {
            let n = self.next;
            self.next += 1;
            Some(self.coords[n])
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum Color {
    Black,
    White,
}

impl Color {
    fn flip(&mut self) {
        match self {
            Color::Black => *self = Color::White,
            Color::White => *self = Color::Black,
        }
    }
}

fn part1(coords: &[HexCoord]) -> usize {
    let mut tiling: HashMap<&HexCoord, Color> = HashMap::new();
    for c in coords {
        tiling.entry(c).or_insert(Color::White).flip();
    }

    tiling
        .values()
        .filter(|t| matches!(t, Color::Black))
        .count()
}

fn part2(coords: &[HexCoord]) -> usize {
    let mut black = HashSet::new();
    for c in coords {
        if !black.insert(*c) {
            black.remove(c);
        }
    }
    let mut white = HashMap::with_capacity(black.len());

    for _day in 0..100 {
        let mut new_black = black.clone();
        for b in &black {
            let mut blk_cnt = 0;
            for n in b.adjacent() {
                if black.contains(&n) {
                    blk_cnt += 1;
                } else {
                    white.entry(n).and_modify(|c| *c += 1).or_insert(1);
                }
            }
            if blk_cnt == 0 || blk_cnt > 2 {
                new_black.remove(b);
            }
        }
        for (w, blk_cnt) in white.drain() {
            if blk_cnt == 2 {
                new_black.insert(w);
            }
        }
        black = new_black;
    }

    black.len()
}

#[test]
fn test_small() {
    assert_eq!(
        "nwwswee".parse::<HexCoord>().unwrap(),
        HexCoord { x: 0, y: 0 }
    );
    assert_eq!(
        "esew".parse::<HexCoord>().unwrap(),
        HexCoord { x: 1, y: -1 }
    );
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(part1(&d), 473);
    assert_eq!(part2(&d), 4070);
}

#[test]
fn test_data() {
    let data = // Example data
"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 10);
    assert_eq!(part2(&d), 2208);
}
