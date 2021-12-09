#![allow(dead_code)]
#![allow(clippy::ptr_arg)]

use itertools::Itertools;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

pub mod grid;
pub mod y2020;
pub mod y2021;

macro_rules! advent{
    ($($mod:ident),*) => {
         $(pub mod $mod;)*
    }
}

pub(crate) use advent;

pub fn module_data_file(mod_path: &str) -> String {
    let (day, year, _root) = mod_path.rsplitn(3, "::").collect_tuple().unwrap();
    let year = &year[1..]; // remove 'y'
    format!("./data/{}/{}.txt", year, day)
}

#[allow(unused)]
macro_rules! data_file {
    () => {
        crate::module_data_file(module_path!())
    };
}
#[allow(unused)]
pub(crate) use data_file;

pub fn buf_reader<P>(path: P) -> BufReader<File>
where
    P: AsRef<Path> + Debug,
{
    let file =
        File::open(path.as_ref()).unwrap_or_else(|_| panic!("Failed to open file {:?}", path));
    io::BufReader::new(file)
}

pub fn load_strings<P>(path: P) -> Box<dyn Iterator<Item = String>>
where
    P: AsRef<Path> + Debug,
{
    Box::new(buf_reader(path).lines().map(|l| l.unwrap()))
}

pub fn load_input<C, R, P, Q>(path: P, conv: C) -> Q
where
    P: AsRef<Path> + Debug,
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
