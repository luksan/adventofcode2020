use crate::grid::Coord;
use itertools::Itertools;

type Dots = Vec<Coord>;
type Folds = Vec<(u8, i32)>;

type Input = (Dots, Folds);

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    let mut lines = line_source.into_iter();
    let dots = (&mut lines)
        .take_while(|p| !p.as_ref().is_empty())
        .map(|l| l.as_ref().parse().unwrap())
        .collect();

    let folds = lines.map(parse).collect();
    (dots, folds)
}

fn parse<S: AsRef<str>>(s: S) -> (u8, i32) {
    let (axis, pos) = s
        .as_ref()
        .trim_start_matches("fold along ")
        .split('=')
        .collect_tuple()
        .unwrap();
    (axis.as_bytes()[0], pos.parse().unwrap())
}

fn part1(input: &Input) -> usize {
    let (dots, folds) = input;
    let mut dots = dots.clone();
    fold_once(folds[0], &mut dots);
    dots.len()
}

fn fold_once(fold: (u8, i32), dots: &mut Vec<Coord>) {
    match fold.0 {
        b'x' => {
            let line = fold.1;
            for c in dots.iter_mut() {
                if c.x > line {
                    c.x = line - (c.x - line);
                }
            }
            dots.retain(|d| d.x != line);
        }
        b'y' => {
            let line = fold.1;
            for c in dots.iter_mut() {
                if c.y > line {
                    c.y = line - (c.y - line);
                }
            }
            dots.retain(|d| d.y != line);
        }
        _ => unreachable!(),
    }
    dots.sort_unstable();
    dots.dedup();
}

fn part2(input: &Input) -> usize {
    let (dots, folds) = input;
    let mut dots = dots.clone();
    for f in folds {
        fold_once(*f, &mut dots);
    }
    let x_max = dots.iter().map(|d| d.x).max().unwrap();
    let y_max = dots.iter().map(|d| d.y).max().unwrap();
    for y in 0..=y_max {
        for x in 0..=x_max {
            if dots.contains(&Coord { x, y }) {
                print!("\u{2588}"); // Full block
            } else {
                print!(" ");
            }
        }
        println!();
    }

    dots.len()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 751);
    // PGHRKLKL
    assert_eq!(part2(&d), 95);
}

#[test]
fn test_data() {
    let data = // Example data
"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 17);
    assert_eq!(part2(&d), 16);
}
