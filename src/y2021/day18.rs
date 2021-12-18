use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, u8 as nom_u8};
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;

type Input = Vec<Pair>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Pair {
    match pair(s.as_ref()) {
        Ok((_buf, pair)) => pair,
        _ => panic!("Parse error"),
    }
}

type NomRes<'b, O> = IResult<&'b str, O>;

fn pair(buf: &str) -> NomRes<Pair> {
    let (buf, (left, right)) = terminated(
        preceded(char('['), separated_pair(inner, char(','), inner)),
        char(']'),
    )(buf)?;
    Ok((buf, Pair { left, right }))
}

fn inner(buf: &str) -> NomRes<Elem> {
    alt((map(nom_u8, Elem::Regular), map(pair, |p| p.into())))(buf)
}

#[derive(Clone)]
struct Pair {
    left: Elem,
    right: Elem,
}

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?}]", self.left, self.right)
    }
}

#[derive(Clone)]
enum Elem {
    Regular(u8),
    Pair(Box<Pair>),
}

impl Debug for Elem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Elem::Regular(d) => write!(f, "{}", d),
            Elem::Pair(p) => write!(f, "{}", p),
        }
    }
}

impl From<Pair> for Elem {
    fn from(p: Pair) -> Self {
        Self::Pair(Box::new(p))
    }
}

impl From<u8> for Elem {
    fn from(d: u8) -> Self {
        Self::Regular(d)
    }
}

impl Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Pair {
            left: self.into(),
            right: rhs.into(),
        };
        sum.reduce();
        sum
    }
}

impl Elem {
    pub fn add(&mut self, n: u8, mut f: impl FnMut(&mut Pair) -> &mut Elem) {
        if n == 0 {
            return;
        }
        match self {
            Elem::Regular(d) => *d += n,
            Elem::Pair(p) => f(p).add(n, f),
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(u8, u8)> {
        let pair = match self {
            Self::Pair(p) => p,
            _ => return None,
        };

        if depth > 4 {
            return match (&pair.left, &pair.right) {
                (Elem::Regular(l), Elem::Regular(r)) => {
                    let (l, r) = (*l, *r);
                    *self = Self::Regular(0);
                    Some((l, r))
                }
                _ => unreachable!("Elements to split should always be regular"),
            };
        }
        if let Some((l, r)) = pair.left.explode(depth + 1) {
            pair.right.add(r, |p| &mut p.left);
            return Some((l, 0));
        }
        if let Some((l, r)) = pair.right.explode(depth + 1) {
            pair.left.add(l, |p| &mut p.right);
            return Some((0, r));
        }
        None
    }
}

impl Pair {
    fn add_left(&mut self, n: u8) {
        match &mut self.left {
            Elem::Regular(d) => *d += n,
            Elem::Pair(p) => p.add_left(n),
        }
    }

    fn explode(&mut self, depth: usize) -> bool {
        if let Some((_, r)) = self.left.explode(depth + 1) {
            self.right.add(r, |p| &mut p.left);
            return true;
        }
        if let Some((l, _)) = self.right.explode(depth + 1) {
            self.left.add(l, |p| &mut p.right);
            return true;
        }
        false
    }

    fn splits(&mut self) -> bool {
        for e in [&mut self.left, &mut self.right] {
            match e {
                Elem::Regular(d) => {
                    if *d > 9 {
                        *e = Pair {
                            left: (*d / 2).into(),
                            right: ((*d + 1) / 2).into(),
                        }
                        .into();
                        return true;
                    }
                }
                Elem::Pair(p) => {
                    if p.splits() {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn reduce(&mut self) {
        while self.explode(1) || self.splits() {
            //  println!("{}", self)
        }
    }

    fn magnitude(&self) -> usize {
        let l = match &self.left {
            Elem::Regular(d) => *d as usize,
            Elem::Pair(p) => p.magnitude(),
        };
        let r = match &self.right {
            Elem::Regular(d) => *d as usize,
            Elem::Pair(p) => p.magnitude(),
        };
        3 * l + 2 * r
    }
}

#[test]
fn test_explode() {
    let mut x = parse("[[[[[9,8],1],2],3],4]");
    x.reduce();
    assert_eq!(x.to_string(), "[[[[0,9],2],3],4]");
    let mut x = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    x.reduce();
    assert_eq!(x.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    let mut x = parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    x.reduce();
    assert_eq!(x.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn test_add() {
    let a = parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let b = parse("[1,1]");
    assert_eq!((a + b).to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
}

fn part1(input: &Input) -> usize {
    let mut iter = input.iter();
    let mut sum = iter.next().unwrap().clone();
    for n in iter {
        sum = sum + n.clone();
    }
    sum.magnitude()
}

fn part2(input: &Input) -> usize {
    input
        .iter()
        .combinations(2)
        .map(|n| {
            (n[0].clone() + n[1].clone())
                .magnitude()
                .max((n[1].clone() + n[0].clone()).magnitude())
        })
        .max()
        .unwrap()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 3486);
    assert_eq!(part2(&d), 4747);
}

#[test]
fn test_data() {
    let data = // Example data
"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 4140);
    assert_eq!(part2(&d), 3993);
}
