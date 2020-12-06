use std::collections::HashMap;

fn load_groups() -> Vec<LineType> {
    crate::load_input_groups("data/day6.txt", parse_group)
}

fn parse_group(group_iter: &mut (dyn Iterator<Item = String>)) -> LineType {
    group_iter.fold((0, HashMap::new()), |(grp_size, mut answers), line| {
        line.chars()
            .for_each(|c| *answers.entry(c).or_insert(0) += 1);
        (grp_size + 1, answers)
    })
}

type LineType = (usize, HashMap<char, usize>);

fn part1(lines: &[LineType]) -> usize {
    lines.iter().map(|(_, cnt)| cnt.len()).sum()
}

fn part2(lines: &[LineType]) -> usize {
    lines
        .iter()
        .map(|(grp_size, answers)| answers.iter().filter(|(_q, a)| *a == grp_size).count())
        .sum()
}

#[test]
fn test_real_data() {
    let d = load_groups();
    assert_eq!(d.len(), 487);
    assert_eq!(part1(&d), 6565);
    assert_eq!(part2(&d), 3137);
}
