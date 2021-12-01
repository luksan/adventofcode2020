use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::str::FromStr;

const INPUT_FILE: &str = "data/2020/day24.txt";

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Vec<Coord> {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Coord {
    s.as_ref().parse::<Coord>().unwrap()
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl FromStr for Coord {
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
        Ok(Coord { x, y })
    }
}

impl Coord {
    fn adjacent(&self) -> AdjIter {
        AdjIter::new(self)
    }
}

struct AdjIter {
    coords: [Coord; 6],
    next: usize,
}

impl AdjIter {
    fn new(center: &Coord) -> AdjIter {
        let coords = [(2, 0), (-2, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)]
            .iter()
            .map(|(dx, dy)| Coord {
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
    type Item = Coord;

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

fn part1(coords: &[Coord]) -> usize {
    let mut tiling: HashMap<&Coord, Color> = HashMap::new();
    for c in coords {
        tiling.entry(c).or_insert(Color::White).flip();
    }

    tiling
        .values()
        .filter(|t| matches!(t, Color::Black))
        .count()
}

fn part2(coords: &[Coord]) -> usize {
    let mut tiling: HashMap<Coord, Color> = HashMap::new();
    for c in coords {
        tiling.entry(*c).or_insert(Color::White).flip();
    }

    for _day in 0..100 {
        let mut new_tiling = tiling.clone();

        let mut white: HashSet<Coord> = HashSet::new();

        for (pos, color) in &tiling {
            let mut black = 0;
            for adj in pos.adjacent() {
                match tiling.get(&adj) {
                    Some(Color::Black) => black += 1,
                    _ => {
                        white.insert(adj);
                    }
                }
            }
            match color {
                Color::Black => {
                    if black == 0 || black > 2 {
                        new_tiling.get_mut(pos).unwrap().flip()
                    }
                }
                Color::White => {
                    white.insert(*pos);
                }
            }
        }

        for c in white {
            let black = c
                .adjacent()
                .filter(|c| matches!(tiling.get(c), Some(Color::Black)))
                .count();
            if black == 2 {
                new_tiling.entry(c).or_insert(Color::White).flip();
            }
        }
        tiling = new_tiling;
    }

    tiling
        .values()
        .filter(|t| matches!(t, Color::Black))
        .count()
}

#[test]
fn test_small() {
    assert_eq!("nwwswee".parse::<Coord>().unwrap(), Coord { x: 0, y: 0 });
    assert_eq!("esew".parse::<Coord>().unwrap(), Coord { x: 1, y: -1 });
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
