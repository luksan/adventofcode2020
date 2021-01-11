use counter::Counter;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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

type Allergen = String;
type Ingredient = String;

struct Food {
    ingredients: Vec<Ingredient>,
    allergens: Vec<Allergen>,
}

fn part1(foods: &[Food]) -> usize {
    let allergens = Allergens::from_foods(foods);

    let mut safe_ingr: HashSet<&Ingredient> =
        foods.iter().flat_map(|f| f.ingredients.iter()).collect();

    for ctr in allergens.0.values() {
        let x = ctr.most_common();
        let max_cnt = x[0].1;
        for (ingr, cnt) in x {
            if cnt < max_cnt {
                break;
            }
            safe_ingr.remove(ingr);
        }
    }

    foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .filter(|ingr| safe_ingr.contains(ingr))
        .count()
}

struct Allergens<'a>(HashMap<&'a Allergen, Counter<&'a Ingredient>>);

impl Allergens<'_> {
    fn from_foods(foods: &[Food]) -> Allergens {
        let mut allergens: HashMap<&Allergen, Counter<&Ingredient>> = HashMap::new();
        for food in foods {
            for al in &food.allergens {
                allergens
                    .entry(&al)
                    .or_insert_with(Counter::new)
                    .update(&food.ingredients);
            }
        }
        Allergens(allergens)
    }

    fn remove_allergen(&mut self, al: &Allergen) {
        self.0.remove(al);
    }

    fn remove_ingredient(&mut self, ingr: &Ingredient) {
        for ctr in self.0.values_mut() {
            ctr.remove(ingr);
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn find_dangerous_ingredient<'a>(allergen: &Allergens<'a>) -> (&'a Allergen, &'a Ingredient) {
    for (al, ctr) in &allergen.0 {
        if ctr.len() == 1 {
            return (&*al, ctr.keys().next().unwrap());
        }
    }
    for (al, ctr) in &allergen.0 {
        let x = ctr.most_common();
        if x[0].1 > x[1].1 {
            return (&*al, x[0].0);
        }
    }
    unreachable!("Yup.")
}

fn part2(foods: &[Food]) -> String {
    let mut allergens = Allergens::from_foods(foods);
    let mut cdil: Vec<(&Allergen, &Ingredient)> = Vec::new();
    while !allergens.is_empty() {
        let (al, ingr) = find_dangerous_ingredient(&allergens);
        cdil.push((al, ingr));
        allergens.remove_allergen(al);
        allergens.remove_ingredient(ingr);
    }

    cdil.sort_by_key(|k| k.0);
    cdil.iter().map(|x| x.1).join(",")
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 33);
    assert_eq!(part1(&d), 1882);
    assert_eq!(
        part2(&d),
        "xgtj,ztdctgq,bdnrnx,cdvjp,jdggtft,mdbq,rmd,lgllb"
    );
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
    assert_eq!(part2(&d), "mxmxvkd,sqjhc,fvjkl");
}
