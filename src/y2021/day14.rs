use itertools::Itertools;

use std::collections::HashMap;
use std::convert::TryInto;

pub type Input = (String, Vec<Rule>);

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    let mut lines = line_source.into_iter();
    let template = lines.next().unwrap().as_ref().to_owned();
    lines.next();
    (template, lines.map(parse).collect())
}

fn parse<S: AsRef<str>>(s: S) -> Rule {
    let (pair, ins) = s.as_ref().split(" -> ").collect_tuple().unwrap();
    Rule {
        pair: pair.as_bytes().try_into().unwrap(),
        insert: ins.as_bytes()[0],
    }
}

pub struct Rule {
    pair: [u8; 2],
    insert: u8,
}

const CNT_MAX: usize = 32;

#[derive(Clone)]
struct Cnt([usize; CNT_MAX]);

impl Cnt {
    pub fn new() -> Self {
        Self([0; CNT_MAX])
    }

    pub fn max(&self) -> (u8, usize) {
        self.0
            .iter()
            .enumerate()
            .max_by_key(|(_idx, &b)| b)
            .map(|(a, &b)| (a as u8 + b'A', b))
            .unwrap()
    }

    pub fn min(&self) -> (u8, usize) {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, &b)| b > 0)
            .min_by_key(|(_idx, &b)| b)
            .map(|(a, &b)| (a as u8 + b'A', b))
            .unwrap()
    }

    pub fn add(&mut self, other: &Self) {
        self.0
            .iter_mut()
            .zip(other.0.iter())
            .for_each(|(a, b)| *a += *b);
    }

    pub fn count(&mut self, item: u8) {
        self.0[(item - b'A') as usize] += 1;
    }
}

impl Extend<u8> for Cnt {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        for n in iter.into_iter() {
            self.count(n);
        }
    }
}

fn part1(input: &Input) -> usize {
    let (template, rules) = input;
    let mut poly = template.as_bytes().iter().copied().collect_vec();
    for _ in 0..10 {
        poly = poly
            .windows(2)
            .fold(Vec::from(&poly[..1]), |mut new, pair| {
                if let Some(n) = rules.iter().find(|r| r.pair == pair) {
                    new.push(n.insert);
                }
                new.push(pair[1]);
                new
            });
    }
    let mut cnt = Cnt::new();
    cnt.extend(poly.iter().copied());

    cnt.max().1 - cnt.min().1
}

fn extend(
    pair: [u8; 2],
    depth: u8,
    rules: &[Rule],
    cache: &mut HashMap<([u8; 2], u8), Cnt>,
    total_cnt: &mut Cnt,
) {
    if depth == 0 {
        return;
    }
    if let Some(c) = cache.get(&(pair, depth)) {
        total_cnt.add(c);
        return;
    }
    if let Some(r) = rules.iter().find(|r| r.pair == pair) {
        let mut sub_cnt = Cnt::new();
        extend([pair[0], r.insert], depth - 1, rules, cache, &mut sub_cnt);
        extend([r.insert, pair[1]], depth - 1, rules, cache, &mut sub_cnt);
        sub_cnt.count(r.insert);
        total_cnt.add(&sub_cnt);
        cache.insert((pair, depth), sub_cnt);
    } else {
        // done
    }
}

fn part2(input: &Input) -> usize {
    let (template, rules) = input;

    let mut cnt = Cnt::new();
    cnt.extend(template.as_bytes().iter().copied());

    let mut cache = HashMap::with_capacity(3250);
    for pair in template.as_bytes().windows(2) {
        extend([pair[0], pair[1]], 40, rules, &mut cache, &mut cnt);
    }

    cnt.max().1 - cnt.min().1
}

pub fn bench(input: &Input) {
    part2(input);
}

pub fn bench_input() -> Input {
    load_input(crate::load_strings(crate::data_file!()))
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 2345);
    assert_eq!(part2(&d), 2432786807053);
}

#[test]
fn test_data() {
    let data = // Example data
"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 1588);
    assert_eq!(part2(&d), 2188189693529);
}
