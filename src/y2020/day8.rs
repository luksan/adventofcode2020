use itertools::Itertools;
use std::str::FromStr;

type Instr = (InstructionSet, i32);
type Program = Vec<Instr>;

fn load_input() -> Program {
    crate::load_input("data/2020/day8.txt", parse)
}

fn parse<S: AsRef<str>>(s: S) -> Instr {
    s.as_ref()
        .split_ascii_whitespace()
        .next_tuple()
        .map(|(instr, arg)| (instr.parse().unwrap(), arg.parse().unwrap()))
        .unwrap()
}

#[derive(Debug, Copy, Clone)]
enum InstructionSet {
    ACC,
    HCF,
    JMP,
    NOP,
}

impl FromStr for InstructionSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InstructionSet::*;
        Ok(match s {
            "acc" => ACC,
            "jmp" => JMP,
            "nop" => NOP,
            _ => HCF,
        })
    }
}

struct Cpu {
    prog: Program,
    acc: i32,
    ip: usize,
}

impl Cpu {
    pub fn new(prog: Program) -> Self {
        Self {
            prog,
            acc: 0,
            ip: 0,
        }
    }

    pub fn run(&mut self) -> Result<i32, i32> {
        use InstructionSet::*;
        while let Some(instr) = self.prog.get(self.ip) {
            let mut ip_incr = 1;
            match instr {
                (ACC, arg) => self.acc += arg,
                (JMP, arg) => ip_incr = *arg,
                (NOP, _) => {}
                (HCF, _) => return Err(self.acc),
            }
            self.prog[self.ip] = (HCF, 0);
            self.ip = (self.ip as i32 + ip_incr) as usize;
        }
        Ok(self.acc)
    }
}
fn part1(prog: &Program) -> i32 {
    let mut cpu = Cpu::new(prog.clone());
    cpu.run().unwrap_err()
}

fn part2(prog: &Program) -> i32 {
    use InstructionSet::*;
    for i in 0..prog.len() {
        if matches!(prog[i], (ACC, _)) {
            continue;
        }
        let mut patched = prog.clone();
        match patched[i] {
            (JMP, arg) => patched[i] = (NOP, arg),
            (NOP, arg) => patched[i] = (JMP, arg),
            _ => unreachable!("No more instruction types."),
        }
        if let Ok(acc) = Cpu::new(patched).run() {
            return acc;
        }
    }
    0
}

#[test]
fn real_data() {
    let d = load_input();
    assert_eq!(d.len(), 628);
    assert_eq!(part1(&d), 1766);
    assert_eq!(part2(&d), 1639);
}

#[test]
fn test_data() {
    let prog = // Example program
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let d: Program = prog.lines().map(parse).collect();
    assert_eq!(part1(&d), 5);
    assert_eq!(part2(&d), 8);
}
