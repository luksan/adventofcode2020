#![allow(dead_code)]

use adventofcode2020::DayOfAdvent;

macro_rules! advent{
    ($($mod:ident),*) => {
         $(mod $mod;)*

         fn day_list() -> Vec<Box<fn() -> Box<dyn DayOfAdvent + 'static>>> {
            let mut days:  Vec<Box<fn() -> Box<dyn DayOfAdvent + 'static>>> = Vec::new();
            $(days.push(Box::new($mod::solve));)*
            days
        }
    }
}

advent!(day1, day2, day3, day4, day5);

fn print_day<T: AsRef<dyn DayOfAdvent>>(day: T) {
    let day = day.as_ref();
    println!("** Day {} **", day.day());
    for line in day.result_strings() {
        println!("{}", line);
    }
}

fn main() {
    let days = day_list();

    print_day(days.last().unwrap()());
}
