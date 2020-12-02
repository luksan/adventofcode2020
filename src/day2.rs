use adventofcode2020::DayOfAdvent;

fn load_input() -> Vec<(Rule, Vec<char>)> {
    adventofcode2020::load_input("data/day2/input.txt", parse_line)
}

struct Rule {
    letter: char,
    n1: usize,
    n2: usize,
}

type Line = (Rule, Vec<char>);

pub fn solve() -> Box<dyn DayOfAdvent> {
    let mut s = Solver::new(load_input());
    s.part1();
    s.part2();
    Box::new(s)
}

pub struct Solver {
    input: Vec<Line>,
    p1_sol: u32,
    p2_sol: u32,
}

fn parse_line<T: AsRef<str>>(line: T) -> Line {
    let (n1, n2, letter, pwd) =
        scan_fmt::scan_fmt!(line.as_ref(), "{d}-{d} {}: {}", usize, usize, char, String).unwrap();
    (Rule { n1, n2, letter }, pwd.chars().collect())
}

impl Solver {
    fn new(input: Vec<Line>) -> Self {
        Self {
            input,
            p1_sol: 0,
            p2_sol: 0,
        }
    }

    fn part1(&mut self) {
        self.p1_sol = self
            .input
            .iter()
            .filter(|(rule, pwd)| {
                let cnt = pwd.iter().filter(|&&p| p == rule.letter).count();
                cnt >= rule.n1 && cnt <= rule.n2
            })
            .count() as u32;
    }

    fn part2(&mut self) {
        let mut valid = 0;
        for (rule, pwd) in &self.input {
            match (pwd.get(rule.n1 - 1), pwd.get(rule.n2 - 1)) {
                (Some(a), Some(b)) if a == b => {}
                (Some(&a), _) if a == rule.letter => valid += 1,
                (_, Some(&a)) if a == rule.letter => valid += 1,
                (_, _) => {}
            }
        }
        self.p2_sol = valid;
    }
}

impl DayOfAdvent for Solver {
    fn day(&self) -> u32 {
        2
    }

    fn result_strings(&self) -> Vec<String> {
        let mut ret = Vec::new();
        ret.push(format!("Part 1: {} valid passwords.", self.p1_sol));
        ret.push(format!("Part 2: {} valid passwords.", self.p2_sol));
        ret
    }
}

#[test]
fn test_real_data() {
    let mut solver = Solver::new(load_input());
    solver.part1();
    assert_eq!(solver.p1_sol, 620);
    solver.part2();
    assert_eq!(solver.p2_sol, 727);
}

#[test]
fn test_example_data() {
    let lines = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
        .iter()
        .map(parse_line)
        .collect::<Vec<_>>();

    let mut solver = Solver::new(lines);
    solver.part1();
    assert_eq!(solver.p1_sol, 2);

    solver.part2();
    assert_eq!(solver.p2_sol, 1);
}
