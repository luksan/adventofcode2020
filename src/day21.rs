use itertools::Itertools;

const INPUT_FILE: &str = "data/day21.txt";

type Foodstuffs = Vec<Food>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Foodstuffs {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Food {
    let s = s.as_ref();
    let (ingr, allerg) = s[..s.len() - 1] // Remove ')'
        .split(" (contains ")
        .collect_tuple()
        .unwrap();
    Food {
        ingredients: ingr.split(' ').map(|s| s.to_owned()).collect(),
        allergens: allerg.split(", ").map(|s| s.to_owned()).collect(),
    }
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn part1(foods: &Foodstuffs) -> usize {
    0
}

fn part2(_lines: &Foodstuffs) -> usize {
    0
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 33);
    assert_eq!(part1(&d), 1);
    // assert_eq!(part2(&d), 1);
}

#[test]
fn test_data() {
    let data = // Example data
"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 5);
    // assert_eq!(part2(&d), 1);
}
