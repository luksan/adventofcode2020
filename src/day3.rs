use adventofcode2020::DayOfAdvent;
use std::ops::Index;

fn load_input() -> Vec<TreeLine> {
    adventofcode2020::load_input("data/day3.txt", parse)
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Tile {
    Tree,
    Open,
}

fn parse(s: String) -> TreeLine {
    TreeLine(
        s.chars()
            .map(|c| match c {
                '.' => Tile::Open,
                '#' => Tile::Tree,
                _ => unreachable!("Bad tile input"),
            })
            .collect(),
    )
}

struct TreeLine(Vec<Tile>);

impl Index<usize> for TreeLine {
    type Output = Tile;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index.rem_euclid(self.0.len())]
    }
}

struct Slope<'a> {
    forest: &'a [TreeLine],
    down: usize,
    right: usize,
    pos: (usize, usize),
}

impl<'a> Slope<'a> {
    fn new(forest: &'a [TreeLine], down: usize, right: usize) -> Self {
        Self {
            forest,
            down,
            right,
            pos: (0, 0),
        }
    }
}

impl Iterator for Slope<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.forest.get(self.pos.0).map(|line| line[self.pos.1]);
        self.pos = (self.pos.0 + self.down, self.pos.1 + self.right);
        ret
    }
}

struct Solver {
    forest: Vec<TreeLine>,
    p1: usize,
    p2: usize,
}
impl Solver {
    fn new(forest: Vec<TreeLine>) -> Self {
        Self {
            forest,
            p1: 0,
            p2: 0,
        }
    }

    fn part1(&mut self) {
        self.p1 = Slope::new(&self.forest, 1, 3)
            .filter(|&tile| tile == Tile::Tree)
            .count();
    }

    fn part2(&mut self) {
        self.p2 = [(1usize, 1usize), (1, 3), (1, 5), (1, 7), (2, 1)]
            .iter()
            .map(|(down, right)| Slope::new(&self.forest, *down, *right))
            .fold(1, |prod, slope| {
                slope.filter(|&tile| tile == Tile::Tree).count() * prod
            });
    }
}
pub fn solve() -> Box<dyn DayOfAdvent> {
    let mut x = Solver::new(load_input());

    x.part1();

    Box::new(x)
}

impl DayOfAdvent for Solver {
    fn day(&self) -> u32 {
        3
    }

    fn result_strings(&self) -> Vec<String> {
        let ret = Vec::new();

        ret
    }
}

#[test]
fn load_data() {
    let x = load_input();
    assert_eq!(x.len(), 323);
    assert_eq!(x.last().unwrap().0.len(), 31);

    assert_eq!(x[0][0], x[0][31]);
}

#[test]
fn test_real_data() {
    let mut s = Solver::new(load_input());
    s.part1();
    s.part2();
    assert_eq!(s.p1, 282);
    assert_eq!(s.p2, 958815792);
}
