use crate::GroupBlankLine;
use itertools::Itertools;
use std::str::FromStr;

const INPUT_FILE: &str = "data/day4.txt";

pub type Passport = Vec<(FieldType, String)>;

pub fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Vec<Passport> {
    line_source.into_iter().group_by_blanks(parse_group)
}

fn parse_group<S: AsRef<str>>(group_iter: &mut (dyn Iterator<Item = S>)) -> Passport {
    group_iter.fold(Passport::with_capacity(8), |mut passport, line| {
        passport.extend(line.as_ref().split_ascii_whitespace().map(|field| {
            field
                .split(':')
                .next_tuple()
                .map(|(t, v)| (t.parse::<FieldType>().unwrap(), v.to_string()))
                .unwrap()
        }));
        passport
    })
}

#[derive(PartialEq, Debug)]
pub enum FieldType {
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

fn fields_present(p: &Passport) -> bool {
    p.len() == 8 || p.len() == 7 && p.iter().find(|(t, _)| *t == FieldType::Cid).is_none()
}

fn part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| fields_present(p)).count()
}

fn part2(passports: &[Passport]) -> usize {
    fn valid(p: &Passport) -> bool {
        let eye_color = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !fields_present(p) {
            return false;
        }
        p.iter().all(|(t, val)| match t {
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
            FieldType::Pid => val.len() == 9 && val.chars().all(|c| c.is_ascii_digit()),
            FieldType::Cid => true,
        })
    }

    passports.iter().filter(|p| valid(&p)).count()
}

#[test]
fn real_data() {
    let x = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(x.len(), 282);

    assert_eq!(x[0].len(), 8);
    assert_eq!(x[0][0].1, "grn");

    assert_eq!(part1(&x), 226);
    assert_eq!(part2(&x), 160);
}

#[test]
fn test_data() {
    let d = // Example
"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    let d = load_input(d.lines());
    assert_eq!(part1(&d), 2);
    assert_eq!(part2(&d), 2);
}
