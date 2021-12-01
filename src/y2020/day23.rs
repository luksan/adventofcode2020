use arrayvec::ArrayVec;
use itertools::Itertools;
use std::cell::Cell;
use std::cmp::Ordering;
use std::convert::TryInto;

fn parse(s: &str) -> Circle {
    Circle {
        cups: s.chars().map(|c| c.to_digit(10).unwrap() as Cup).collect(),
        current: 0,
    }
}

type Cup = u32;
type Link = Cell<Cup>;

struct CupPtr<'a> {
    links: &'a Vec<Link>,
    cup: Cup,
}

impl<'a> CupPtr<'a> {
    fn new(links: &Vec<Link>, node: Cup) -> CupPtr {
        CupPtr { links, cup: node }
    }

    fn next(&self) -> CupPtr<'a> {
        CupPtr {
            links: &*self.links,
            cup: self.links[self.cup as usize].get(),
        }
    }

    fn set_next(&self, next_cup: Cup) {
        self.links[self.cup as usize].set(next_cup)
    }
}

struct LinkedCircle {
    links: Vec<Cell<Cup>>,
    current: Cup,
}

impl LinkedCircle {
    const CUP_CNT: u32 = 1_000_000;

    fn from_circle(circle: &Circle) -> LinkedCircle {
        let links = (0..=Self::CUP_CNT).map(|l| Cell::new(l + 1)).collect();

        let mut ptr = CupPtr::new(&links, (links.len() - 1) as Cup);
        for cup in &circle.cups {
            ptr.set_next(*cup);
            ptr = ptr.next();
        }
        ptr.set_next(10);

        LinkedCircle {
            links,
            current: circle.cups[0],
        }
    }

    fn new_ptr(&self, node: Cup) -> CupPtr {
        CupPtr::new(&self.links, node)
    }

    fn single_move(&mut self) {
        let current = self.new_ptr(self.current);
        let removed = self.take_three(&current);

        let mut maybe_dest = self.current - 1;
        let dest = loop {
            if maybe_dest == 0 {
                maybe_dest = Self::CUP_CNT as Cup;
            }
            if !removed.contains(&maybe_dest) {
                break maybe_dest;
            }
            maybe_dest -= 1;
        };

        self.place_three(dest, removed);
        self.current = current.next().cup;
    }

    fn take_three(&self, right_of: &CupPtr) -> [Cup; 3] {
        let first = right_of.next();
        let n2 = first.next();
        let third = n2.next();
        let fourth = third.next();
        right_of.set_next(fourth.cup);
        [first.cup, n2.cup, third.cup]
    }

    fn place_three(&self, right_of: Cup, cups: [Cup; 3]) {
        let right_of = self.new_ptr(right_of);
        let third = self.new_ptr(cups[2]);
        let fourth = right_of.next();
        right_of.set_next(cups[0]);
        third.set_next(fourth.cup);
    }
}

#[derive(Clone, Debug)]
pub struct Circle {
    cups: ArrayVec<Cup, 9>,
    current: usize,
}

impl Circle {
    fn single_move(&mut self) {
        match self.current.cmp(&5) {
            Ordering::Less => self.cups.rotate_right(5 - self.current),
            Ordering::Greater => self.cups.rotate_left(5 - self.current),
            Ordering::Equal => {}
        }
        self.current = 5;
        let removed: [Cup; 3] = (&self.cups[self.current + 1..]).try_into().unwrap();
        self.cups.truncate(self.current + 1);

        let dst = self.dest_cup();
        let shift_right = 5 - dst;
        self.cups.rotate_right(shift_right); // Place the destination cup at the end of the vector
        self.cups.try_extend_from_slice(&removed).unwrap();

        self.current += 3 + shift_right;
        if self.current >= 8 {
            self.current -= 8;
        }
    }

    fn dest_cup(&self) -> usize {
        let mut dst_label: Cup = self.cups[self.current] - 1;
        loop {
            if dst_label == 0 {
                dst_label = 9;
            }
            if let Some((pos, _)) = self.cups.iter().find_position(|&&c| c == dst_label) {
                return pos;
            }
            dst_label -= 1;
        }
    }
}
fn part1(circle: &Circle) -> String {
    let mut circle = circle.clone();
    for _ in 0..100 {
        circle.single_move();
    }
    circle
        .cups
        .iter()
        .cycle()
        .skip_while(|&&c| c != 1)
        .skip(1)
        .take_while(|&&c| c != 1)
        .join("")
}

fn part2(circle: &Circle) -> usize {
    let mut circle = LinkedCircle::from_circle(circle);

    for _ in 0..10_000_000 {
        circle.single_move();
    }

    let n1 = circle.new_ptr(1).next();
    let n2 = n1.next();
    n1.cup as usize * n2.cup as usize
}

pub fn bench_input() -> Circle {
    parse("219748365")
}

pub fn bench(input: &Circle) {
    part2(&input);
}

#[test]
fn real_data() {
    let d = parse("219748365");
    assert_eq!(part1(&d), "35827964");
    assert_eq!(part2(&d), 5403610688);
}

#[test]
fn test_data() {
    let d = parse("389125467");
    assert_eq!(part1(&d), "67384529");
    assert_eq!(part2(&d), 149245887792);
}
