use crate::grid::{Coord, Grid};
use itertools::Itertools;

type Lines = Vec<Line>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Lines {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Line {
    let (start, end) = s.as_ref().split(" -> ").collect_tuple().unwrap();
    Line {
        start: start.parse().unwrap(),
        end: end.parse().unwrap(),
    }
}

#[derive(Clone, Debug)]
struct Line {
    start: Coord,
    end: Coord,
}

fn part1(lines: &Lines) -> usize {
    let grid_size = 1000;
    let mut grid = Grid::new(
        vec![0u16; grid_size * grid_size],
        grid_size as i32,
        grid_size as i32,
    );
    for l in lines {
        if l.start.x != l.end.x && l.start.y != l.end.y {
            continue; // Skip non horizontal / vertical lines
        }
        for c in l.start.line_to(l.end) {
            grid[c] += 1;
        }
    }
    grid.iter_tiles().filter(|(_, &t)| t >= 2).count()
}

fn part2(lines: &Lines) -> usize {
    let grid_size = 1000;
    let mut grid = Grid::new(
        vec![0u16; grid_size * grid_size],
        grid_size as i32,
        grid_size as i32,
    );
    for l in lines {
        for c in l.start.line_to(l.end) {
            grid[c] += 1;
        }
    }
    grid.iter_tiles().filter(|(_, &t)| t >= 2).count()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 4873);
    assert_eq!(part2(&d), 19472);
}

#[test]
fn test_data() {
    let data = // Example data
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 5);
    assert_eq!(part2(&d), 12);
}
