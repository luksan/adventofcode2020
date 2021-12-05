use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};

use crate::grid::*;

const INPUT_FILE: &str = "data/2020/day11.txt";

pub type Row = Vec<Tile>;
pub type TileGrid = Grid<Tile>;

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> TileGrid {
    let mut grid = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for l in line_source.into_iter() {
        let mut x = parse(l);
        width = x.len();
        height += 1;
        grid.append(&mut x);
    }

    Grid::new(grid, width as i32, height)
}

fn parse<S: AsRef<str>>(s: S) -> Row {
    s.as_ref().chars().map(|c| c.try_into().unwrap()).collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Floor,
    Chair,
    Occupied,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Floor),
            'L' => Ok(Tile::Chair),
            '#' => Ok(Tile::Occupied),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => ".",
                Tile::Chair => "L",
                Tile::Occupied => "#",
            }
        )
    }
}

pub fn part1(grid: &TileGrid) -> usize {
    let mut next = (*grid).clone();
    for _ in 0..100 {
        let grid = next;
        next = grid.clone();
        'coords: for coord in grid.coords() {
            match grid[&coord] {
                Tile::Floor => {}
                Tile::Occupied => {
                    let mut cnt = 0;
                    for t in grid.neighbours(&coord) {
                        match t {
                            Tile::Occupied => cnt += 1,
                            Tile::Floor => continue,
                            Tile::Chair => {}
                        }
                        if cnt >= 4 {
                            next[&coord] = Tile::Chair;
                            break;
                        }
                    }
                }
                Tile::Chair => {
                    for t in grid.neighbours(&coord) {
                        if matches!(t, &Tile::Occupied) {
                            continue 'coords;
                        }
                    }
                    next[&coord] = Tile::Occupied
                }
            }
        }
        if next == grid {
            break;
        }
    }
    next.iter_tiles()
        .filter(|(_c, &t)| t == Tile::Occupied)
        .count()
}

pub fn part2(input: &TileGrid) -> usize {
    let directions = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut next = (*input).clone();
    for _ in 0..100 {
        let grid = next;
        next = grid.clone();
        'next_tile: for c in grid.coords() {
            match grid[&c] {
                Tile::Floor => {}
                Tile::Chair => {
                    for d in &directions {
                        for t in grid.line(&c, *d) {
                            match t {
                                Tile::Occupied => continue 'next_tile,
                                Tile::Floor => continue,
                                Tile::Chair => break,
                            }
                        }
                    }
                    next[&c] = Tile::Occupied;
                }
                Tile::Occupied => {
                    let mut cnt = 0;
                    for d in &directions {
                        for t in grid.line(&c, *d) {
                            match t {
                                Tile::Occupied => cnt += 1,
                                Tile::Floor => continue,
                                Tile::Chair => {}
                            }
                            break;
                        }
                        if cnt >= 5 {
                            next[&c] = Tile::Chair;
                            break;
                        }
                    }
                }
            }
        }

        if next == grid {
            break;
        }
    }
    next.iter_tiles()
        .filter(|(_c, &t)| t == Tile::Occupied)
        .count()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.height(), 99);
    assert_eq!(part1(&d), 2453);
    assert_eq!(part2(&d), 2159);
}

#[test]
fn test_data() {
    let data = // Example data
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 37);
    assert_eq!(part2(&d), 26);
}
