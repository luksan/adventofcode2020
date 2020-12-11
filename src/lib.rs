#![allow(dead_code)]
#![allow(clippy::ptr_arg)]

use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

macro_rules! advent{
    ($($mod:ident),*) => {
         $(pub mod $mod;)*
    }
}

advent!(day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11);

pub fn buf_reader<P>(path: P) -> BufReader<File>
where
    P: AsRef<Path>,
{
    let file = File::open(path.as_ref()).unwrap();
    io::BufReader::new(file)
}

pub fn load_strings<P>(path: P) -> Box<dyn Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    Box::new(buf_reader(path).lines().map(|l| l.unwrap()))
}

pub fn load_input<C, R, P, Q>(path: P, conv: C) -> Q
where
    P: AsRef<Path>,
    C: FnMut(String) -> R,
    Q: FromIterator<R>,
{
    buf_reader(path)
        .lines()
        .map(|l| l.unwrap())
        .map(conv)
        .collect()
}

pub fn load_input_groups<C, R, P, Q>(path: P, group_parser: C) -> Q
where
    P: AsRef<Path>,
    C: Fn(&mut (dyn Iterator<Item = String>)) -> R,
    Q: FromIterator<R>,
{
    buf_reader(path)
        .lines()
        .map(|l| l.unwrap())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(empty, _)| !empty)
        .map(|(_, mut group)| group_parser(&mut group))
        .collect()
}
