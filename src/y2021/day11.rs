use crate::grid::Grid;

type OctoGrid = Grid<u8>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> OctoGrid {
    Grid::from_lines(line_source, |c| c.to_digit(10).unwrap() as u8)
}

fn part1(grid: &OctoGrid) -> usize {
    let mut grid = grid.clone();
    let mut blinks = vec![];
    let mut blinked = vec![];
    let mut blink_count = 0;
    for _cycle in 0..100 {
        for c in grid.coords() {
            grid[c] += 1;
            if grid[c] > 9 {
                blinks.push(c);
            }
        }
        while let Some(c) = blinks.pop() {
            blinked.push(c);
            blink_count += 1;
            for n in grid.neighbour_coords(c) {
                grid[n] += 1;
                if grid[n] == 10 {
                    blinks.push(n);
                }
            }
        }
        for b in blinked.drain(..) {
            grid[b] = 0;
        }
    }
    blink_count
}

fn part2(grid: &OctoGrid) -> usize {
    let mut grid = grid.clone();
    let mut blinks = vec![];
    let mut blinked = vec![];
    for cycle in 1..100000 {
        for c in grid.coords() {
            grid[c] += 1;
            if grid[c] > 9 {
                blinks.push(c);
            }
        }
        while let Some(c) = blinks.pop() {
            blinked.push(c);
            for n in grid.neighbour_coords(c) {
                grid[n] += 1;
                if grid[n] == 10 {
                    blinks.push(n);
                }
            }
        }
        if blinked.len() == (grid.height() * grid.width()) as usize {
            return cycle;
        }
        for b in blinked.drain(..) {
            grid[b] = 0;
        }
    }
    0
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 1773);
    assert_eq!(part2(&d), 494);
}

#[test]
fn test_data() {
    let data = // Example data
"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 1656);
    assert_eq!(part2(&d), 195);
}
