use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter, Write};
use std::ops::Index;

const INPUT_FILE: &str = "data/day11.txt";

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn unpack(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn offset(&self, x: i32, y: i32) -> Coord {
        Coord::new(self.x + x, self.y + y)
    }

    pub fn line(&self, direction: (i32, i32)) -> LineIter {
        LineIter::new(direction, &self)
    }
}

pub struct LineIter {
    direction: (i32, i32),
    curr: Coord,
}

impl LineIter {
    pub fn new(direction: (i32, i32), start: &Coord) -> Self {
        Self {
            direction,
            curr: *start,
        }
    }
}

impl Iterator for LineIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.x += self.direction.0;
        self.curr.y += self.direction.1;
        Some(self.curr)
    }
}

pub struct Grid<T> {
    width: i32,
    height: i32,
    tiles: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(tiles: Vec<T>, width: i32, height: i32) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn get(&self, coord: &Coord) -> Option<&T> {
        match self.valid_coord(coord) {
            true => Some(&self.tiles[self.coord_to_idx(coord)]),
            false => None,
        }
    }

    pub fn get_mut(&mut self, coord: &Coord) -> Option<&mut T> {
        match self.valid_coord(coord) {
            true => {
                let idx = self.coord_to_idx(coord);
                Some(&mut self.tiles[idx])
            }
            false => None,
        }
    }

    pub fn neighbours(&self, coord: &Coord) -> NeighboursIter<T> {
        NeighboursIter::new(coord, self)
    }

    pub fn line(&self, start: &Coord, direction: &(i32, i32)) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(
            start
                .line(*direction)
                .take_while(move |&coord| self.valid_coord(&coord))
                .map(move |coord| &self[&coord]),
        )
    }

    pub fn coords(&self) -> Box<dyn Iterator<Item = Coord> + '_> {
        Box::new((0..self.height).flat_map(move |y| (0..self.width).map(move |x| Coord::new(x, y))))
    }

    fn coord_to_idx(&self, coord: &Coord) -> usize {
        if !self.valid_coord(coord) {
            panic!("Invalid coord to coord_to_idx")
        }
        (coord.y * self.width + coord.x) as usize
    }

    pub fn valid_coord(&self, c: &Coord) -> bool {
        !(c.x < 0 || c.x >= self.width || c.y < 0 || c.y >= self.height)
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            tiles: self.tiles.clone(),
            ..*self
        }
    }
}

impl<T> Index<&Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Coord) -> &Self::Output {
        &self.tiles[self.coord_to_idx(index)]
    }
}
impl Display for TileGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tiles = self.tiles.iter();
        for _y in 0..self.height {
            for t in (&mut tiles).take(self.width as usize) {
                f.write_char(match t {
                    Tile::Floor => '.',
                    Tile::Chair => 'L',
                    Tile::Occupied => '#',
                })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: PartialEq> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}

pub struct NeighboursIter<'a, T> {
    grid: &'a Grid<T>,
    center: Coord,
    next: Coord,
}

impl<'a, T> NeighboursIter<'a, T> {
    fn new(coord: &Coord, grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            center: *coord,
            next: coord.offset(-2, -1),
        }
    }
}

impl<'a, T> Iterator for NeighboursIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.x += 1;
        if self.next.x > self.center.x + 1 {
            self.next.x = self.center.x - 1;
            self.next.y += 1;
        }
        if self.next.y > self.center.y + 1 {
            return None;
        }
        if self.next == self.center {
            return self.next();
        }
        match self.grid.get(&self.next) {
            None => self.next(),
            t => t,
        }
    }
}

fn count_surrounding_occupied(chair: &Coord, grid: &TileGrid) -> u8 {
    assert_ne!(grid.get(chair).unwrap(), &Tile::Floor);
    grid.neighbours(chair)
        .filter(|&n| n == &Tile::Occupied)
        .count() as u8
}

pub fn part1(grid: &TileGrid) -> usize {
    let mut next = (*grid).clone();
    for _ in 0..10000 {
        let grid = next;
        next = grid.clone();
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let coord = &Coord::new(x, y);
                match grid.get(coord).unwrap() {
                    Tile::Floor => {}
                    Tile::Occupied => {
                        if count_surrounding_occupied(coord, &grid) >= 4 {
                            *next.get_mut(coord).unwrap() = Tile::Chair;
                        }
                    }
                    Tile::Chair => {
                        if count_surrounding_occupied(coord, &grid) == 0 {
                            *next.get_mut(coord).unwrap() = Tile::Occupied;
                        }
                    }
                }
            }
        }
        if next == grid {
            break;
        }
    }
    next.tiles.iter().filter(|t| *t == &Tile::Occupied).count()
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
            match grid.get(&c).unwrap() {
                Tile::Floor => {}
                Tile::Chair => {
                    for d in &directions {
                        for t in grid.line(&c, d) {
                            match t {
                                Tile::Occupied => continue 'next_tile,
                                Tile::Floor => continue,
                                Tile::Chair => break,
                            }
                        }
                    }
                    *next.get_mut(&c).unwrap() = Tile::Occupied;
                }
                Tile::Occupied => {
                    let mut cnt = 0;
                    for d in &directions {
                        for t in grid.line(&c, d) {
                            match t {
                                Tile::Occupied => cnt += 1,
                                Tile::Floor => continue,
                                Tile::Chair => {}
                            }
                            break;
                        }
                    }
                    if cnt >= 5 {
                        *next.get_mut(&c).unwrap() = Tile::Chair;
                    }
                }
            }
        }

        if next == grid {
            break;
        }
    }
    next.tiles.iter().filter(|t| *t == &Tile::Occupied).count()
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
    //assert_eq!(part1(&d), 37);
    assert_eq!(part2(&d), 26);
}
