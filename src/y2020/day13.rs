use itertools::Itertools;
use std::iter::FromIterator;

const INPUT_FILE: &str = "data/day13.txt";

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> InputData {
    line_source.into_iter().collect()
}

pub struct InputData {
    departure: u32,
    timetable: Vec<Entry>,
}

#[derive(Debug, Copy, Clone)]
pub enum Entry {
    Bus(u32),
    X,
}

impl<S: AsRef<str>> FromIterator<S> for InputData {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut s = iter.into_iter();
        let departure = s.next().unwrap().as_ref().parse().unwrap();
        let timetable = s
            .next()
            .unwrap()
            .as_ref()
            .split(',')
            .map(|s| match s.parse() {
                Ok(bus) => Entry::Bus(bus),
                _ => Entry::X,
            })
            .collect();
        Self {
            departure,
            timetable,
        }
    }
}

pub fn part1(input: &InputData) -> u32 {
    let arrival = input.departure;

    let mut shortest_wait = 999999;
    let mut first_bus = 0;
    for e in &input.timetable {
        if let Entry::Bus(bus) = *e {
            let wait = (arrival + bus - 1) / bus * bus - arrival;
            if wait < shortest_wait {
                shortest_wait = wait;
                first_bus = bus;
            }
        }
    }

    shortest_wait * first_bus
}

#[allow(clippy::many_single_char_names)]
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub fn part2(input: &InputData) -> i64 {
    let flt = input
        .timetable
        .iter()
        .enumerate()
        .filter_map(|(idx, d)| match d {
            Entry::Bus(bus) => Some((*bus as i64 - idx as i64, *bus as i64)),
            Entry::X => None,
        })
        .collect_vec();
    let busses = flt.iter().map(|(_, b)| *b).collect_vec();
    let residues = flt.iter().map(|(r, _)| *r).collect_vec();
    chinese_remainder(&residues, &busses).unwrap()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.timetable.len(), 68);
    assert_eq!(part1(&d), 1915);
    assert_eq!(part2(&d), 294354277694107);
}

#[test]
fn test_data() {
    let data = // Example data
"939
7,13,x,x,59,x,31,19";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 295);
    assert_eq!(part2(&d), 1068781);
}
