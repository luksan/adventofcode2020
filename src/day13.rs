use diophantine::Solution;
use itertools::Itertools;
use std::iter::FromIterator;

const INPUT_FILE: &str = "data/day13.txt";

pub type LineType = String;

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

pub fn part2(input: &InputData) -> usize {
    0
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.timetable.len(), 68);
    assert_eq!(part1(&d), 1915);
    assert_eq!(part2(&d), 1);
}

#[test]
fn test_data() {
    let data = // Example data
"939
7,13,x,x,59,x,31,19";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 295);
    assert_eq!(part2(&d), 1);
}
