use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn load_input() -> Vec<LineType> {
    let mut file = File::open("data/day6.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s.trim_end()
        .split("\n\n")
        .map(|grp| {
            (
                grp.split('\n').count(),
                grp.chars()
                    .filter(|c| c.is_alphabetic())
                    .fold(HashMap::new(), |mut cnt, c| {
                        *cnt.entry(c).or_insert(0) += 1;
                        cnt
                    }),
            )
        })
        .collect()
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
    let d = load_input();
    assert_eq!(d.len(), 487);
    assert_eq!(part1(&d), 6565);
    assert_eq!(part2(&d), 3137);
}
