use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait DayOfAdvent: Sync {
    fn day(&self) -> u32;

    fn result_strings(&self) -> Vec<String>;
}

pub fn load_input<C, R, P>(path: P, conv: C) -> Vec<R>
where
    P: AsRef<Path>,
    C: FnMut(String) -> R,
{
    let file = File::open(path.as_ref()).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).map(conv).collect()
}
