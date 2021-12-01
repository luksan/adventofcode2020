use std::slice::Iter;

const INPUT_FILE: &str = "data/2020/day18.txt";

pub type Expression = Vec<Token>;
pub type MathProblems = Vec<Expression>;

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> MathProblems {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Expression {
    s.as_ref()
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| {
            use Token::*;
            match c {
                '(' => LPar,
                ')' => RPar,
                '+' => Add,
                '*' => Mul,
                d if d.is_ascii_digit() => Digit(d.to_digit(10).unwrap() as u64),
                _ => WS,
            }
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Digit(u64),
    Add,
    Mul,
    LPar,
    RPar,
    WS,
}

fn eval_rhs1(iter: &mut Iter<Token>) -> u64 {
    while let Some(t) = iter.next() {
        match t {
            Token::Digit(d) => return *d,
            Token::LPar => return eval1(iter),
            Token::WS => continue,
            _ => panic!("Unexpected token"),
        }
    }
    0
}

fn eval1(expr: &mut Iter<Token>) -> u64 {
    let mut lhs = eval_rhs1(expr);
    while let Some(t) = expr.next() {
        match t {
            Token::Digit(d) => lhs = *d,
            Token::Add => lhs += eval_rhs1(expr),
            Token::Mul => lhs *= eval_rhs1(expr),
            Token::RPar => return lhs,
            Token::WS => continue,
            _ => panic!("Unexpected token"),
        }
    }
    lhs
}

pub fn part1(expressions: &MathProblems) -> u64 {
    let mut sum = 0;
    for expr in expressions {
        sum += eval1(&mut expr.iter());
    }
    sum
}

fn eval_rhs2(iter: &mut Iter<Token>) -> u64 {
    while let Some(t) = iter.next() {
        match t {
            Token::Digit(d) => return *d,
            Token::LPar => return eval2(iter, true),
            Token::WS => continue,
            _ => panic!("Unexpected token"),
        }
    }
    0
}

fn next_rpar<'a>(expr: &'a mut Iter<Token>, consume_rpar: bool) -> Option<&'a Token> {
    let s = expr.as_slice();
    match !s.is_empty() && s[0] == Token::RPar && !consume_rpar {
        true => None,
        false => expr.next(),
    }
}

fn eval2(expr: &mut Iter<Token>, consume_rpar: bool) -> u64 {
    let mut lhs = eval_rhs2(expr);
    while let Some(t) = next_rpar(expr, consume_rpar) {
        match t {
            Token::Digit(d) => lhs = *d,
            Token::Add => lhs += eval_rhs2(expr),
            Token::Mul => lhs *= eval2(expr, false),
            Token::RPar => return lhs,
            Token::WS => continue,
            _ => panic!("Unexpected token"),
        }
    }
    lhs
}

pub fn part2(expressions: &MathProblems) -> u64 {
    let mut sum = 0;
    for expr in expressions {
        sum += eval2(&mut expr.iter(), false);
    }
    sum
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 378);
    assert_eq!(part1(&d), 8929569623593);
    assert_eq!(part2(&d), 231235959382961);
}

#[test]
fn test_data() {
    let data = // Example data
"1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 71 + 51 + 26 + 437 + 12240 + 13632);
    assert_eq!(part2(&d), 231 + 51 + 46 + 1445 + 669060 + 23340);
}
