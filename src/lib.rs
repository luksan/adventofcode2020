#![allow(dead_code)]
#![allow(clippy::ptr_arg)]

use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

pub mod grid;
mod y2020;

macro_rules! advent{
    ($($mod:ident),*) => {
         $(pub mod $mod;)*
    }
}

pub(crate) use advent;

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
