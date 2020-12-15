pub type Number = u32;

fn elfgame(start_list: &[Number], until: Number) -> Number {
    let mut history = vec![0; until as usize];
    let mut prev = 0;
    let mut count = 0;

    for num in start_list {
        count += 1;
        history[*num as usize] = count;
        prev = *num;
    }

    while count < until {
        let a = history[prev as usize];
        history[prev as usize] = count;
        prev = if a == 0 { 0 } else { count - a };
        count += 1;
    }
    prev
}

pub fn part1(start_list: &[Number]) -> Number {
    elfgame(start_list, 2020)
}

pub fn part2(start_list: &[Number]) -> Number {
    elfgame(start_list, 30_000_000)
}

#[test]
fn real_data() {
    let d = vec![9, 12, 1, 4, 17, 0, 18];
    assert_eq!(part1(&d), 610);
    // assert_eq!(part2(&d), 1407);
}

#[test]
fn test_data() {
    let d = vec![0, 3, 6];
    assert_eq!(part1(&d), 436);
    // assert_eq!(part2(&d), 175594);
}
