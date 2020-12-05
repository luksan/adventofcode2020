fn load_input() -> Vec<(Rule, Vec<char>)> {
    crate::load_input("data/day2/input.txt", parse_line)
}

fn parse_line<T: AsRef<str>>(line: T) -> Line {
    let (n1, n2, letter, pwd) =
        scan_fmt::scan_fmt!(line.as_ref(), "{d}-{d} {}: {}", usize, usize, char, String).unwrap();
    (Rule { n1, n2, letter }, pwd.chars().collect())
}

struct Rule {
    letter: char,
    n1: usize,
    n2: usize,
}

type Line = (Rule, Vec<char>);

pub struct Solver {
    input: Vec<Line>,
    p1_sol: u32,
    p2_sol: u32,
}

fn part1(passwords: &[Line]) -> usize {
    passwords
        .iter()
        .filter(|(rule, pwd)| {
            let cnt = pwd.iter().filter(|&&p| p == rule.letter).count();
            cnt >= rule.n1 && cnt <= rule.n2
        })
        .count()
}

fn part2(passwords: &[Line]) -> usize {
    let mut valid = 0;
    for (rule, pwd) in passwords {
        match (pwd.get(rule.n1 - 1), pwd.get(rule.n2 - 1)) {
            (Some(a), Some(b)) if a == b => {}
            (Some(&a), _) if a == rule.letter => valid += 1,
            (_, Some(&a)) if a == rule.letter => valid += 1,
            (_, _) => {}
        }
    }
    valid
}

#[test]
fn test_real_data() {
    let passwords = load_input();
    assert_eq!(part1(&passwords), 620);
    assert_eq!(part2(&passwords), 727);
}

#[test]
fn test_example_data() {
    let lines = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
        .iter()
        .map(parse_line)
        .collect::<Vec<_>>();
    assert_eq!(part1(&lines), 2);
    assert_eq!(part2(&lines), 1);
}
