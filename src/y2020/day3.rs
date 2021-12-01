use std::ops::Index;

fn load_input() -> Vec<TreeLine> {
    crate::load_input("data/2020/day3.txt", parse)
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

#[derive(PartialEq, Copy, Clone, Debug)]
enum Tile {
    Tree,
    Open,
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

fn part1(forest: &[TreeLine]) -> usize {
    Slope::new(forest, 1, 3)
        .filter(|&tile| tile == Tile::Tree)
        .count()
}

fn part2(forest: &[TreeLine]) -> usize {
    [(1usize, 1usize), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|(down, right)| Slope::new(forest, *down, *right))
        .fold(1, |prod, slope| {
            slope.filter(|&tile| tile == Tile::Tree).count() * prod
        })
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
    let forest = load_input();
    assert_eq!(part1(&forest), 282);
    assert_eq!(part2(&forest), 958815792);
}
