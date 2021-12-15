use arrayvec::ArrayVec;
use itertools::Itertools;

use std::cmp::{max, min};
use std::fmt::{Debug, Display, Formatter};
use std::iter::Enumerate;
use std::ops::{Index, IndexMut};
use std::slice::ChunksExact;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn offset(&self, x: i32, y: i32) -> Coord {
        Coord::new(self.x + x, self.y + y)
    }

    pub fn line(&self, direction: (i32, i32)) -> LineIter {
        LineIter::new(direction, &self)
    }

    pub fn line_to(&self, end: Coord) -> LineTo {
        let dx = (end.x - self.x).signum();
        let dy = (end.y - self.y).signum();
        LineTo {
            curr: *self,
            direction: (dx, dy),
            end,
        }
    }
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { x, y })
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

pub struct LineTo {
    curr: Coord,
    direction: (i32, i32),
    end: Coord,
}

impl Iterator for LineTo {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let (dx, dy) = self.direction;
        if self.curr.x * dx > self.end.x * dx || self.curr.y * dy > self.end.y * dy {
            return None;
        }
        let next = Some(self.curr);
        self.curr.x += self.direction.0;
        self.curr.y += self.direction.1;
        next
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

    pub fn from_lines<L, S, P>(line_source: S, mut tile_parser: P) -> Self
    where
        L: AsRef<str>,
        S: IntoIterator<Item = L>,
        P: FnMut(char) -> T,
    {
        let mut tiles = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in line_source.into_iter() {
            height += 1;
            let line = line.as_ref();
            if width == 0 {
                tiles.reserve(line.len() * line.len());
            }
            width = tiles.len();
            tiles.extend(line.chars().map(|c| tile_parser(c)));
            width = tiles.len() - width;
        }
        assert!(width > 0);
        assert!(height > 0);
        assert_eq!(width * height, tiles.len());
        Grid::new(tiles, width as i32, height as i32)
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        match self.valid_coord(coord) {
            true => Some(&self.tiles[self.coord_to_idx(coord)]),
            false => None,
        }
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        match self.valid_coord(coord) {
            true => {
                let idx = self.coord_to_idx(coord);
                Some(&mut self.tiles[idx])
            }
            false => None,
        }
    }

    pub fn row_slices(&self) -> ChunksExact<'_, T> {
        self.tiles.as_slice().chunks_exact(self.width as usize)
    }

    /// Returns an iterator over all the tiles in the grid
    pub fn iter_tiles(&self) -> AllTiles<T> {
        AllTiles::new(self)
    }

    /// Returns an iterator giving the eight surrounding tiles, or less if at edge.
    pub fn neighbours(&self, coord: Coord) -> NeighboursIter<T> {
        NeighboursIter::new(coord, self)
    }

    pub fn neighbour_coords(&self, coord: Coord) -> NeighbourCoords {
        NeighbourCoords::new(coord, self.height, self.width)
    }

    /// Return iterator for the four neighbouring tiles on the main axes.
    pub fn updownleftright(&self, coord: Coord) -> UpDownLeftRight<T> {
        UpDownLeftRight::new(self, coord)
    }

    /// Returns an iterator for the tiles along the given line. `start` is not included
    /// in the set.
    pub fn line(&self, start: Coord, direction: (i32, i32)) -> GridLine<'_, T> {
        GridLine::new(self, start, direction)
    }

    /// Returns an iterator for all the coordinates in the grid
    pub fn coords(&self) -> AllCoords {
        AllCoords {
            width: self.width,
            height: self.height,
            next: Coord::new(0, 0),
        }
    }

    fn coord_to_idx(&self, coord: Coord) -> usize {
        if !self.valid_coord(coord) {
            panic!("Invalid coord to coord_to_idx {:?}", coord)
        }
        (coord.y * self.width + coord.x) as usize
    }

    fn idx_to_coord(&self, idx: usize) -> Coord {
        assert!(idx < self.tiles.len(), "Index outside of grid.");
        let y = idx / self.width as usize;
        let x = idx - y * self.width as usize;

        Coord {
            x: x as i32,
            y: y as i32,
        }
    }

    pub fn valid_coord(&self, c: Coord) -> bool {
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

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(1);
        for row in self.tiles.chunks_exact(self.width as usize) {
            for t in row {
                write!(f, "{:width$?} ", t, width = width)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.tiles[self.coord_to_idx(index)]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let idx = self.coord_to_idx(index);
        &mut self.tiles[idx]
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tiles = self.tiles.iter();
        for _y in 0..self.height {
            for t in (&mut tiles).take(self.width as usize) {
                t.fmt(f)?;
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

pub struct AllTiles<'a, T> {
    grid: &'a Grid<T>,
    iter: Enumerate<std::slice::Iter<'a, T>>,
}

impl<'a, T> AllTiles<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            iter: grid.tiles.iter().enumerate(),
        }
    }
}

impl<'a, T> Iterator for AllTiles<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((idx, tile)) = self.iter.next() {
            Some((self.grid.idx_to_coord(idx), tile))
        } else {
            None
        }
    }
}

pub struct AllCoords {
    width: i32,
    height: i32,
    next: Coord,
}

impl Iterator for AllCoords {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.y >= self.height {
            return None;
        }
        let ret = self.next;
        self.next.x += 1;
        if self.next.x >= self.width {
            self.next.x = 0;
            self.next.y += 1;
        }
        Some(ret)
    }
}

pub struct NeighboursIter<'a, T> {
    grid: &'a Grid<T>,
    center: Coord,
    next: Coord,
    x_max: i32,
    x_min: i32,
    y_max: i32,
}

impl<'a, T> NeighboursIter<'a, T> {
    fn new(coord: Coord, grid: &'a Grid<T>) -> Self {
        let x_min = max(0, coord.x - 1);
        let y_min = max(0, coord.y - 1);
        Self {
            grid,
            x_max: min(grid.width - 1, coord.x + 1),
            x_min,
            y_max: min(grid.height - 1, coord.y + 1),
            center: coord,
            next: Coord::new(x_min - 1, y_min),
        }
    }
}

impl<'a, T> Iterator for NeighboursIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.next.x += 1;
            if self.next.x > self.x_max {
                self.next.x = self.x_min;
                self.next.y += 1;
            }
            if self.next.y > self.y_max {
                return None;
            }
            if self.next == self.center {
                continue;
            }
            return Some(&self.grid[self.next]);
        }
    }
}

pub struct NeighbourCoords {
    center: Coord,
    next: Coord,
    x_max: i32,
    x_min: i32,
    y_max: i32,
}

impl NeighbourCoords {
    fn new(coord: Coord, height: i32, width: i32) -> Self {
        let x_min = max(0, coord.x - 1);
        let y_min = max(0, coord.y - 1);
        Self {
            x_max: min(width - 1, coord.x + 1),
            x_min,
            y_max: min(height - 1, coord.y + 1),
            center: coord,
            next: Coord::new(x_min - 1, y_min),
        }
    }
}

impl Iterator for NeighbourCoords {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.next.x += 1;
            if self.next.x > self.x_max {
                self.next.x = self.x_min;
                self.next.y += 1;
            }
            if self.next.y > self.y_max {
                return None;
            }
            if self.next == self.center {
                continue;
            }
            return Some(self.next);
        }
    }
}

pub struct UpDownLeftRight<'a, T> {
    grid: &'a Grid<T>,
    coords: ArrayVec<Coord, 4>,
}

impl<'a, T> UpDownLeftRight<'a, T> {
    fn new(grid: &'a Grid<T>, center: Coord) -> Self {
        let mut coords = ArrayVec::new();
        for x in [-1, 1] {
            let c = Coord {
                x: center.x + x,
                y: center.y,
            };
            if grid.valid_coord(c) {
                coords.push(c);
            }
        }
        for y in [-1, 1] {
            let c = Coord {
                x: center.x,
                y: center.y + y,
            };
            if grid.valid_coord(c) {
                coords.push(c);
            }
        }
        Self { grid, coords }
    }
}

impl<'a, T> Iterator for UpDownLeftRight<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.coords.pop()?;
        Some((c, &self.grid[c]))
    }
}

pub struct GridLine<'a, T> {
    grid: &'a Grid<T>,
    coord: Coord,
    direction: (i32, i32),
}

impl<'a, T> GridLine<'a, T> {
    fn new(grid: &'a Grid<T>, start: Coord, direction: (i32, i32)) -> Self {
        Self {
            grid,
            coord: start,
            direction,
        }
    }
}

impl<'a, T> Iterator for GridLine<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.coord.x += self.direction.0;
        self.coord.y += self.direction.1;
        if self.coord.x < 0
            || self.coord.x >= self.grid.width
            || self.coord.y < 0
            || self.coord.y >= self.grid.height
        {
            return None;
        }
        Some(&self.grid[self.coord])
    }
}
