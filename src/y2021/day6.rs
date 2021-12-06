use std::collections::VecDeque;

type Fishes = Vec<usize>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Fishes {
    line_source
        .into_iter()
        .next()
        .unwrap()
        .as_ref()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

struct FishLoop {
    days: VecDeque<usize>,
}

impl FishLoop {
    fn new(fish: &[usize]) -> Self {
        let mut days = vec![0; 9];
        for &f in fish {
            days[f] += 1;
        }
        Self { days: days.into() }
    }

    fn next_day(&mut self) {
        self.days.rotate_left(1); // idx 0 -> idx 8
        self.days[6] += self.days[8];
    }

    fn total_size(&self) -> usize {
        self.days.iter().sum()
    }
}

fn part1(school: &Fishes) -> usize {
    let mut d = FishLoop::new(school);
    for _n in 0..80 {
        d.next_day()
    }
    d.total_size()
}

fn part2(school: &Fishes) -> usize {
    let mut d = FishLoop::new(school);
    for _n in 0..256 {
        d.next_day();
    }
    d.total_size()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 363101);
    assert_eq!(part2(&d), 1644286074024);
}

#[test]
fn test_data() {
    let data = // Example data
"3,4,3,1,2";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 5934);
    assert_eq!(part2(&d), 26984457539);
}
