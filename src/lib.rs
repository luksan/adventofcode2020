#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::Path;

macro_rules! advent{
    ($($mod:ident),*) => {
         $(mod $mod;)*
    }
}

advent!(day1, day2, day3, day4, day5, day6);

pub fn load_input<C, R, P, Q>(path: P, conv: C) -> Q
where
    P: AsRef<Path>,
    C: FnMut(String) -> R,
    Q: FromIterator<R>,
{
    let file = File::open(path.as_ref()).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).map(conv).collect()
}
