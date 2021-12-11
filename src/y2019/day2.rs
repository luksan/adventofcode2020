use crate::y2019::intcode::Intcode;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Intcode {
    Intcode::load_program(line_source.into_iter().next().unwrap().as_ref())
}

fn part1(intcode: &Intcode) -> usize {
    let mut intcode = intcode.clone();
    intcode.run_until_end(&[]);
    intcode.peek(0) as usize
}

fn part2(pre_run: &Intcode) -> isize {
    for verb in 0..100 {
        for noun in 0..100 {
            let mut intcode = pre_run.clone();
            intcode.poke(1, noun);
            intcode.poke(2, verb);
            intcode.run_until_end(&[]);
            if intcode.peek(0) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

#[test]
fn real_data() {
    let mut d = load_input(crate::load_strings(crate::data_file!()));
    d.poke(1, 12);
    d.poke(2, 2);
    assert_eq!(part1(&d), 3058646);
    assert_eq!(part2(&d), 8976);
}

#[test]
fn test_data() {
    let data = // Example data
"1,9,10,3,2,3,11,0,99,30,40,50";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 3500);
}
