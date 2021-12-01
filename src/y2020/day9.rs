use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::VecDeque;
use std::ops::Add;
use std::slice::Iter;

const INPUT_FILE: &str = "data/day9.txt";

type Num = u64;
type CypherText = Vec<Num>;

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> CypherText {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Num {
    let line = s.as_ref();
    line.parse::<Num>().unwrap()
}

struct WindowPairsSum<'a, T: Add<Output = T> + Copy> {
    push: Iter<'a, T>,
    pairs: VecDeque<(T, Vec<T>)>,
}

impl<'a, T: Add<Output = T> + Copy> WindowPairsSum<'a, T> {
    pub fn new(slice: &'a [T], win_size: usize) -> Self {
        assert!(slice.len() >= win_size);

        let mut pairs = VecDeque::with_capacity(win_size);
        pairs.push_back((slice[0], Vec::with_capacity(win_size - 1))); // Dummy entry
        for n in 0..win_size - 1 {
            let v = slice[n];
            let mut vec = Vec::with_capacity(win_size - 1);
            for w in &slice[n + 1..win_size - 1] {
                vec.push(v + *w);
            }
            pairs.push_back((v, vec));
        }

        Self {
            push: slice[win_size - 1..].iter(),
            pairs,
        }
    }

    /// Returns a tuple (next list entry, iterator of pair-sums of previous window)
    /// This is a streaming iterator, so we can't implement the Iterator protocol
    fn next(&mut self) -> Option<(T, Box<dyn Iterator<Item = T> + '_>)> {
        if self.push.as_slice().len() < 2 {
            return None;
        }

        let (_, mut vec) = self.pairs.pop_front().unwrap();
        vec.clear();

        let new = *self.push.next()?;
        for (v, p) in &mut self.pairs {
            p.push(*v + new)
        }

        self.pairs.push_back((new, vec));

        Some((
            self.push.as_slice()[0],
            Box::new(self.pairs.iter().map(|(_, p)| p.iter().copied()).flatten()),
        ))
    }
}

pub fn part1(nums: &CypherText, preamble_len: usize) -> u64 {
    let mut sum_iter = WindowPairsSum::new(nums, preamble_len);

    while let Some((v, mut sums)) = sum_iter.next() {
        if sums.find(|s| *s == v).is_none() {
            return v;
        }
    }
    0
}

pub fn part2(text: &CypherText, step1: u64) -> u64 {
    let (mut a, mut b, mut sum) = (0, 0, 0);
    loop {
        while sum < step1 {
            sum += text[b];
            b += 1;
        }
        while sum > step1 {
            sum -= text[a];
            a += 1;
        }
        if sum == step1 && b > a + 1 {
            break;
        }
    }
    if let MinMax(x, y) = text[a..b].iter().minmax() {
        x + y
    } else {
        0
    }
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 1000);
    let p1 = part1(&d, 25);
    assert_eq!(p1, 70639851);
    assert_eq!(part2(&d, p1), 8249240);
}

#[test]
fn test_data() {
    let data = // Example data
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let d: CypherText = load_input(data.lines());
    let p1 = part1(&d, 5);
    assert_eq!(p1, 127);
    assert_eq!(part2(&d, p1), 62);
}
