use arrayvec::ArrayVec;
use std::collections::{HashMap, HashSet};
use std::ops::Index;
use std::str::FromStr;

const INPUT_FILE: &str = "data/2020/day24.txt";

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Vec<HexCoord> {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> HexCoord {
    s.as_ref().parse::<HexCoord>().unwrap()
}

// https://www.redblobgames.com/grids/hexagons/
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct HexCoord {
    q: i16,
    r: i16,
}

impl FromStr for HexCoord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            unreachable!("Nein!")
        }
        let mut q = 0;
        let mut r = 0;
        let mut chars = s.chars();
        while let Some(c1) = chars.next() {
            match c1 {
                'e' => {
                    q += 1;
                }
                'w' => {
                    q -= 1;
                }
                'n' => {
                    r -= 1;
                    if matches!(chars.next(), Some('e')) {
                        q += 1;
                    }
                }
                's' => {
                    r += 1;
                    if matches!(chars.next(), Some('w')) {
                        q -= 1;
                    }
                }
                _ => unreachable!("Bad input"),
            }
        }
        Ok(HexCoord { q, r })
    }
}

impl HexCoord {
    fn adjacent(&self) -> [HexCoord; 6] {
        /*
        assert!(self.q > -70);
        assert!(self.r > -70);
        assert!(self.q < 70);
        assert!(self.r < 70);
        */

        let mut ret = ArrayVec::new();
        let offsets = [(0, -1), (1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0)];
        for o in offsets {
            ret.push(Self {
                q: self.q + o.0,
                r: self.r + o.1,
            });
        }

        unsafe { ret.into_inner_unchecked() }
    }

    const MAX_RADIUS: i16 = 70;
    const STRIDE: usize = Self::MAX_RADIUS as usize * 2 + 1;
    fn as_index(&self) -> usize {
        let r_off = (self.r + Self::MAX_RADIUS) as usize;
        let q_off = (self.q + Self::MAX_RADIUS) as usize;
        q_off * Self::STRIDE as usize + r_off
    }

    fn from_index(idx: usize) -> Self {
        let r_off = idx % Self::STRIDE;
        let q_off = (idx - r_off) / Self::STRIDE as usize;
        Self {
            q: q_off as i16 - Self::MAX_RADIUS,
            r: r_off as i16 - Self::MAX_RADIUS,
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

struct Floor(Vec<u8>);

impl Floor {
    fn new() -> Self {
        Self(vec![
            0;
            HexCoord::MAX_RADIUS as usize * 2 * HexCoord::STRIDE + 1
        ])
    }

    fn flip(&mut self, coord: HexCoord) -> u8 {
        let c = &mut self.0[coord.as_index()];
        let r = *c;
        if *c > 0 {
            *c = 0;
        } else {
            *c = 1;
        }
        r
    }

    fn contains(&self, coord: HexCoord) -> bool {
        self.0[coord.as_index()] > 0
    }

    fn count_white(&mut self, coord: HexCoord) {
        self.0[coord.as_index()] += 1;
    }
}

impl Index<HexCoord> for Floor {
    type Output = u8;

    fn index(&self, index: HexCoord) -> &Self::Output {
        &self.0[index.as_index()]
    }
}

fn part2(coords: &[HexCoord]) -> usize {
    let mut black = HashSet::new();
    let mut floor = Floor::new();
    for c in coords {
        if !black.insert(*c) {
            black.remove(c);
        }
        floor.flip(*c);
    }

    let mut white_count = Floor::new();
    let mut white_list = vec![];
    let mut flip_to_white = vec![];

    for _day in 0..100 {
        let mut new_black = black.clone();
        for b in &black {
            let mut blk_cnt = 0;
            for n in b.adjacent() {
                if floor.contains(n) {
                    blk_cnt += 1;
                } else {
                    white_count.count_white(n);
                    white_list.push(n);
                }
            }
            if blk_cnt == 0 || blk_cnt > 2 {
                new_black.remove(b);
                flip_to_white.push(*b);
            }
        }
        for b in flip_to_white.drain(..) {
            floor.flip(b);
        }
        for w in white_list.drain(..) {
            if white_count[w] == 0 {
                continue;
            }
            if white_count.flip(w) == 2 {
                new_black.insert(w);
                floor.flip(w);
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
        HexCoord { q: 0, r: 0 }
    );
    assert_eq!("esew".parse::<HexCoord>().unwrap(), HexCoord { r: 1, q: 0 });
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
