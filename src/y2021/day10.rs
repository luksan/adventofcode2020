use itertools::Itertools;

type LineType = Vec<u8>;
type LineContainer = Vec<LineType>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> LineContainer {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> LineType {
    s.as_ref().bytes().collect()
}

fn find_invalid(line: &LineType) -> Option<u8> {
    let mut stack = vec![];
    for &c in line {
        if [b'<', b'(', b'[', b'{'].contains(&(c as u8)) {
            stack.push(c);
        } else if let Some(open) = stack.pop() {
            let open = open as i8;
            let c2 = c as i8;
            if open - c2 < -2 || open - c2 > -1 {
                // mismatched braces
                return Some(c);
            } else {
                // OK
            }
        } else {
            // missing opening brace
            return Some(c);
        }
    }
    None
}

fn part1(lines: &LineContainer) -> usize {
    lines
        .iter()
        .filter_map(find_invalid)
        .map(|c| match c {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn complete_line(line: &LineType) -> usize {
    let mut stack = vec![];
    // prepare stack
    for &c in line.iter() {
        if [b'<', b'(', b'[', b'{'].contains(&(c as u8)) {
            stack.push(c as u8);
        } else {
            stack.pop();
        }
    }

    stack
        .into_iter()
        .rev()
        .map(|c| match c {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => unreachable!(),
        })
        .fold(0, |acc, score| acc * 5 + score)
}

fn part2(lines: &LineContainer) -> usize {
    let mut scores = lines
        .iter()
        .filter(|l| find_invalid(l).is_none())
        .map(complete_line)
        .collect_vec();

    scores.sort_unstable();
    *scores.get(scores.len() / 2).unwrap()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 341823);
    assert_eq!(part2(&d), 2801302861);
}

#[test]
fn test_data() {
    let data = // Example data
"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 26397);
    assert_eq!(part2(&d), 288957);
}
