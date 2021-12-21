use std::collections::HashMap;
use std::iter;
use std::ops::Range;

type Input = [u32; 2];

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    let p: Vec<_> = line_source.into_iter().map(parse).collect();
    [p[0], p[1]]
}

fn parse<S: AsRef<str>>(s: S) -> u32 {
    s.as_ref()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

struct PracticeDie(iter::Enumerate<iter::Cycle<Range<u32>>>);

impl PracticeDie {
    fn new() -> Self {
        Self((1..101).cycle().enumerate())
    }
    fn roll(&mut self) -> u32 {
        self.0.next().unwrap().1
    }
    fn roll_cnt(mut self) -> u32 {
        self.0.next().unwrap().0 as u32
    }
}

fn part1(input: &Input) -> u32 {
    let mut pos = *input;
    let mut score = [0; 2];
    let mut die = PracticeDie::new();
    'game: loop {
        for (p, s) in pos.iter_mut().zip(score.iter_mut()) {
            *p += (die.roll() + die.roll() + die.roll()) % 10;
            if *p > 10 {
                *p -= 10;
            }
            *s += *p;
            if *s >= 1000 {
                break 'game;
            }
        }
    }
    score.iter().min().unwrap() * die.roll_cnt()
}

fn dirac_roll(
    player: u8,
    roll_cnt: u8,
    pos: [u8; 2],
    score: [u8; 2],
    cache: &mut HashMap<[u8; 6], [usize; 2]>,
) -> [usize; 2] {
    let cache_key = [player, roll_cnt, pos[0], pos[1], score[0], score[1]];
    if let Some(w) = cache.get(&cache_key) {
        return *w;
    }
    let mut win_cnt = [0; 2];
    let p = player as usize;
    for die in 1..4 {
        let mut pos = pos;
        pos[p] += die;
        if pos[p] > 10 {
            pos[p] -= 10;
        }
        let c = if roll_cnt == 2 {
            let mut score = score;
            score[p] += pos[p];
            if score[p] >= 21 {
                win_cnt[p] += 1;
                continue;
            } else {
                dirac_roll((player + 1) % 2, 0, pos, score, cache)
            }
        } else {
            dirac_roll(player, roll_cnt + 1, pos, score, cache)
        };
        win_cnt[0] += c[0];
        win_cnt[1] += c[1];
    }
    cache.insert(cache_key, win_cnt);
    win_cnt
}

fn part2(input: &Input) -> usize {
    let pos = [input[0] as u8, input[1] as u8];
    *dirac_roll(0, 0, pos, [0, 0], &mut HashMap::with_capacity(1000))
        .iter()
        .max()
        .unwrap()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 853776);
    assert_eq!(part2(&d), 301304993766094);
}

#[test]
fn test_data() {
    let data = // Example data
"Player 1 starting position: 4
Player 2 starting position: 8";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 739785);
    assert_eq!(part2(&d), 444356092776315);
}
