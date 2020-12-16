use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::iter::FromIterator;
use std::ops::Deref;
use std::str::FromStr;

const INPUT_FILE: &str = "data/day16.txt";

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    let i: IterWrap<_> = line_source.into_iter().into();
    i.try_into().unwrap()
}

pub struct Range(std::ops::RangeInclusive<u32>);

impl Deref for Range {
    type Target = std::ops::RangeInclusive<u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split('-')
            .map(|s| s.parse())
            .collect_tuple::<(_, _)>()
            .with_context(|| format!("Error parsing range '{}'", s))?;
        Ok(Self(a?..=b?))
    }
}

pub struct Rule([Range; 2]);

impl Rule {
    pub fn matches(&self, v: &u32) -> bool {
        self[0].contains(v) || self[1].contains(v)
    }
}

impl Deref for Rule {
    type Target = [Range; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Input {
    rules: Vec<Rule>,
    my_ticket: Vec<u32>,
    tickets: Vec<Vec<u32>>,
}

struct IterWrap<T>(T); // E0119 https://github.com/rust-lang/rust/issues/50133
impl<T> From<T> for IterWrap<T> {
    fn from(x: T) -> Self {
        Self(x)
    }
}

impl<I: Iterator<Item = S>, S: AsRef<str>> TryFrom<IterWrap<I>> for Input {
    type Error = Error;

    fn try_from(iter: IterWrap<I>) -> Result<Self, Self::Error> {
        let mut lines = iter.0;

        let mut rules = Vec::new();
        for line in &mut lines {
            let s = line.as_ref();
            if s.is_empty() {
                break;
            }
            let (_name, ranges) = s.split(": ").collect_tuple().context("Rule name failure")?;
            let mut ri = ranges.split(" or ");
            rules.push(Rule([
                ri.next().context("rule 1 split error")?.parse()?,
                ri.next().context("rule 2 split error")?.parse()?,
            ]))
        }

        lines.next(); // skip "your ticket:"
        let my_ticket;
        if let Some(line) = lines.next() {
            let s = line.as_ref();
            my_ticket = s.split(',').map(|n| n.parse()).collect::<Result<_, _>>()?;
        } else {
            bail!("My ticket not found!");
        }
        // skip 2 lines
        lines.next();
        lines.next();

        let tickets = lines
            .map(|line| {
                line.as_ref()
                    .split(',')
                    .map(|n| n.parse::<u32>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            rules,
            my_ticket,
            tickets,
        })
    }
}

pub fn part1(input: &Input) -> u32 {
    let mut tot = 0;
    for t in &input.tickets {
        'value: for v in t {
            for r in &input.rules {
                if r.matches(v) {
                    continue 'value;
                }
            }
            tot += *v;
        }
    }
    tot
}

pub fn part2(input: &Input) -> u64 {
    let valid_tickets = input
        .tickets
        .iter()
        // all values on the ticket must match at least one rule for the ticket to be valid
        .filter(|&t| t.iter().all(|v| input.rules.iter().any(|r| r.matches(v))))
        .collect_vec();

    let field_cnt = input.my_ticket.len() as u32;
    // order index is the index for the ticket field
    // each hashset contains all matching rules for the ticket field
    let mut order: Vec<HashSet<u32, RandomState>> =
        vec![HashSet::from_iter(0..field_cnt); field_cnt as usize];

    for ticket in valid_tickets {
        for (field, matching) in ticket.iter().zip(order.iter_mut()) {
            for (idx, r) in input.rules.iter().enumerate() {
                if !r.matches(field) {
                    matching.remove(&(idx as u32));
                }
            }
        }
    }
    let mut n_iter = (0..order.len()).cycle();
    let mut done = HashSet::new();
    while done.len() < field_cnt as usize {
        let mut uniq = 0;
        for n in &mut n_iter {
            if order[n].len() == 1 && !done.contains(&n) {
                uniq = *order[n].iter().next().unwrap();
                done.insert(n);
                break;
            }
        }
        for o in &mut order {
            if o.len() > 1 {
                o.remove(&uniq);
            }
        }
    }
    let mut departure: u64 = 1;
    for (rule, field) in order.iter().zip(input.my_ticket.iter()) {
        let rule_idx = rule.iter().exactly_one().unwrap();
        if *rule_idx < 6 {
            departure *= *field as u64;
        }
    }
    departure
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(part1(&d), 26026);
    assert_eq!(part2(&d), 1305243193339);
}

#[test]
fn test_data() {
    let data = // Example data
"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
    let d = load_input(data.lines());
    assert_eq!(d.tickets.len(), 4);
    assert_eq!(part1(&d), 71);
}