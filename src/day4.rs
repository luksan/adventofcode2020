use adventofcode2020::DayOfAdvent;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn load_input() -> Vec<Passport> {
    let mut file = File::open("data/day4.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s.split("\n\n")
        .map(|p| {
            Passport::new(
                p.split_ascii_whitespace()
                    .map(|e| {
                        e.split(':')
                            .next_tuple()
                            .map(|(t, v)| (t.parse::<FieldType>().unwrap(), v.to_string()))
                            .unwrap()
                    })
                    //.map(|e| (FieldType::Byr, e.to_string()))
                    .collect(),
            )
        })
        .collect()
}

struct Passport {
    fields: Vec<(FieldType, String)>,
}

impl Passport {
    fn new(fields: Vec<(FieldType, String)>) -> Self {
        Self { fields }
    }
}

#[derive(PartialEq, Debug)]
enum FieldType {
    Byr, //(Birth Year)
    Iyr, //(Issue Year)
    Eyr, //(Expiration Year)
    Hgt, //(Height)
    Hcl, //(Hair Color)
    Ecl, //(Eye Color)
    Pid, //(Passport ID)
    Cid, //(Country ID)
}

impl FromStr for FieldType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FieldType::*;
        Ok(match s {
            "byr" => Byr, //(Birth Year)
            "iyr" => Iyr, //(Issue Year)
            "eyr" => Eyr, //(Expiration Year)
            "hgt" => Hgt, //(Height)
            "hcl" => Hcl, //(Hair Color)
            "ecl" => Ecl, //(Eye Color)
            "pid" => Pid, //(Passport ID)
            "cid" => Cid, //(Country ID)
            f => unreachable!("Bad field type {}", f),
        })
    }
}
struct Solver {
    p1: usize,
    p2: usize,
}

fn part1(passports: &Vec<Passport>) -> usize {
    fn valid(p: &Passport) -> bool {
        p.fields.len() == 8
            || p.fields.len() == 7
                && p.fields
                    .iter()
                    .find(|(t, _)| *t == FieldType::Cid)
                    .is_none()
    }
    passports.iter().filter(|p| valid(p)).count()
}

fn part2(passports: &Vec<Passport>) -> usize {
    fn valid(p: &Passport) -> bool {
        let eye_color = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !(p.fields.len() == 8
            || p.fields.len() == 7
                && p.fields
                    .iter()
                    .find(|(t, _)| *t == FieldType::Cid)
                    .is_none())
        {
            return false;
        }
        p.fields.iter().all(|(t, val)| match t {
            FieldType::Byr => val.parse::<u32>().map_or(false, |y| y >= 1920 && y <= 2002),
            FieldType::Iyr => val.parse::<u32>().map_or(false, |y| y >= 2010 && y <= 2020),
            FieldType::Eyr => val.parse::<u32>().map_or(false, |y| y >= 2020 && y <= 2030),
            FieldType::Hgt => {
                scan_fmt::scan_fmt!(val, "{d}{}", u32, String).map_or(false, |(len, unit)| {
                    match unit.as_str() {
                        "cm" => len >= 150 && len <= 193,
                        "in" => len >= 59 && len <= 76,
                        _ => false,
                    }
                })
            }
            FieldType::Hcl => {
                val.starts_with('#') && val.len() == 7 && u32::from_str_radix(&val[1..], 16).is_ok()
            }
            FieldType::Ecl => eye_color.contains(&val.as_str()),
            FieldType::Pid => val.len() == 9 && val.parse::<u32>().is_ok(),
            FieldType::Cid => true,
        })
    }

    passports.iter().filter(|p| valid(&p)).count()
}

impl Solver {
    fn new() -> Self {
        Self { p1: 0, p2: 0 }
    }

    fn part1(&mut self) {}

    fn part2(&mut self) {}
}
pub fn solve() -> Box<dyn DayOfAdvent> {
    let mut x = Solver::new();

    x.part1();

    Box::new(x)
}

impl DayOfAdvent for Solver {
    fn day(&self) -> u32 {
        4
    }

    fn result_strings(&self) -> Vec<String> {
        let ret = Vec::new();

        ret
    }
}

#[test]
fn real_data() {
    let x = load_input();
    assert_eq!(x.len(), 282);

    assert_eq!(x[0].fields.len(), 8);
    assert_eq!(x[0].fields[0].1, "grn");

    assert_eq!(part1(&x), 226);
    assert_eq!(part2(&x), 160);
}
