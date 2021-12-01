const INPUT_FILE: &str = "data/2020/day1.txt";

type Entry = u32;
type Report = Vec<Entry>;

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Report {
    let mut lines: Vec<_> = line_source
        .into_iter()
        .map(|s| s.as_ref().parse().unwrap())
        .collect();
    lines.sort_unstable();
    lines
}

fn part1(arr: &Report) -> Entry {
    for a in arr {
        let b = 2020 - a;
        if arr.binary_search(&b).is_ok() {
            return a * b;
        }
    }
    0
}

fn part2(arr: &Report) -> Entry {
    let len = arr.len();
    for ai in 0..len {
        let a = arr[ai];
        for bi in ai..len {
            let b = arr[bi];
            let c = 2020_u32.saturating_sub(a + b);
            if let Ok(ci) = arr.binary_search(&c) {
                if ai == ci || bi == ci {
                    continue;
                }
                return a * b * c;
            }
        }
    }
    0
}

#[test]
fn test_day1() {
    let arr = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(arr.len(), 200);
    assert_eq!(arr[0], 350);
    assert_eq!(part1(&arr), 866436);
    assert_eq!(part2(&arr), 276650720);
}

#[test]
fn test_example_data() {
    let arr = load_input(["1721", "979", "366", "299", "675", "1456"].iter());
    assert_eq!(part1(&arr), 514579);
    assert_eq!(part2(&arr), 241861950);
}
