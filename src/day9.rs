use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

type Num = u64;
type CypherText = Vec<Num>;

pub fn load_input() -> CypherText {
    crate::load_input("data/day9.txt", parse)
}

fn parse<S: AsRef<str>>(s: S) -> Num {
    let line = s.as_ref();
    line.parse::<Num>().unwrap()
}

pub fn part1(nums: &CypherText, preamble_len: usize) -> u64 {
    let sums: Vec<Vec<Num>> = nums
        .as_slice()
        .windows(preamble_len)
        .map(|w| {
            w.iter()
                .combinations(2)
                .map(|c| c.into_iter().sum())
                .collect::<Vec<_>>()
        })
        .collect();

    assert_eq!(nums.len(), sums.len() + preamble_len - 1);
    assert_eq!(sums[0].len(), sums.last().unwrap().len());
    assert_eq!(sums[0].len(), (0..preamble_len).sum());

    nums.iter()
        .skip(preamble_len)
        .zip(sums.into_iter())
        .find_map(|(num, sums)| {
            sums.into_iter()
                .find(|s| s == num)
                .map_or(Some(*num), |_| None)
        })
        .unwrap()
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
    let d = load_input();
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
    let d: CypherText = data.lines().map(parse).collect();
    let p1 = part1(&d, 5);
    assert_eq!(p1, 127);
    assert_eq!(part2(&d, p1), 62);
}
