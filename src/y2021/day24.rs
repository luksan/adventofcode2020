use either::Either;
use itertools::Itertools;
use std::borrow::Cow;

use std::fmt::{Display, Formatter};
use std::io::Write;
use std::num::NonZeroI32;
use std::str::FromStr;

type Input = Vec<Instruction>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Instruction {
    s.as_ref().parse::<Instruction>().unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Reg {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

impl Reg {
    fn from_str(r: &str) -> Option<Self> {
        Some(match r.as_bytes()[0] {
            b'w' => Self::W,
            b'x' => Self::X,
            b'y' => Self::Y,
            b'z' => Self::Z,
            _ => return None,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OpCode {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
    Inp,
}

impl OpCode {
    fn from_str(op: &str) -> Self {
        match op {
            "add" => Self::Add,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "mod" => Self::Mod,
            "eql" => Self::Eql,
            "inp" => Self::Inp,
            _ => panic!("Bad op code {}", op),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    op: OpCode,
    a: Reg,
    b: Either<Reg, i32>,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, args) = s.splitn(2, ' ').collect_tuple().unwrap();

        let (a, b) = if op == "inp" {
            (Reg::from_str(args).unwrap(), Either::Right(0))
        } else {
            let (a, b) = args.splitn(2, ' ').collect_tuple().unwrap();
            (
                Reg::from_str(a).unwrap(),
                Reg::from_str(b)
                    .map(Either::Left)
                    .or_else(|| Some(Either::Right(i32::from_str(b).ok().unwrap())))
                    .unwrap(),
            )
        };
        Ok(Instruction {
            op: OpCode::from_str(op),
            a,
            b,
        })
    }
}

struct Alu<'a> {
    program: &'a [Instruction],
    regs: [i64; 4],
}

impl<'a> Alu<'a> {
    fn run_with_input(&mut self, mut input: impl Iterator<Item = i32>) {
        self.regs = [0; 4];
        for instr in self.program.iter() {
            let b = match instr.b {
                Either::Left(reg) => self.regs[reg as usize],
                Either::Right(imm) => imm as i64,
            };
            let a = &mut self.regs[instr.a as usize];
            match instr.op {
                OpCode::Add => *a += b,
                OpCode::Mul => *a *= b,
                OpCode::Div => *a /= b,
                OpCode::Mod => *a %= b,
                OpCode::Eql => *a = (*a == b) as i64,
                OpCode::Inp => *a = input.next().unwrap() as i64,
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Digit(NonZeroI32);

impl Default for Digit {
    fn default() -> Self {
        unsafe { Self(NonZeroI32::new_unchecked(9)) }
    }
}

impl Digit {
    fn decrement(&mut self) -> bool {
        if self.0.get() == 1 {
            unsafe { self.0 = NonZeroI32::new_unchecked(9) }
            return true;
        } else {
            unsafe { self.0 = NonZeroI32::new_unchecked(self.0.get() - 1) }
        }
        false
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct ModelNumber([Digit; 14]);

impl ModelNumber {
    fn new() -> Self {
        Self([Digit::default(); 14])
    }

    fn decrement(&mut self) {
        for d in self.0.iter_mut().rev() {
            if !d.decrement() {
                break;
            }
        }
    }
}

impl Display for ModelNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for d in &self.0 {
            write!(f, "{}", d.0.get())?;
        }
        Ok(())
    }
}

fn part1(program: &Input) -> usize {
    let mut alu = Alu {
        program,
        regs: [0; 4],
    };
    let mut model = ModelNumber::default();
    loop {
        let iter = model.0.iter().map(|d| d.0.get());
        let z = super::alu::alu(iter);
        if z == 0 {
            break;
        }
        model.decrement();
        // println!("{}", model);
    }

    println!("Best model number {}", model);
    0
}

fn part2(_input: &Input) -> usize {
    0
}

#[test]
fn create_func() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    let mut out = std::fs::File::create("./src/y2021/alu_new.rs").unwrap();
    writeln!(
        out,
        "pub fn alu(mut input: impl Iterator<Item=i32>) -> i64 {{"
    )
    .unwrap();
    writeln!(out, "let mut w: i64= 0;").unwrap();
    writeln!(out, "let mut x: i64= 0;").unwrap();
    writeln!(out, "let mut y: i64= 0;").unwrap();
    writeln!(out, "let mut z: i64= 0;").unwrap();
    fn reg_str(reg: Reg) -> &'static str {
        match reg {
            Reg::W => "w",
            Reg::X => "x",
            Reg::Y => "y",
            Reg::Z => "z",
        }
    }
    for instr in d.iter() {
        let b = match instr.b {
            Either::Left(reg) => reg_str(reg).into(),
            Either::Right(imm) => Cow::Owned(imm.to_string()),
        };
        let a = reg_str(instr.a);
        match instr.op {
            OpCode::Add => {
                if b != "0" {
                    writeln!(out, "{} += {};", a, b)
                } else {
                    continue;
                }
            }
            OpCode::Mul => {
                if b == "0" {
                    writeln!(out, "\n{} = 0;", a)
                } else {
                    writeln!(out, "{} *= {};", a, b)
                }
            }
            OpCode::Div => {
                if b != "1" {
                    writeln!(out, "{} /= {};", a, b)
                } else {
                    continue;
                }
            }
            OpCode::Mod => writeln!(out, "{} %= {};", a, b),
            OpCode::Eql => writeln!(out, "{} = ({} == {}) as i64;", a, a, b),
            OpCode::Inp => writeln!(out, "\n{} = input.next().unwrap() as i64;", a),
        }
        .unwrap();
    }

    writeln!(out, "z }}").unwrap();
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 1);
    // assert_eq!(part2(&d), 1);
}

// #[test]
fn test_data() {
    let data = // Example data
"";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 1);
    // assert_eq!(part2(&d), 1);
}
