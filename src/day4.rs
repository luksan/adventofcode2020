use itertools::Itertools;
use std::str::FromStr;

type Passport = Vec<(FieldType, String)>;

fn load_groups() -> Vec<Passport> {
    crate::load_input_groups("data/day4.txt", parse_group)
}

fn parse_group(group_iter: &mut (dyn Iterator<Item = String>)) -> Passport {
    group_iter.fold(Passport::with_capacity(8), |mut passport, line| {
        passport.extend(line.split_ascii_whitespace().map(|field| {
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
    let x = load_groups();
    assert_eq!(x.len(), 282);

    assert_eq!(x[0].len(), 8);
    assert_eq!(x[0][0].1, "grn");

    assert_eq!(part1(&x), 226);
    assert_eq!(part2(&x), 160);
}
