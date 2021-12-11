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
    intcode.run_until_end(&[5])[0]
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 4887191);
    assert_eq!(part2(&d), 3419022);
}

#[test]
fn test_data() {
    // test eq
    let m = Intcode::load_program("3,9,8,9,10,9,4,9,99,-1,8");
    for (i, o) in [(8, 1), (9, 0)] {
        assert_eq!(m.clone().run_until_end(&[i])[0], o);
    }
    // test eq imm
    let m = Intcode::load_program("3,3,1108,-1,8,3,4,3,99");
    for (i, o) in [(8, 1), (9, 0)] {
        assert_eq!(m.clone().run_until_end(&[i])[0], o);
    }
    // jump pos
    let m = Intcode::load_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    for (i, o) in [(8, 1), (0, 0)] {
        assert_eq!(m.clone().run_until_end(&[i])[0], o);
    }
}
