#![allow(unused_imports, unused_mut, unused_variables)]

use crate::GroupBlankLine;

use anyhow::private::kind::TraitKind;
use counter::Counter;
use itertools::Itertools;
use num_integer::Roots;

use std::fmt::{Debug, Display, Formatter};
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
        for n in (0..10).rev() {
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
        let x = s.bytes().fold(0, |mut line, c| {
            line <<= 1;
            if c == b'#' {
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

    fn inner(&self) -> u8 {
        ((self.0 >> 1) & 0xFF) as u8
    }

    fn get(&self, pos: usize) -> bool {
        self.0 >> (9 - pos) & 1 == 1
    }
}

#[test]
fn test_line() {
    let s = ".##...#.##";
    let l: Line = s.parse().unwrap();
    assert_eq!(format!("{}", l), s);
    assert_eq!(l.0 & 1, 1);
    assert!(!l.get(0));
    assert!(l.get(9));
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Tile {
    id: u16,
    lines: [Line; 10],
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile #{}: ", self.id)?;
        for s in Side::all_sides() {
            write!(f, "{:?} ", self.edge(s))?;
        }
        Ok(())
    }
}

impl Tile {
    fn col(&self, n: usize) -> Line {
        let mut line = 0;
        for row in &self.lines {
            line = line << 1 | row.get(n) as u16;
        }
        Line(line)
    }

    fn row(&self, n: usize) -> Line {
        self.lines[n]
    }

    fn edge(&self, side: Side) -> Edge {
        let tile = self;
        match side {
            Side::N => Edge {
                tile,
                side: Side::N,
                line: self.row(0),
            },
            Side::E => Edge {
                tile,
                side: Side::E,
                line: self.col(9),
            },
            Side::S => Edge {
                tile,
                side: Side::S,
                line: self.row(9),
            },
            Side::W => Edge {
                tile,
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
    fn all_sides() -> [Side; 4] {
        [Side::N, Side::E, Side::S, Side::W]
    }

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

#[derive(Clone)]
struct Edge<'a> {
    tile: &'a Tile,
    line: Line,
    side: Side,
}

impl Debug for Edge<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.side, self.line)
    }
}
impl PartialEq for Edge<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line
    }
}

impl Edge<'_> {
    fn match_any_dir(&self, other: &Self) -> bool {
        self.tile.id != other.tile.id
            && (self.line == other.line || self.line.reverse() == other.line)
    }
}

fn collect_edges(tiles: &Puzzle) -> Vec<Edge> {
    let mut edges = Vec::with_capacity(tiles.len() * 4);
    for t in tiles {
        edges.extend(t.edges())
    }
    edges.sort_by(|a, b| a.line.cmp(&b.line).then(a.tile.id.cmp(&b.tile.id)));
    edges
}

fn get_corners<'a, 'b>(outer: &'b [Edge<'a>]) -> Vec<&'a Tile> {
    let mut cnt = Counter::<_, usize>::new();
    cnt.extend(outer.iter().map(|e| e.tile));
    let mut c = cnt
        .iter()
        .filter(|(_a, &b)| b == 2)
        .map(|(&a, _)| a)
        .collect_vec();
    c.sort_by_key(|t| t.id);
    c
}

fn find_outer<'a, 'b>(edges: &'a [Edge<'b>]) -> Vec<Edge<'b>> {
    let mut ret = Vec::new();
    for n in 0..edges.len() {
        if !edges.iter().any(|e| e.match_any_dir(&edges[n])) {
            ret.push(edges[n].clone())
        }
    }
    ret
}

fn part1(tiles: &Puzzle) -> usize {
    let outer = find_outer(&collect_edges(tiles));
    get_corners(&outer)
        .iter()
        .map(|&tile| tile.id as usize)
        .product()
}

#[derive(Copy, Clone)]
struct TileTransform<'a> {
    tile: &'a Tile,
    up: Side,   // indicates rotation by which edge is up
    flip: bool, // Flip  along vertical axis
}

impl Debug for TileTransform<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tile #{}: up: {:?}, flipped: {}",
            self.tile.id, self.up, self.flip
        )
    }
}

impl<'a> TileTransform<'a> {
    fn edge(&self, mut transformed_side: Side) -> Edge {
        if self.flip && matches!(transformed_side, Side::E | Side::W) {
            transformed_side = transformed_side.opposite();
        }
        let rot_flip = match self.up {
            Side::N => false,
            Side::E => {
                matches!(transformed_side, Side::E | Side::W)
            }
            Side::S => true,
            Side::W => {
                matches!(transformed_side, Side::N | Side::S)
            }
        };

        let up = self.up;
        let actual_side = match transformed_side {
            Side::N => up,
            Side::E => up.right_of(),
            Side::S => up.opposite(),
            Side::W => up.left_of(),
        };
        let mut edge = self.tile.edge(actual_side);
        if rot_flip {
            edge.line = edge.line.reverse();
        }
        if self.flip && matches!(transformed_side, Side::N | Side::S) {
            edge.line = edge.line.reverse();
        }
        edge
    }

    fn row(&self, row: usize) -> Line {
        match self.up {
            Side::N => {
                let r = self.tile.row(row);
                if self.flip {
                    r.reverse()
                } else {
                    r
                }
            }
            Side::E => {
                let r = self.tile.col(9 - row);
                if self.flip {
                    r.reverse()
                } else {
                    r
                }
            }
            Side::S => {
                let r = self.tile.row(9 - row);
                if !self.flip {
                    r.reverse()
                } else {
                    r
                }
            }
            Side::W => {
                let r = self.tile.col(row);
                if self.flip {
                    r
                } else {
                    r.reverse()
                }
            }
        }
    }
}

fn get_matching_edges<'a>(
    edge: &Edge,
    iter: &mut dyn std::iter::Iterator<Item = &'a Edge>,
) -> Vec<&'a Edge<'a>> {
    iter.filter(|e| e.match_any_dir(edge)).collect()
}

type TileMap<'a> = Vec<Vec<Option<TileTransform<'a>>>>;

struct Tiling<'a> {
    map: TileMap<'a>,
    _p: PhantomData<&'a Tile>,
}

fn find_and_align_edge<'a, 'b>(
    needle: &TileTransform<'a>,
    needle_side: Side,
    mut edges: impl Iterator<Item = &'b Edge<'a>>,
) -> TileTransform<'a>
where
    'a: 'b,
{
    // get needle edge, considering transform
    let old_edge = needle.edge(needle_side);
    let new_edges = edges.filter(|e| e.match_any_dir(&old_edge)).collect_vec();
    if new_edges.is_empty() {
        println!("No match for {:?} at side {:?}", needle, needle_side);
    }
    assert_eq!(new_edges.len(), 1, "Didn't find exactly one matching edge.");
    let new_edge = new_edges[0];
    //let new_edge = edges.find(|e| e.match_any_dir(&old_edge)).unwrap();
    let new_up = match needle_side {
        Side::N => new_edge.side.opposite(),
        Side::E => new_edge.side.right_of(), // Rotate so that the west side is aligned to the needle edge
        Side::S => new_edge.side,
        Side::W => new_edge.side.left_of(),
    };

    let mut new_trans = TileTransform {
        tile: new_edge.tile,
        up: new_up,
        flip: false,
    };
    if new_trans.edge(needle_side.opposite()).line != old_edge.line {
        new_trans.flip = true;
        if matches!(needle_side, Side::E | Side::W) {
            new_trans.up = new_trans.up.opposite();
        }
    } /*
      println!("{:?}", needle);
      for d in [Side::N, Side::E, Side::S, Side::W] {
          print!("{:?} ", needle.edge(d));
      }
      println!();
      println!("{:?}", new_trans);
      for d in [Side::N, Side::E, Side::S, Side::W] {
          print!("{:?} ", new_trans.edge(d));
      }
      println!();
      println!(
          "{:?} {:?} ({:?}) {:?}",
          needle_side,
          old_edge.line,
          old_edge.line.reverse(),
          new_trans.edge(needle_side.opposite()).line
      );*/
    assert_eq!(new_trans.edge(needle_side.opposite()).line, old_edge.line);

    new_trans
}

fn solve_puzzle(tiles: &Puzzle) -> TileMap {
    let side_len = tiles.len().sqrt();

    let mut tiling: TileMap = vec![vec![Default::default(); side_len]; side_len];

    let mut edges = collect_edges(tiles);
    let outer_edges = find_outer(&edges);
    assert_eq!(outer_edges.len(), side_len * 4);

    // Find and align first corner
    let first_corner = get_corners(&outer_edges)[1];
    // assert_eq!(first_corner.id, 1951);
    // rotate so up / left is outer
    let mut trans = TileTransform {
        tile: first_corner,
        up: Side::S,
        flip: true,
    };

    assert_eq!(trans.edge(Side::N).side, Side::S);
    assert_eq!(trans.edge(Side::W).side, Side::W);

    assert!(outer_edges
        .iter()
        .any(|e| e.line == trans.edge(Side::N).line));
    assert!(outer_edges
        .iter()
        .any(|e| e.line == trans.edge(Side::W).line.reverse()));

    while !outer_edges
        .iter()
        .any(|e| e.line == trans.edge(Side::N).line)
        || !outer_edges
            .iter()
            .any(|e| e.line == trans.edge(Side::W).line.reverse())
    {
        trans.up = trans.up.right_of()
    }

    edges.retain(|e| e.tile.id != trans.tile.id);
    tiling[0][0] = Some(trans);

    for row in 0..side_len {
        if row > 0 {
            let new =
                find_and_align_edge(tiling[row - 1][0].as_ref().unwrap(), Side::S, edges.iter());
            edges.retain(|e| e.tile.id != new.tile.id);
            tiling[row][0] = Some(new);
        }
        for col in 1..side_len {
            // println!("row {} col {}", row, col);
            let new = find_and_align_edge(
                tiling[row][col - 1].as_ref().unwrap(),
                Side::E,
                edges.iter(),
            );
            edges.retain(|e| e.tile.id != new.tile.id);
            tiling[row][col] = Some(new);
        }
    }

    tiling
}

fn print_pic(pic: &[u128]) {
    for row in pic {
        for n in (0..(8 * 3)).rev() {
            if row >> n & 1 == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_tiling(tiles: &TileMap) {
    for row in tiles {
        for line in 0..10 {
            for tile in row {
                print!("{} ", tile.unwrap().row(line));
            }
            println!()
        }
        println!()
    }
}
fn part2(puzzle: &Puzzle) -> usize {
    let tiles = solve_puzzle(puzzle);
    // trim edges and merge

    print_tiling(&tiles);

    let mut picture = vec![0u128; tiles.len() * 8];
    for row in 0..tiles.len() {
        for tile in &tiles[row] {
            for line in 0..8 {
                picture[row * 8 + line] =
                    picture[row * 8 + line] << 8 | tile.unwrap().row(line + 1).inner() as u128;
            }
        }
    }

    print_pic(&picture);
    0
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 144);
    assert_eq!(part1(&d), 29293767579581);
    assert_eq!(part2(&d), 1);
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
