use std::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Debug)]
enum OpCode {
    Add(OpMode<3>),
    Mul(OpMode<3>),
    Input(OpMode<1>),
    Output(OpMode<1>),
    JNZ(OpMode<2>),
    JZ(OpMode<2>),
    Less(OpMode<3>),
    Eq(OpMode<3>),
    AdjRelBase(OpMode<1>),
    Halt(OpMode<0>),
}

impl TryFrom<MemCell> for OpCode {
    type Error = ();

    fn try_from(value: MemCell) -> Result<Self, Self::Error> {
        let op = value % 100;
        use OpCode::*;
        Ok(match op {
            1 => Add(value.into()),
            2 => Mul(value.into()),
            3 => Input(value.into()),
            4 => Output(value.into()),
            5 => JNZ(value.into()),
            6 => JZ(value.into()),
            7 => Less(value.into()),
            8 => Eq(value.into()),
            9 => AdjRelBase(value.into()),
            99 => Halt(value.into()),

            _ => panic!("Unknown opcode"),
        })
    }
}

#[derive(Copy, Clone, Debug)]
struct OpMode<const N: usize>([Mode; N]);

impl<const N: usize> OpMode<N> {
    fn new(val: MemCell) -> Self {
        let mut m = [Mode::Pos; N];
        let mut c = val / 100; // remove op code
        for i in 0..N {
            let mode = match c % 10 {
                0 => Mode::Pos,
                1 => Mode::Imm,
                2 => Mode::Rel,
                _ => panic!("Unhandled op mode."),
            };
            m[i] = mode;
            c /= 10;
        }
        Self(m)
    }
}

impl<const N: usize> From<MemCell> for OpMode<N> {
    fn from(m: MemCell) -> Self {
        Self::new(m)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Mode {
    Imm,
    Pos,
    Rel,
}

pub type MemCell = isize;

#[derive(Clone, Debug)]
pub struct Intcode {
    mem: Vec<MemCell>,
    ip: usize,
    rel_base: isize,
}

pub enum PauseCause<'a> {
    Halt,
    Input(&'a mut MemCell),
    Output(MemCell),
}

impl Intcode {
    pub fn load_program(prog: &str) -> Self {
        let mem = prog.split(',').map(|n| n.parse().unwrap()).collect();
        Self {
            mem,
            ip: 0,
            rel_base: 0,
        }
    }

    fn op_addr_and_ip<const N: usize>(&mut self, modes: OpMode<N>) -> [usize; N] {
        let mut r = [0; N];
        for n in 0..N {
            let imm_addr = self.ip + n + 1;
            r[n] = match modes.0[n] {
                Mode::Imm => imm_addr as isize,
                Mode::Pos => self.mem[imm_addr],
                Mode::Rel => self.rel_base + self.mem[imm_addr],
            } as usize;
        }
        if N > 0 {
            let max_addr = *r.iter().max().unwrap();
            if max_addr >= self.mem.len() {
                if self.mem.len() >= 10_000_000 {
                    panic!("Intcode more than 10 M memory cells.")
                }
                self.mem.resize(max_addr * 2, 0);
            }
        }
        self.ip += N + 1;
        r
    }

    pub fn run_until_end(&mut self, input: &[MemCell]) -> Vec<MemCell> {
        let mut output = vec![];
        let mut input = input.iter();
        loop {
            match self.run() {
                PauseCause::Halt => break,
                PauseCause::Input(mem_ref) => {
                    *mem_ref = *input.next().unwrap();
                }
                PauseCause::Output(out) => {
                    output.push(out);
                }
            }
        }
        output
    }

    pub fn run(&mut self) -> PauseCause {
        loop {
            let op: OpCode = self.mem[self.ip].try_into().unwrap();
            macro_rules! aritm3 {
            ($op:tt, $mode:ident) => {{
                let [a,b,c] = self.op_addr_and_ip($mode);
                self.mem[c] = (self.mem[a] $op self.mem[b]) as MemCell;
            }}
        }
            match op {
                OpCode::Add(mode) => aritm3!(+, mode),
                OpCode::Mul(mode) => aritm3!(*, mode),
                OpCode::Input(mode) => {
                    let [target] = self.op_addr_and_ip(mode);
                    return PauseCause::Input(&mut self.mem[target]);
                }
                OpCode::Output(mode) => {
                    let [target] = self.op_addr_and_ip(mode);
                    return PauseCause::Output(self.mem[target]);
                }
                OpCode::JNZ(mode) => {
                    let [c, t] = self.op_addr_and_ip(mode);
                    if self.mem[c] != 0 {
                        self.ip = self.mem[t] as usize;
                    }
                }
                OpCode::JZ(mode) => {
                    let [c, t] = self.op_addr_and_ip(mode);
                    if self.mem[c] == 0 {
                        self.ip = self.mem[t] as usize;
                    }
                }
                OpCode::Less(mode) => aritm3!(<, mode),
                OpCode::Eq(mode) => aritm3!(==, mode),
                OpCode::AdjRelBase(mode) => {
                    let [o] = self.op_addr_and_ip(mode);
                    self.rel_base += self.mem[o];
                }
                OpCode::Halt(_mode) => return PauseCause::Halt,
            }
        }
    }

    pub fn peek(&self, addr: usize) -> MemCell {
        self.mem[addr]
    }

    pub fn poke(&mut self, addr: usize, val: MemCell) -> MemCell {
        std::mem::replace(&mut self.mem[addr], val)
    }
}
