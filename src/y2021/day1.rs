const INPUT_FILE: &str = "data/2021/day1.txt";

type LineType = i32;
type LineContainer = Vec<LineType>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> LineContainer {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> LineType {
    s.as_ref().parse::<LineType>().unwrap()
}

fn part1(lines: &LineContainer) -> usize {
    let mut incr = 0;
    let mut li = lines.iter();
    let mut prev = *li.next().unwrap();
    for l in li {
        if *l > prev {
            incr += 1;
        }
        prev = *l;
    }
    incr
}

fn part2(lines: &LineContainer) -> usize {
    let mut li = lines.windows(3).map(|win| win.iter().sum());
    let mut prev: i32 = li.next().unwrap();
    let mut incr = 0;
    for w in li {
        if w > prev {
            incr += 1;
        }
        prev = w;
    }
    incr
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(part1(&d), 1301);
    assert_eq!(part2(&d), 1346);
}

#[test]
fn test_data() {
    let data = // Example data
"199
200
208
210
200
207
240
269
260
263";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 7);
    assert_eq!(part2(&d), 5);
}
