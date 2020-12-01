use std::fs::File;
use std::io::{self, BufRead};

fn load_input() -> Vec<u32> {
    let file = File::open("data/day1/input").unwrap();
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .map(|num| num.unwrap().parse::<u32>().unwrap())
        .collect()
}

fn part1(arr: &[u32]) -> (u32, u32, u32) {
    for a in arr {
        let b = 2020 - a;
        if arr.binary_search(&b).is_ok() {
            return (*a, b, a * b);
        }
    }
    (0, 0, 0)
}

fn part2(arr: &[u32]) -> (u32, u32, u32, u32) {
    let len = arr.len();
    for ai in 0..len {
        let a = arr[ai];
        for bi in ai..len {
            let b = arr[bi];
            let c = 2020_u32.saturating_sub(a + b);
            if let Ok(ci) = arr.binary_search(&c) {
                if ai == ci || bi == ci {
                    continue;
                }
                return (a, b, c, a * b * c);
            }
        }
    }
    (0, 0, 0, 0)
}

pub fn day1() {
    let mut arr = load_input();
    arr.sort_unstable();

    println!("** Day 1 **");
    let (a, b, ab) = part1(&arr);
    println!("Part 1: {} * {} = {}", a, b, ab);

    let (a, b, c, abc) = part2(&arr);
    println!("Part 2: {} * {} * {} = {}", a, b, c, abc);
}

#[test]
fn test_day1() {
    let mut arr = load_input();
    arr.sort_unstable();
    assert_eq!(arr.len(), 200);
    assert_eq!(arr[0], 350);
    assert_eq!(part1(&arr), (618, 1402, 866436));
    assert_eq!(part2(&arr), (545, 547, 928, 276650720))
}

#[test]
fn test_example_data() {
    let mut arr = vec![1721, 979, 366, 299, 675, 1456];
    arr.sort_unstable();
    assert_eq!(part1(&arr), (299, 1721, 514579));
    assert_eq!(part2(&arr), (366, 675, 979, 241861950));
}
