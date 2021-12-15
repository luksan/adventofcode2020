use crate::grid::{Coord, Grid};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

type LineType = String;
type Input = Grid<u8>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    Grid::from_lines(line_source, |p| p as u8 - b'0')
}

fn part1(input: &Input) -> usize {
    a_star(input)
}

// https://en.wikipedia.org/wiki/A*_search_algorithm
fn a_star(grid: &Input) -> usize {
    let h = |n: Coord| (grid.width() - n.x).abs() + (grid.height() - n.y).abs();
    let bottom_right = Coord {
        x: grid.width() - 1,
        y: grid.height() - 1,
    };

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct FScore {
        c: Coord,
        f_score: i32,
    }
    impl PartialOrd for FScore {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for FScore {
        fn cmp(&self, other: &Self) -> Ordering {
            // reverse ordering so that we get a min-heap
            self.f_score
                .cmp(&other.f_score)
                .reverse()
                .then(self.c.cmp(&other.c))
        }
    }
    let start = Coord { x: 0, y: 0 };
    // This should be a Fibonacci heap, or similar
    let mut open_heap: BinaryHeap<FScore> = BinaryHeap::new();
    // Might be better to skip this and run contains() on the heap?
    let mut open_set = HashSet::new();
    open_set.insert(start);
    open_heap.push(FScore {
        c: start,
        f_score: h(start),
    });
    let mut g_score = HashMap::new();
    g_score.insert(start, 0i32);

    while let Some(curr) = open_heap.pop() {
        open_set.remove(&curr.c);
        if curr.c == bottom_right {
            return g_score[&curr.c] as usize;
        }
        for (n, &cost) in grid.updownleftright(curr.c) {
            let tentative_g = g_score[&curr.c] + cost as i32;
            if tentative_g < *g_score.get(&n).unwrap_or(&i32::MAX) {
                g_score.insert(n, tentative_g);
                if !open_set.contains(&n) {
                    open_set.insert(n);
                    let f_score = tentative_g + h(n);
                    open_heap.push(FScore { c: n, f_score })
                }
            }
        }
    }
    0
}

fn part2(input: &Input) -> usize {
    let mut x5 = Vec::with_capacity((input.width() * input.height() * 25) as usize);
    for row in 0..5 {
        for input_row in input.row_slices() {
            for col in 0..5 {
                x5.extend(input_row.iter().map(|&c| {
                    let c = c + row + col;
                    if c > 9 {
                        c - 9
                    } else {
                        c
                    }
                }));
            }
        }
    }

    let x5 = Grid::new(x5, input.width() * 5, input.height() * 5);
    a_star(&x5)
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 553);
    assert_eq!(part2(&d), 2858);
}

#[test]
fn test_data() {
    let data = // Example data
"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 40);
    assert_eq!(part2(&d), 315);
}
