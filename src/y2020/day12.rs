use std::ops::{AddAssign, Mul, MulAssign};
use std::str::FromStr;

const INPUT_FILE: &str = "data/day12.txt";

pub type ActionList = Vec<Action>;

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> ActionList {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Action {
    s.as_ref().parse::<Action>().unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    Move((i32, i32)),
    Fwd(i32),
    Rotate(Rotation),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Action::*;
        let cmd = s.chars().next().unwrap();
        let d = s[1..].parse().unwrap();

        Ok(match cmd {
            'N' => Move((0, d)),
            'S' => Move((0, -d)),
            'E' => Move((d, 0)),
            'W' => Move((-d, 0)),
            'F' => Fwd(d),
            'L' => Rotate(match d {
                90 => Rotation::Left,
                180 => Rotation::Reverse,
                270 => Rotation::Right,
                _ => return Err(()),
            }),
            'R' => Rotate(match d {
                90 => Rotation::Right,
                180 => Rotation::Reverse,
                270 => Rotation::Left,
                _ => return Err(()),
            }),
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Rotation {
    Left,
    Reverse,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Pos(i32, i32);

impl AddAssign<(i32, i32)> for Pos {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
    }
}

impl MulAssign<Rotation> for Pos {
    fn mul_assign(&mut self, rhs: Rotation) {
        *self = match rhs {
            Rotation::Left => Pos(-self.1, self.0),
            Rotation::Reverse => Pos(-self.0, -self.1),
            Rotation::Right => Pos(self.1, -self.0),
        };
    }
}

impl Mul<i32> for Pos {
    type Output = (i32, i32);

    fn mul(self, rhs: i32) -> Self::Output {
        (self.0 * rhs, self.1 * rhs)
    }
}

pub fn part1(actions: &ActionList) -> i32 {
    let mut pos = Pos(0, 0);
    let mut direction = Pos(1, 0);
    for action in actions {
        match action {
            Action::Move(dist) => pos += *dist,
            Action::Fwd(d) => pos += direction * *d,
            Action::Rotate(rot) => direction *= *rot,
        }
    }
    pos.0.abs() + pos.1.abs()
}

pub fn part2(actions: &ActionList) -> i32 {
    let mut pos = Pos(0, 0);
    let mut waypoint = Pos(10, 1);
    for action in actions {
        match action {
            Action::Move(dist) => waypoint += *dist,
            Action::Fwd(d) => pos += waypoint * *d,
            Action::Rotate(rot) => waypoint *= *rot,
        }
    }
    pos.0.abs() + pos.1.abs()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 776);
    assert_eq!(part1(&d), 1424);
    assert_eq!(part2(&d), 63447);
}

#[test]
fn test_data() {
    let data = // Example data
"F10
N3
F7
R90
F11";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 25);
    assert_eq!(part2(&d), 286);
}
