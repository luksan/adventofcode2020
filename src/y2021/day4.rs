use crate::grid::{Coord, Grid};
use crate::GroupBlankLine;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

type Game = (Vec<u8>, Vec<Board>);

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Game {
    let mut lines = line_source.into_iter();
    let draw: Vec<u8> = lines
        .next()
        .unwrap()
        .as_ref()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    (draw, lines.group_by_blanks(parse_board))
}

fn parse_board<S: AsRef<str>>(iter: &mut dyn Iterator<Item = S>) -> Board {
    let mut brd = Vec::with_capacity(25);
    for line in iter {
        brd.extend(line.as_ref().split_whitespace().map(|n| n.parse().unwrap()));
    }
    Board {
        grid: Grid::new(brd, 5, 5),
        won: false,
    }
}

#[derive(Clone)]
struct Number {
    n: u8,
    marked: bool,
}

impl PartialEq<u8> for Number {
    fn eq(&self, other: &u8) -> bool {
        self.n == *other
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:>2}", self.n);
        if self.marked {
            write!(f, "{}", ansi_term::Style::new().bold().underline().paint(s))
        } else {
            write!(f, "{}", s)
        }
    }
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            n: s.parse().unwrap(),
            marked: false,
        })
    }
}

#[derive(Debug, Clone)]
struct Board {
    grid: Grid<Number>,
    won: bool,
}

impl Board {
    fn mark_number(&mut self, num: u8) -> bool {
        if let Some((c, _)) = self.grid.iter_tiles().find(|(_c, t)| t.n == num) {
            self.grid[c].marked = true;
            self.check_win(&c)
        }
        self.won
    }

    fn check_win(&mut self, coord: &Coord) {
        let hc = Coord { x: -1, y: coord.y };
        let vc = Coord { x: coord.x, y: -1 };
        self.won = self.grid.line(hc, (1, 0)).all(|t| t.marked)
            || self.grid.line(vc, (0, 1)).all(|t| t.marked);
    }

    fn final_score(&self, n: usize) -> usize {
        self.grid
            .iter_tiles()
            .filter_map(|(_c, t)| if !t.marked { Some(t.n as usize) } else { None })
            .sum::<usize>()
            * n
    }
}

fn part1(game: &Game) -> usize {
    let (draw, boards) = game;
    let mut boards = (*boards).clone();
    for n in draw {
        for board in boards.iter_mut() {
            if board.mark_number(*n) {
                return board.final_score(*n as usize);
            }
        }
    }
    0
}

fn part2(game: &Game) -> usize {
    let (draw, boards) = game;
    let mut boards = (*boards).clone();
    for n in draw {
        let bc = boards.len();
        for board in boards.iter_mut() {
            if board.mark_number(*n) && bc == 1 {
                return board.final_score(*n as usize);
            }
        }
        boards.retain(|b| !b.won);
    }
    0
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 35670);
    assert_eq!(part2(&d), 22704);
}

#[test]
fn test_data() {
    let data = // Example data
"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
    let d = load_input(data.lines());
    assert_eq!(d.1.len(), 3);
    assert_eq!(part1(&d), 4512);
    assert_eq!(part2(&d), 1924);
}
