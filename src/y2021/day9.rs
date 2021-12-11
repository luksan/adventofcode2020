use crate::grid::Grid;

type Tile = u8;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Grid<Tile> {
    Grid::from_lines(line_source, |c| c as u8 - b'0')
}

fn part1(grid: &Grid<Tile>) -> usize {
    let mut risk = 0;
    for (c, &ct) in grid.iter_tiles() {
        if grid.updownleftright(c).all(|(_c, &t)| t > ct) {
            risk += (ct + 1) as usize;
        }
    }
    risk
}

fn part2(grid: &Grid<Tile>) -> usize {
    let mut grid = grid.clone();
    let mut lows = vec![];
    for c in grid.coords() {
        if grid.updownleftright(c).all(|(_c, &t)| t > grid[c]) {
            lows.push(c);
        }
    }
    let mut basins = Vec::with_capacity(lows.len());
    let mut candidates = Vec::with_capacity(20);
    for low in lows {
        let mut basin_size = 1;
        grid[low] = 9;
        candidates.extend(
            grid.updownleftright(low)
                .filter(|(_c, &t)| t < 9)
                .map(|(c, _t)| c),
        );
        while let Some(c) = candidates.pop() {
            if grid[c] >= 9 {
                continue;
            }
            basin_size += 1;
            grid[c] = 9;
            candidates.extend(
                grid.updownleftright(c)
                    .filter(|(_c, &t)| t < 9)
                    .map(|(c, _t)| c),
            );
        }

        basins.push(basin_size);
        candidates.clear();
    }
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 498);
    assert_eq!(part2(&d), 1071000);
}

#[test]
fn test_data() {
    let data = // Example data
"2199943210
3987894921
9856789892
8767896789
9899965678";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 15);
    assert_eq!(part2(&d), 1134);
}
