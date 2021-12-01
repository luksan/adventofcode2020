use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write;

const INPUT_FILE: &str = "data/2020/day19.txt";

pub type LineType = String;
pub type RuleId = usize;

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    let mut lines = line_source.into_iter();
    Input {
        rules: (&mut lines)
            .take_while(|l| !l.as_ref().is_empty())
            .map(parse_rule)
            .collect(),
        messages: lines.map(|s| s.as_ref().to_owned()).collect(),
    }
}

fn parse_rule<S: AsRef<str>>(s: S) -> (RuleId, Vec<Token>) {
    let (id_str, s) = s.as_ref().split(':').collect_tuple().unwrap();
    let id: RuleId = id_str.parse().unwrap();
    (
        id,
        s.split(' ')
            .filter(|s| !s.is_empty())
            .map(|t| match t {
                "|" => Token::Or,
                r#""a""# => Token::A,
                r#""b""# => Token::B,
                d => Token::RuleId(d.parse().unwrap()),
            })
            .collect_vec(),
    )
}

pub type RuleSet = HashMap<RuleId, Vec<Token>>;

pub struct Input {
    rules: RuleSet,
    messages: Vec<String>,
}

pub enum Token {
    A,
    B,
    Or,
    RuleId(RuleId),
}

fn expand_rule(id: RuleId, ret: &mut String, rules: &RuleSet) {
    ret.push_str("(?:");
    for tok in rules.get(&id).unwrap() {
        match tok {
            Token::A => ret.push('a'),
            Token::B => ret.push('b'),
            Token::Or => ret.push('|'),
            Token::RuleId(id) => expand_rule(*id, ret, rules),
        }
    }
    ret.push(')');
}

fn expand_rule2(id: RuleId, ret: &mut String, rules: &RuleSet) {
    ret.push_str("(?:");
    if id == 8 {
        expand_rule(42, ret, rules);
        ret.push_str("+)");
        return;
    }
    if id == 11 {
        let mut r31 = String::new();
        let mut r42 = String::new();
        expand_rule(31, &mut r31, rules);
        expand_rule(42, &mut r42, rules);
        for n in 1..7 {
            write!(ret, "(?:{}{{{}}}{}{{{}}})|", &r42, n, &r31, n).unwrap();
        }
        ret.pop();
        ret.push(')');
        return;
    }
    for tok in rules.get(&id).unwrap() {
        match tok {
            Token::A => ret.push('a'),
            Token::B => ret.push('b'),
            Token::Or => ret.push('|'),
            Token::RuleId(id) => expand_rule2(*id, ret, rules),
        }
    }
    ret.push(')');
}

pub fn part1(input: &Input) -> usize {
    let mut r0 = String::from("^(?:");
    expand_rule(0, &mut r0, &input.rules);
    r0.push_str(")$");
    let re = Regex::new(&r0).unwrap();
    input.messages.iter().filter(|m| re.is_match(&m)).count()
}

pub fn part2(input: &Input) -> usize {
    let mut r0 = String::from("^(?:");
    expand_rule2(0, &mut r0, &input.rules);
    r0.push_str(")$");
    let re = Regex::new(&r0).unwrap();
    input.messages.iter().filter(|m| re.is_match(&m)).count()
}

#[test]
fn debug() {
    let data = // Example data
r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

aab
a
aba
"#;

    let d = load_input(data.lines());
    assert_eq!(part1(&d), 2);
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.messages.len(), 482);
    assert_eq!(part1(&d), 210);
    assert_eq!(part2(&d), 422);
}

#[test]
fn test_data1() {
    let data = // Example data
r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;

    let d = load_input(data.lines());
    assert_eq!(d.messages.len(), 5);
    assert_eq!(part1(&d), 2);
}

#[test]
fn test_data2() {
    let data = // Example data
r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#;

    let d = load_input(data.lines());
    assert_eq!(part2(&d), 12);
}
