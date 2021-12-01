use crate::GroupBlankLine;
use counter::Counter;

const INPUT_FILE: &str = "data/day6.txt";

type Group = (usize, Counter<char>); // (size of group, answer count)

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Vec<Group> {
    line_source.into_iter().group_by_blanks(parse_group)
}

fn parse_group<S: AsRef<str>>(group_iter: &mut (dyn Iterator<Item = S>)) -> Group {
    group_iter.fold((0, Counter::new()), |(grp_size, mut answers), line| {
        answers.update(line.as_ref().chars());
        (grp_size + 1, answers)
    })
}

fn part1(lines: &[Group]) -> usize {
    lines.iter().map(|(_, cnt)| cnt.len()).sum()
}

fn part2(lines: &[Group]) -> usize {
    lines
        .iter()
        .map(|(grp_size, answers)| answers.iter().filter(|(_q, a)| *a == grp_size).count())
        .sum()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 487);
    assert_eq!(part1(&d), 6565);
    assert_eq!(part2(&d), 3137);
}

#[test]
fn test_data() {
    let d = // Example
"abc

a
b
c

ab
ac

a
a
a
a

b";
    let d = load_input(d.lines());
    assert_eq!(part1(&d), 11);
    assert_eq!(part2(&d), 6);
}
