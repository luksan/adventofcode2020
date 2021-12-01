use itertools::Itertools;

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
    lines.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2(lines: &LineContainer) -> usize {
    lines
        .windows(3)
        .map(|w| w.iter().sum())
        .tuple_windows::<(i32, i32)>()
        .filter(|(prev, curr)| curr > prev)
        .count()
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
