use lazy_static::lazy_static;
use regex::Regex;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

type BagRule = (SmolStr, HashMap<SmolStr, usize>);
type Rules = HashMap<SmolStr, HashMap<SmolStr, usize>>;

fn load_input() -> Rules {
    crate::load_input("data/day7.txt", parse)
}

fn parse<S: AsRef<str>>(s: S) -> BagRule {
    lazy_static! {
        static ref SUBJ: Regex = Regex::new(r"^(?P<subj>.+?) bags contain ").unwrap();
        static ref RULE: Regex =
            Regex::new(r"(?:(?P<count>\d+) (?P<color>.+?) bags?(?:, |\.$)?)").unwrap();
    }
    let s = s.as_ref();
    let subj = SUBJ.captures(s).unwrap();
    let rules_str = &s[subj.get(0).unwrap().end()..];

    let rules = RULE
        .captures_iter(rules_str)
        .map(|cap| (SmolStr::new_inline(&cap[2]), cap[1].parse().unwrap()))
        .collect();

    (SmolStr::new_inline(&subj[1]), rules)
}

fn inside_out(rules: &Rules) -> HashMap<&SmolStr, HashSet<&SmolStr>> {
    let mut ret = HashMap::new();
    for (bag, contains) in rules {
        for inside in contains.keys() {
            ret.entry(inside).or_insert_with(HashSet::new).insert(bag);
        }
    }
    ret
}

fn part1(rules: &Rules) -> usize {
    let my_bag = SmolStr::new_inline("shiny gold");
    let rev_map = inside_out(&rules);

    let mut bags: HashSet<&SmolStr> = HashSet::new();
    bags.extend(rev_map[&my_bag].iter());

    let mut new_bags = bags.clone();
    while !new_bags.is_empty() {
        let new = new_bags.iter().fold(Vec::new(), |mut new, bag| {
            if let Some(contains) = rev_map.get(*bag) {
                new.push(contains.iter())
            }
            new
        });

        new_bags.clear();

        for bag in new.into_iter().flatten() {
            if bags.insert(bag) {
                new_bags.insert(bag);
            }
        }
    }
    bags.len()
}

fn bag_contains(bag: &SmolStr, rules: &Rules) -> usize {
    rules[bag]
        .iter()
        .map(|(bag, cnt)| cnt + cnt * bag_contains(bag, rules))
        .sum()
}

fn part2(rules: &Rules) -> usize {
    let my_bag = SmolStr::new_inline("shiny gold");
    bag_contains(&my_bag, rules)
}

#[test]
fn real_data() {
    let d = load_input();
    assert_eq!(d.len(), 594);
    assert_eq!(part1(&d), 169);
    assert_eq!(part2(&d), 82372);
}

#[test]
fn test_data() {
    let input = // cargo fmt comment :)
"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    let rules: Rules = input.lines().map(parse).collect();
    assert_eq!(rules["light red"]["bright white"], 1);

    assert_eq!(part1(&rules), 4);
    assert_eq!(part2(&rules), 32);
}
