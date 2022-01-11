#![allow(unused)]
use scan_fmt::scan_fmt;

use std::ops::{Add, Range};

type Input = Vec<Cube>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Cube {
    let (v, x1, x2, y1, y2, z1, z2) = scan_fmt!(
        s.as_ref(),
        "{} x={d}..{d},y={d}..{d},z={d}..{d}",
        String,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32
    )
    .unwrap();
    let c1 = Coord::new(x1, y1, z1);
    let c2 = Coord::new(x2, y2, z2);
    Cube {
        c1,
        c2,
        on: v == "on",
    }
}

trait Xyz: Copy + Add<i32, Output = Self> + Ord + Eq {
    const IDX: usize;
    fn p(self) -> i32;
    fn new(p: i32) -> Self;
}

macro_rules! xyz_def {
    ($xyz:ident, $idx:literal) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        struct $xyz(i32);
        impl Xyz for $xyz {
            const IDX: usize = $idx;
            fn p(self) -> i32 {
                self.0
            }
            fn new(p: i32) -> $xyz {
                $xyz(p)
            }
        }
        impl Add<i32> for $xyz {
            type Output = $xyz;
            fn add(self, other: i32) -> Self::Output {
                $xyz(self.0 + other)
            }
        }
    };
}
xyz_def!(X, 0);
xyz_def!(Y, 1);
xyz_def!(Z, 2);

#[derive(Copy, Clone, Debug)]
struct Coord([i32; 3]);

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self([x, y, z])
    }

    fn set<T: Xyz>(&self, p: T) -> Self {
        let mut c = self.0;
        c[T::IDX] = p.p();
        Self(c)
    }
    fn get<T: Xyz>(&self) -> T {
        T::new(self.0[T::IDX])
    }
}

#[derive(Copy, Clone, Debug)]
struct Cube {
    c1: Coord,
    c2: Coord,
    on: bool,
}

impl Cube {
    fn split<T: Xyz>(&self, left: T) -> [Cube; 2] {
        if self.c1.get::<T>() < left && self.c2.get::<T>() > left {
            let c2 = self.c2.set(left);
            let c1 = self.c1.set(left + 1);
            [Cube { c2, ..*self }, Cube { c1, ..*self }]
        } else {
            panic!("Invalid split")
        }
    }
    fn intersects(&self, other: &Self) -> Option<Coord> {
        todo!()
    }
}

fn part1(input: &Input) -> usize {
    let mut candidates = input.clone();
    let mut core: Vec<Cube> = Vec::new();
    'cand: while let Some(cand) = candidates.pop() {
        if [&cand.c1, &cand.c2]
            .iter()
            .flat_map(|c| c.0.iter())
            .any(|p| p.abs() > 50)
        {
            continue 'cand;
        }
        for c in core.iter().map(|c| c.c1) {}
    }
    0
}

fn part2(_input: &Input) -> usize {
    0
}

//#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 1);
    // assert_eq!(part2(&d), 1);
}

//#[test]
fn test_data() {
    let data = // Example data
"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 1);
    // assert_eq!(part2(&d), 1);
}
