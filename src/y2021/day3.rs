const INPUT_FILE: &str = "data/2021/day3.txt";

type Numbers = Vec<BinNum>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Numbers {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> BinNum {
    s.as_ref()
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => panic!("bad bool"),
        })
        .collect()
}

type BinNum = Box<[bool]>;

#[derive(Clone, Debug, Default)]
struct BoolCnt(u16, u16);

impl BoolCnt {
    fn count(&mut self, b: bool) {
        if b {
            self.0 += 1;
        } else {
            self.1 += 1;
        }
    }

    fn most_common(&self) -> bool {
        self.0 >= self.1
    }
}

fn count_column(lines: &Numbers, col: usize) -> BoolCnt {
    let mut cnt = BoolCnt::default();
    for l in lines {
        cnt.count(l[col]);
    }
    cnt
}

fn binnum2usize(num: &BinNum) -> usize {
    let mut r = 0;
    for &bit in num.iter() {
        r = r << 1 | bit as usize;
    }
    r
}

fn part1(lines: &Numbers) -> usize {
    let mut counters = vec![BoolCnt::default(); lines[0].len()];

    for line in lines {
        for (&bit, cnt) in line.iter().zip(counters.iter_mut()) {
            cnt.count(bit)
        }
    }

    let mut gamma: usize = 0;
    for bit in &counters {
        gamma = gamma << 1 | bit.most_common() as usize;
    }

    let epsilon = !gamma & ((1 << counters.len()) - 1);
    gamma * epsilon
}

fn part2(lines: &Numbers) -> usize {
    // Oxygen
    let mut o2 = lines.clone();
    for col in 0..lines[0].len() {
        let cnt = count_column(&o2, col);
        o2.retain(|num| num[col] == cnt.most_common());
        if o2.len() < 2 {
            break;
        }
    }

    //co2
    let mut co2 = lines.clone();
    for col in 0..lines[0].len() {
        let cnt = count_column(&co2, col);
        co2.retain(|num| num[col] != cnt.most_common());
        if co2.len() < 2 {
            break;
        }
    }

    binnum2usize(&o2[0]) * binnum2usize(&co2[0])
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(part1(&d), 1540244);
    assert_eq!(part2(&d), 4203981);
}

#[test]
fn test_data() {
    let data = // Example data
"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 198);
    assert_eq!(part2(&d), 230);
}
