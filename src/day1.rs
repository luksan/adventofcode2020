fn load_input() -> Vec<u32> {
    let mut lines: Vec<_> = crate::load_input("data/day1.txt", |s| s.parse::<u32>().unwrap());
    lines.sort_unstable();
    lines
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

#[test]
fn test_day1() {
    let arr = load_input();
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
