use crate::y2019::intcode::Intcode;

use itertools::Itertools;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Intcode {
    Intcode::load_program(line_source.into_iter().next().unwrap().as_ref())
}

fn part1(pre_run: &Intcode) -> isize {
    let mut out_max = 0;

    let phases = [0, 1, 2, 3, 4];

    for phases in phases.iter().copied().permutations(5) {
        let mut input = 0;
        for p in phases.into_iter() {
            input = pre_run.clone().run(&[p, input])[0];
        }
        if input > out_max {
            out_max = input;
        }
    }

    out_max
}

fn part2(pre_run: &Intcode) -> isize {
    let mut intcode = pre_run.clone();
    intcode.run(&[5])[0]
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 24635);
    assert_eq!(part2(&d), 3419022);
}

#[test]
fn test_data() {}
