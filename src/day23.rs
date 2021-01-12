use arrayvec::ArrayVec;
use itertools::Itertools;
use std::cell::Cell;
use std::cmp::Ordering;
use std::convert::TryInto;

type Cup = u32;
type Cups = Vec<Cup>;

fn parse(s: &str) -> Circle {
    Circle {
        cups: s.chars().map(|c| c.to_digit(10).unwrap() as Cup).collect(),
        current: 0,
    }
}

type Ptr = u32;
type Link = Cell<Ptr>;

struct CupPtr<'a> {
    links: &'a Vec<Link>,
    ptr: Ptr,
}

impl CupPtr<'_> {
    fn new(links: &Vec<Link>, node: Ptr) -> CupPtr {
        CupPtr { links, ptr: node }
    }

    fn next(&self) -> CupPtr {
        CupPtr {
            links: self.links,
            ptr: self.links[self.ptr as usize].get(),
        }
    }

    fn set_next(&self, next_cup: Ptr) {
        self.links[self.ptr as usize].set(next_cup)
    }
}

struct LinkedCircle {
    cups: [Cup; 9],
    links: Vec<Cell<Ptr>>,
    current: Ptr,
}

impl LinkedCircle {
    const CUP_CNT: usize = 1_000_000;

    fn from_circle(circle: &Circle) -> LinkedCircle {
        let mut links: Vec<_> = (1..=Self::CUP_CNT).map(|l| Cell::new(l as u32)).collect();
        links.last_mut().unwrap().set(0);

        LinkedCircle {
            cups: circle.cups.as_slice().try_into().unwrap(),
            links,
            current: circle.current as Ptr,
        }
    }

    fn new_ptr(&self, node: Ptr) -> CupPtr {
        CupPtr::new(&self.links, node)
    }

    fn read_ptr(&self, ptr: Ptr) -> Cup {
        if ptr < self.cups.len() as Ptr {
            self.cups[ptr as usize]
        } else {
            (ptr + 1) as Cup
        }
    }

    fn find_cup(&self, cup: Cup) -> Ptr {
        if cup <= 9 {
            for ptr in 0..9 {
                if self.cups[ptr] == cup {
                    return ptr as Ptr;
                }
            }
            unreachable!("Mhmm")
        }
        (cup - 1) as Ptr
    }

    fn single_move(&mut self) {
        let current = self.new_ptr(self.current);
        let curr_label = self.read_ptr(current.ptr);
        let removed = self.take_three(&current);

        let mut dest_label = curr_label - 1;
        let dest: Ptr = loop {
            if dest_label == 0 {
                dest_label = Self::CUP_CNT as Cup;
            }
            let ptr = self.find_cup(dest_label);
            if !removed.contains(&ptr) {
                break ptr;
            }
            dest_label -= 1;
        };

        self.place_three(dest, removed);
        self.current = current.next().ptr;
    }

    fn take_three(&self, right_of: &CupPtr) -> [Ptr; 3] {
        let first = right_of.next();
        let n2 = first.next();
        let third = n2.next();
        let fourth = third.next();
        right_of.set_next(fourth.ptr);
        [first.ptr, n2.ptr, third.ptr]
    }

    fn place_three(&self, right_of: Ptr, cups: [Ptr; 3]) {
        let right_of = self.new_ptr(right_of);
        let third = self.new_ptr(cups[2]);
        let fourth = right_of.next();
        right_of.set_next(cups[0]);
        third.set_next(fourth.ptr);
    }
}

#[derive(Clone, Debug)]
struct Circle {
    cups: ArrayVec<[Cup; 9]>,
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

    let cup1 = circle.new_ptr(circle.find_cup(1));
    let n1 = cup1.next();
    let n2 = n1.next();

    circle.read_ptr(n1.ptr) as usize * circle.read_ptr(n2.ptr) as usize
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
