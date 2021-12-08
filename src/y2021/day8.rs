use arrayvec::ArrayVec;
use itertools::Itertools;
use std::ops::Sub;
use std::str::FromStr;

type Displays = Vec<Display>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Displays {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Display {
    let (digits, reading) = s.as_ref().split(" | ").collect_tuple().unwrap();
    let n: ArrayVec<Digit, 10> = digits
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let m: ArrayVec<Digit, 4> = reading
        .split_whitespace()
        .map(|r| r.parse().unwrap())
        .collect();
    Display {
        digits: n.into_inner().unwrap_or_else(|_| panic!()),
        reading: m.into_inner().unwrap_or_else(|_| panic!()),
    }
}

struct Display {
    digits: [Digit; 10],
    reading: [Digit; 4],
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
struct Digit(u8);

impl Digit {
    pub fn len(&self) -> u32 {
        self.0.count_ones()
    }
}

impl FromStr for Digit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segs: u8 = 0;
        for c in s.bytes() {
            segs |= 1 << (c - b'a');
        }
        Ok(Self(segs))
    }
}

impl Sub for &Digit {
    type Output = Digit;

    fn sub(self, rhs: Self) -> Self::Output {
        Digit(!rhs.0 & self.0)
    }
}

fn part1(displays: &Displays) -> usize {
    // count occurrences of 1,4,7,8
    displays
        .iter()
        .flat_map(|disp| disp.reading.iter())
        .filter(|&digit| [2, 4, 3, 7].contains(&digit.len()))
        .count()
}

fn decode_display(disp: &Display) -> usize {
    let digits = &disp.digits;
    let find_len = |l| digits.iter().find(|d| d.len() == l).unwrap();
    let one = find_len(2);
    let seven = find_len(3);
    let eight = find_len(7);
    let four = find_len(4);

    let diff_len = |len, diff, dl| {
        digits
            .iter()
            .find(|&d| d.len() == len && (d - diff).len() == dl)
            .unwrap()
    };
    let three = diff_len(5, seven, 2);
    let five = digits
        .iter()
        .filter(|&d| d.len() == 5 && d != three) // 2 or 5 remaining
        .find(|&d| (d - four).len() == 2)
        .unwrap();

    let two = digits
        .iter()
        .find(|&d| d.len() == 5 && d != five && d != three)
        .unwrap();

    let six = diff_len(6, seven, 4);
    let nine = diff_len(6, four, 2);

    let zero = digits
        .iter()
        .find(|&d| d.len() == 6 && d != nine && d != six)
        .unwrap();

    let map = [zero, one, two, three, four, five, six, seven, eight, nine];

    let mut reading = 0;
    for d in &disp.reading {
        let n = map.iter().position(|&x| x == d).unwrap();
        reading = reading * 10 + n;
    }
    reading
}

fn part2(displays: &Displays) -> usize {
    displays.iter().map(|d| decode_display(d)).sum()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 303);
    assert_eq!(part2(&d), 961734);
}

#[test]
fn test_data() {
    let data = // Example data
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 26);
    assert_eq!(part2(&d), 61229);
}
