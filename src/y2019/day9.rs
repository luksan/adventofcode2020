use crate::y2019::intcode::Intcode;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Intcode {
    Intcode::load_program(line_source.into_iter().next().unwrap().as_ref())
}

fn part1(pre_run: &Intcode) -> isize {
    let mut intcode = pre_run.clone();
    let r = intcode.run_until_end(&[1]);
    *r.last().unwrap()
}

fn part2(pre_run: &Intcode) -> isize {
    let mut intcode = pre_run.clone();
    intcode.run_until_end(&[2])[0]
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 3765554916);
    assert_eq!(part2(&d), 76642);
}
