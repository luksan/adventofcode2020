type Crabs = Vec<usize>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Crabs {
    line_source
        .into_iter()
        .next()
        .unwrap()
        .as_ref()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn parse<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().parse::<usize>().unwrap()
}

fn part1(cs: &Crabs) -> usize {
    let max_pos = *cs.iter().max().unwrap();
    let mut cost = vec![0usize; max_pos + 1];
    for &c in cs {
        cost[c..].iter_mut().enumerate().for_each(|(c, s)| *s += c);
        cost[0..c + 1]
            .iter_mut()
            .rev()
            .enumerate()
            .for_each(|(c, s)| *s += c);
    }

    *cost.iter().min().unwrap()
}

fn part2(cs: &Crabs) -> usize {
    let max_pos = *cs.iter().max().unwrap();
    let mut cost = vec![0usize; max_pos + 1];
    for &c in cs {
        let mut curr_cost = 0;
        cost[c..].iter_mut().enumerate().for_each(|(n, c)| {
            curr_cost += n;
            *c += curr_cost;
        });
        curr_cost = 0;
        cost[0..c + 1]
            .iter_mut()
            .rev()
            .enumerate()
            .for_each(|(n, c)| {
                curr_cost += n;
                *c += curr_cost;
            });
    }
    *cost.iter().min().unwrap()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 347449);
    assert_eq!(part2(&d), 98039527);
}

#[test]
fn test_data() {
    let data = // Example data
"16,1,2,0,4,2,7,1,2,14";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 37);
    assert_eq!(part2(&d), 168);
}
