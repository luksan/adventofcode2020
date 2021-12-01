use ndarray::prelude::*;

const INPUT_FILE: &str = "data/day17.txt";

pub type Cube = u8;

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Array2<Cube> {
    let mut grid = Vec::new();
    let mut x_init = 0;
    for l in line_source.into_iter() {
        let mut x = parse(l);
        x_init = x.len();
        grid.append(&mut x);
    }
    Array2::from_shape_vec((grid.len() / x_init, x_init), grid).unwrap()
}

fn parse<S: AsRef<str>>(s: S) -> Vec<Cube> {
    s.as_ref()
        .chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => unreachable!("Bad input"),
        })
        .collect()
}

fn neighbours_one_r3() -> Array3<Cube> {
    let mut one = Array3::ones((3, 3, 3));
    one[[1, 1, 1]] = 0;
    one
}

fn neighbours_one_r4() -> Array4<Cube> {
    let mut one = Array4::ones((3, 3, 3, 3));
    one[[1, 1, 1, 1]] = 0;
    one
}

fn count_live<D: Dimension>(rn: ArrayView<Cube, D>) -> usize {
    rn.fold(0, |sum, v| sum + *v as usize)
}

fn init_r3(initial: &Array2<Cube>) -> Array3<Cube> {
    let (y0, x0) = initial.dim();
    let growth = 6;
    let z_size = 2 * growth + 1;
    let y_size = y0 + 2 * growth + 1;
    let x_size = x0 + 2 * growth + 1;

    // let dim 1 be z, since dbg!() prints slices ocf dim 2,3
    let mut r3 = Array3::zeros((z_size, y_size, x_size));
    let y_offset = (y_size - y0) / 2;
    let x_offset = (x_size - x0) / 2;

    r3.slice_mut(s![growth, y_offset..y_offset + y0, x_offset..x_offset + x0])
        .assign(initial);
    r3
}

fn update_rn<D: Dimension>(mut rn: ArrayViewMut<Cube, D>, count: ArrayView<Cube, D>) {
    azip!((n in &count, cube in &mut rn) {
        if *cube == 1 && *n !=2 && *n != 3 {
            *cube = 0;
        }
        if *cube == 0 && *n == 3 {
            *cube = 1;
        }
    });
}

pub fn part1(initial: &Array2<Cube>) -> usize {
    let mut r3 = init_r3(initial);
    let mut count = Array3::zeros(r3.dim());
    let ones = neighbours_one_r3();

    for _ in 0..6 {
        count.fill(0);
        for ((z, y, x), v) in r3.indexed_iter() {
            if *v == 1 {
                let mut s = count.slice_mut(s![z - 1..=z + 1, y - 1..=y + 1, x - 1..=x + 1]);
                if s.dim() == (3, 3, 3) {
                    s += &ones;
                }
            }
        }
        update_rn(r3.view_mut(), count.view());
    }

    count_live(r3.view())
}

fn init_r4(initial: &Array2<Cube>) -> Array4<Cube> {
    let (y0, x0) = initial.dim();
    let growth = 6;
    let w_size = 2 * growth + 1;
    let z_size = w_size;
    let y_size = y0 + 2 * growth + 1;
    let x_size = x0 + 2 * growth + 1;

    // let dim 1 be z, since dbg!() prints slices ocf dim 2,3
    let mut r4 = Array4::zeros((w_size, z_size, y_size, x_size));
    let y_offset = (y_size - y0) / 2;
    let x_offset = (x_size - x0) / 2;

    r4.slice_mut(s![
        growth,
        growth,
        y_offset..y_offset + y0,
        x_offset..x_offset + x0
    ])
    .assign(initial);
    r4
}

pub fn part2(initial: &Array2<Cube>) -> usize {
    let mut r4 = init_r4(initial);
    let mut count = Array4::<Cube>::zeros(r4.dim());
    let ones = neighbours_one_r4();

    for _ in 0..6 {
        count.fill(0);
        for ((w, z, y, x), v) in r4.indexed_iter() {
            if *v == 1 {
                let mut s = count.slice_mut(s![
                    w - 1..=w + 1,
                    z - 1..=z + 1,
                    y - 1..=y + 1,
                    x - 1..=x + 1
                ]);

                if s.dim() == (3, 3, 3, 3) {
                    s += &ones;
                }
            }
        }
        update_rn(r4.view_mut(), count.view());
    }
    count_live(r4.view())
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.dim(), (8, 8));
    assert_eq!(part1(&d), 267);
    assert_eq!(part2(&d), 1812);
}

#[test]
fn test_data() {
    let data = // Example data
".#.
..#
###";
    let d = load_input(data.lines());
    assert_eq!(count_live(d.view()), 5);
    assert_eq!(part1(&d), 112);
    assert_eq!(part2(&d), 848);
}
