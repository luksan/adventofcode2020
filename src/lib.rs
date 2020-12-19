#![allow(dead_code)]
#![allow(clippy::ptr_arg)]

use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

pub mod grid;

macro_rules! advent{
    ($($mod:ident),*) => {
         $(pub mod $mod;)*
    }
}

advent!(
    day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14, day15,
    day16, day17, day18, day19
);

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

pub trait GroupBlankLine<I, S> {
    fn group_by_blanks<C, R, Q>(self, group_parser: C) -> Q
    where
        C: Fn(&mut (dyn Iterator<Item = S>)) -> R,
        Q: FromIterator<R>;
}

impl<I, S> GroupBlankLine<I, S> for I
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    fn group_by_blanks<C, R, Q>(self, group_parser: C) -> Q
    where
        C: Fn(&mut (dyn Iterator<Item = S>)) -> R,
        Q: FromIterator<R>,
    {
        self.group_by(|line| line.as_ref().is_empty())
            .into_iter()
            .filter(|(empty, _)| !empty)
            .map(|(_, mut group)| group_parser(&mut group))
            .collect()
    }
}
