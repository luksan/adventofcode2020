use crate::grid::Grid;
use crate::y2019::intcode::Intcode;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Intcode {
    Intcode::load_program(line_source.into_iter().next().unwrap().as_ref())
}

fn part1(pre_run: &Intcode) -> isize {
    let mut intcode = pre_run.clone();
    let r = intcode.run_until_end(&[]);
    let scaffold: String = r.iter().map(|&m| m as u8 as char).collect();
    let scaffold = Grid::from_lines(scaffold.trim_end().lines(), |c| c);

    println!("{}", scaffold);

    let mut tot_align = 0;
    for (c, &t) in scaffold.iter_tiles() {
        if t == '#' && scaffold.updownleftright(c).all(|(_, &t)| t == '#') {
            println!("{:?}", c);
            tot_align += c.x * c.y;
        }
    }
    tot_align as isize
}

fn part2(pre_run: &Intcode) -> isize {
    let mut intcode = pre_run.clone();
    intcode.poke(0, 2);
    intcode.run_until_end(&[])[0]
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 7280);
    // assert_eq!(part2(&d), 1);
}
