use itertools::Itertools;

const INPUT_FILE: &str = "data/2021/day2.txt";

type LineContainer = Vec<Cmd>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> LineContainer {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Cmd {
    let c = s.as_ref();
    let (cmd, d) = c.splitn(2, ' ').collect_tuple().unwrap();
    let d: i32 = d.parse().unwrap();
    match cmd {
        "forward" => Cmd::Fwd(d),
        "up" => Cmd::Up(d),
        "down" => Cmd::Down(d),
        _ => panic!("bad cmd"),
    }
}

enum Cmd {
    Fwd(i32),
    Up(i32),
    Down(i32),
}

fn part1(lines: &LineContainer) -> usize {
    let mut fwd = 0;
    let mut depth = 0;
    for c in lines {
        match c {
            Cmd::Fwd(f) => fwd += f,
            Cmd::Up(d) => depth -= d,
            Cmd::Down(d) => depth += d,
        };
    }
    (fwd * depth) as usize
}

fn part2(lines: &LineContainer) -> usize {
    let mut aim = 0;
    let mut fwd = 0;
    let mut depth = 0;
    for c in lines {
        match c {
            Cmd::Fwd(f) => {
                fwd += f;
                depth += aim * f;
            }
            Cmd::Up(d) => aim -= d,
            Cmd::Down(d) => aim += d,
        };
    }
    (fwd * depth) as usize
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(part1(&d), 2187380);
    assert_eq!(part2(&d), 2086357770);
}

#[test]
fn test_data() {
    let data = // Example data
"forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 150);
    assert_eq!(part2(&d), 900);
}
