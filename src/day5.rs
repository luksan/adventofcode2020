fn load_input() -> Vec<Seat> {
    let mut seats = crate::load_input("data/day5.txt", parse);
    seats.sort_unstable();
    seats
}

fn parse(s: String) -> Seat {
    let mut ci = s.chars();
    let (row, x) = (&mut ci).take(7).fold((0, 127), |(a, b), c| {
        let d = (b - a + 1) / 2;
        match c {
            'F' => (a, b - d),
            'B' => (a + d, b),
            _ => unreachable!("Bad row char"),
        }
    });
    assert_eq!(row, x);
    let (col, x) = ci.fold((0, 7), |(a, b), c| {
        let d = (b - a + 1) / 2;
        match c {
            'L' => (a, b - d),
            'R' => (a + d, b),
            _ => unreachable!("Bad row char"),
        }
    });
    assert_eq!(col, x);
    row * 8 + col
}

type Seat = u32;

fn part1(seats: &[Seat]) -> u32 {
    *seats.last().unwrap()
}

fn part2(seats: &[Seat]) -> u32 {
    seats
        .iter()
        .zip(seats.iter().skip(1))
        .find(|(&a, &b)| b - a == 2)
        .map(|(a, _)| a + 1)
        .unwrap()
}

#[test]
fn test_real_data() {
    let d = load_input();
    assert_eq!(d.len(), 798);
    assert_eq!(part1(&d), 883);
    assert_eq!(part2(&d), 532);
}
