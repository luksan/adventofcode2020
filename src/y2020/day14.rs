use arrayvec::ArrayVec;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT_FILE: &str = "data/2020/day14.txt";

pub type Program = Vec<Instr>;

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Program {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Instr {
    s.as_ref().parse::<Instr>().unwrap()
}

#[derive(Debug, Copy, Clone)]
pub enum MaskBit {
    One,
    Zero,
    X,
}

type BitMask = ArrayVec<[MaskBit; 36]>;

#[derive(Debug)]
pub enum Instr {
    Mask(BitMask), // set, clear
    Store(u64, u64),
}

impl FromStr for Instr {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.find(']') {
            Some(rb) => Instr::Store(s[4..rb].parse()?, s[rb + 4..].parse()?),
            None => Instr::Mask(
                s[7..]
                    .chars()
                    .map(|c| match c {
                        '1' => MaskBit::One,
                        '0' => MaskBit::Zero,
                        'X' => MaskBit::X,
                        _ => unreachable!("Bad bitmask input"),
                    })
                    .collect(),
            ),
        })
    }
}

pub fn part1(prog: &Program) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask_set = 0;
    let mut mask_clear = 0;
    for line in prog {
        match line {
            Instr::Mask(mask) => {
                mask_set = 0;
                mask_clear = 0;
                for c in mask {
                    mask_set <<= 1;
                    mask_clear <<= 1;
                    match c {
                        MaskBit::One => mask_set |= 1,
                        MaskBit::Zero => mask_clear |= 1,
                        _ => {}
                    }
                }
                mask_clear = !mask_clear;
            }
            Instr::Store(addr, value) => {
                let v = *value;
                mem.insert(*addr, v & mask_clear | mask_set);
            }
        }
    }
    mem.values().sum()
}

pub fn part2(prog: &Program) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut addr_high = 0;
    let mut floating = Vec::<(u64, u64)>::new(); // (set, clear)

    for line in prog {
        assert!(mem.capacity() < 10_000_000); // Avoid OOM bugs
        match line {
            Instr::Mask(addr_mask) => {
                floating.clear();
                addr_high = 0;
                let mut x_shift = 0;
                for c in addr_mask.iter() {
                    addr_high <<= 1;
                    x_shift += 1;
                    match c {
                        MaskBit::One => addr_high |= 1,
                        MaskBit::X => {
                            if floating.is_empty() {
                                floating.push((1, 0));
                                floating.push((0, 1));
                            } else {
                                floating.reserve(floating.len());
                                for n in 0..floating.len() {
                                    let (s, c) = floating[n];
                                    floating[n] =
                                        (floating[n].0 << x_shift, (floating[n].1 << x_shift) | 1);
                                    floating.push(((s << x_shift) | 1, c << x_shift));
                                }
                            }
                            x_shift = 0;
                        }
                        MaskBit::Zero => {}
                    }
                }
                for (set, clear) in floating.iter_mut() {
                    *set <<= x_shift;
                    *clear <<= x_shift;
                    *clear = !*clear;
                }
            }
            Instr::Store(addr, value) => {
                let addr = *addr | addr_high;
                for (set, clear) in &floating {
                    let a = addr & clear | set;
                    mem.insert(a, *value);
                }
            }
        }
    }
    mem.values().sum()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 579);
    assert_eq!(part1(&d), 17765746710228);
    assert_eq!(part2(&d), 4401465949086);
}

#[test]
fn test_data() {
    let data = // Example data
"mask = 101X0X1
mem[51573] = 6865197";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 6865241);
    assert_eq!(part2(&d), 27460788);
}
