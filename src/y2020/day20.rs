#![allow(unused_imports, unused_mut, unused_variables)]
use crate::GroupBlankLine;
use anyhow::private::kind::TraitKind;
use counter::Counter;
use itertools::Itertools;
use num_integer::Roots;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::Index;
use std::str::FromStr;

const INPUT_FILE: &str = "data/2020/day20.txt";

type Puzzle = Vec<Tile>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Puzzle {
    line_source.into_iter().group_by_blanks(parse_tile)
}

fn parse_tile<S: AsRef<str>>(iter: &mut dyn Iterator<Item = S>) -> Tile {
    let id = scan_fmt::scan_fmt!(iter.next().unwrap().as_ref(), "Tile {d}:", u16).unwrap();
    let mut lines = [Line(0); 10];
    for (n, s) in iter.enumerate() {
        lines[n] = s.as_ref().parse().unwrap()
    }
    Tile { id, lines }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Line(u16);

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for n in 0..10 {
            let c = match self.0 >> n & 1 == 1 {
                true => '#',
                false => '.',
            };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.chars().fold(0, |mut line, c| {
            line <<= 1;
            if c == '#' {
                line |= 1;
            }
            line
        });
        Ok(Line(x))
    }
}

impl Line {
    fn reverse(&self) -> Line {
        Self(self.0.reverse_bits() >> (16 - 10))
    }
}

struct Tile {
    id: u16,
    lines: [Line; 10],
}

impl Tile {
    fn col(&self, n: usize) -> Line {
        let mut line = 0;
        for row in &self.lines {
            line = line << 1 | (row.0 >> n & 1);
        }
        Line(line)
    }

    fn row(&self, n: usize) -> Line {
        self.lines[n]
    }

    fn edge(&self, side: Side) -> Edge {
        let id = self.id;
        match side {
            Side::N => Edge {
                id,
                side: Side::N,
                line: self.row(0),
            },
            Side::E => Edge {
                id,
                side: Side::E,
                line: self.col(9),
            },
            Side::S => Edge {
                id,
                side: Side::S,
                line: self.row(9),
            },
            Side::W => Edge {
                id,
                side: Side::W,
                line: self.col(0),
            },
        }
    }

    fn edges(&self) -> Vec<Edge> {
        vec![
            self.edge(Side::N),
            self.edge(Side::W),
            self.edge(Side::S),
            self.edge(Side::E),
        ]
    }

    fn bottom_edge(&self, trans: &TileTransform) -> Edge {
        self.edge(trans.up.opposite())
    }

    fn right_edge(&self, trans: &TileTransform) -> Edge {
        let right = if trans.flip {
            trans.up.left_of()
        } else {
            trans.up.right_of()
        };
        self.edge(right)
    }
}

type TileId = u16;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Side {
    N,
    E,
    S,
    W,
}

impl Side {
    fn left_of(&self) -> Self {
        use Side::*;
        match self {
            N => W,
            E => N,
            S => E,
            W => S,
        }
    }

    fn right_of(&self) -> Self {
        use Side::*;
        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }

    fn opposite(&self) -> Self {
        self.left_of().left_of()
    }
}

#[derive(Debug)]
struct Edge {
    id: TileId,
    line: Line,
    side: Side,
}

impl Edge {
    fn match_any_dir(&self, other: &Self) -> bool {
        self.id != other.id && (self.line == other.line || self.line.reverse() == other.line)
    }
}

fn collect_edges(tiles: &Puzzle) -> Vec<Edge> {
    let mut edges = Vec::with_capacity(tiles.len() * 4);
    for t in tiles {
        edges.extend(t.edges())
    }
    edges.sort_unstable_by(|a, b| a.line.cmp(&b.line).then(a.id.cmp(&b.id)));
    edges
}

fn get_corners(outer: &Vec<&Edge>) -> Vec<TileId> {
    let mut cnt = Counter::<_, usize>::new();
    cnt.extend(outer.iter().map(|e| e.id));
    cnt.iter()
        .filter(|(_a, &b)| b == 2)
        .map(|(&a, _)| a)
        .collect()
}

fn find_outer(edges: &Vec<Edge>) -> Vec<&Edge> {
    let mut ret = Vec::new();
    for n in 0..edges.len() {
        if !edges.iter().any(|e| e.match_any_dir(&edges[n])) {
            ret.push(&edges[n])
        }
    }
    ret
}

fn part1(tiles: &Puzzle) -> usize {
    get_corners(&find_outer(&collect_edges(tiles)))
        .iter()
        .map(|&a| a as usize)
        .product()
}

#[derive(Debug, Clone)]
struct TileTransform {
    id: TileId,
    up: Side,
    flip: bool,
}

fn get_matching_edges<'a>(
    edge: &Edge,
    iter: &mut dyn std::iter::Iterator<Item = &'a Edge>,
) -> Vec<&'a Edge> {
    iter.filter(|e| e.match_any_dir(edge)).collect()
}

type TileMap = Vec<Vec<Option<TileTransform>>>;

struct Tiling<'a> {
    map: TileMap,
    _p: PhantomData<&'a Tile>,
}

fn part2(tiles: &Puzzle) -> usize {
    let side_len = usize::sqrt(&tiles.len());

    let mut tiling: TileMap = vec![vec![Default::default(); side_len]; side_len];

    let tile_map: HashMap<TileId, &Tile> = tiles.iter().map(|t| (t.id, t)).collect();

    let edges = collect_edges(tiles);
    let outer_edges = find_outer(&edges);

    let mut remaining: HashSet<TileId> = tiles.iter().map(|t| t.id).collect();
    let mut placed: HashSet<TileId> = HashSet::new();

    let mut outer_tiles: HashSet<TileId> = HashSet::from_iter(outer_edges.iter().map(|e| e.id));

    let mut edges_outer_tiles: HashMap<TileId, Vec<Edge>> = outer_tiles
        .iter()
        .flat_map(|&t| tile_map.get(&t).unwrap().edges())
        .fold(HashMap::new(), |mut map, edge| {
            map.entry(edge.id).or_insert_with(Vec::new).push(edge);
            map
        });

    let top_left = tile_map[&get_corners(&outer_edges)[0]];
    outer_tiles.remove(&top_left.id);
    let fo = edges_outer_tiles.remove(&top_left.id).unwrap();
    let trans = TileTransform {
        id: top_left.id,
        up: fo[0].side,
        flip: fo[1].side != fo[0].side.left_of(),
    };
    let mut next_edge = top_left.bottom_edge(&trans);
    tiling[0][0] = Some(trans);
    for n in 1..side_len {
        for m in get_matching_edges(&next_edge, &mut edges_outer_tiles.values().flatten()) {}
    }
    0
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 144);
    assert_eq!(part1(&d), 29293767579581);
    // assert_eq!(part2(&d), 1);
}

#[test]
fn test_data() {
    let data = // Example data
"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 20899048083289);
    assert_eq!(part2(&d), 273);
}
