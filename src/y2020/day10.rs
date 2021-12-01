use std::collections::HashMap;

const INPUT_FILE: &str = "data/2020/day10.txt";

type Adapter = i32;
type Bag = Vec<i32>;

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Bag {
    let mut x: Bag = line_source.into_iter().map(parse).collect();
    x.push(0); // the outlet
    x.sort_unstable();
    x.push(x.last().unwrap() + 3); // built-in adapter
    x
}

fn parse<S: AsRef<str>>(s: S) -> Adapter {
    let line = s.as_ref();
    line.parse::<Adapter>().unwrap()
}

pub fn part1(bag: &Bag) -> i32 {
    let mut diff1 = 0;
    let mut diff3 = 0;
    for pair in bag.windows(2) {
        match pair[1] - pair[0] {
            1 => diff1 += 1,
            2 => {}
            3 => diff3 += 1,
            _ => unreachable!("Bad input data"),
        }
    }
    diff1 * diff3
}

fn p2_recursive(mut adapter_idx: usize, bag: &Bag, cache: &mut HashMap<Adapter, usize>) -> usize {
    if adapter_idx >= bag.len() - 1 {
        return 1;
    }

    let adapter = bag[adapter_idx];
    let mut cnt = 0;
    adapter_idx += 1;

    while let Some(next) = bag.get(adapter_idx) {
        if *next > adapter + 3 {
            break;
        }

        cnt += match cache.get(next) {
            Some(partial) => *partial,
            None => {
                let partial = p2_recursive(adapter_idx, bag, cache);
                cache.insert(*next, partial);
                partial
            }
        };
        adapter_idx += 1;
    }
    cnt
}

pub fn part2(bag: &Bag) -> usize {
    let mut cache = HashMap::with_capacity(bag.len());
    p2_recursive(0, bag, &mut cache)
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 107 + 2);
    assert_eq!(part1(&d), 2475);
    assert_eq!(part2(&d), 442136281481216);
}

#[test]
fn test_data() {
    let data = // Example data
"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    let d: Bag = load_input(data.lines());
    assert_eq!(part1(&d), 22 * 10);
    assert_eq!(part2(&d), 19208);
}
