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

pub fn day1() {
    let mut arr = load_input();
    arr.sort_unstable();

    // part 1
    for a in &arr {
        let b = 2020 - a;
        if arr.binary_search(&b).is_ok() {
            println!("Part 1: {} * {} = {}", a, b, a * b);
            break;
        }
    }

    // part 2
    let len = arr.len();
    'part2: for ai in 0..len {
        let a = arr[ai];
        for bi in ai..len {
            let b = arr[bi];
            let c = 2020_u32.saturating_sub(a + b);
            if let Ok(ci) = arr.binary_search(&c) {
                if ai == ci || bi == ci {
                    continue;
                }
                println!("Part 2: {} * {} * {} = {}", a, b, c, a * b * c);
                break 'part2;
            }
        }
    }
}
