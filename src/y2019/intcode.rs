use num_enum::TryFromPrimitive;

use std::convert::TryInto;

#[derive(Copy, Clone, Debug, TryFromPrimitive)]
#[repr(usize)]
enum OpCode {
    Add = 1,
    Mul = 2,
    Halt = 99,
}

#[derive(Clone, Debug)]
pub struct Intcode {
    mem: Vec<usize>,
}

impl Intcode {
    pub fn load_program(prog: &str) -> Self {
        let mem = prog.split(',').map(|n| n.parse().unwrap()).collect();
        Self { mem }
    }

    pub fn run(&mut self) {
        let mut ip = 0;
        let m = &mut self.mem;
        macro_rules! aritm3 {
            ($op:tt) => {{
                let res = m[m[ip + 1]] $op m[m[ip+2]];
                let target_addr = m[ip+3];
                m[target_addr] = res;
                ip +=4;
            }}
        }
        loop {
            let op: OpCode = m[ip].try_into().unwrap();
            match op {
                OpCode::Add => aritm3!(+),
                OpCode::Mul => aritm3!(*),
                OpCode::Halt => break,
            }
        }
    }

    pub fn peek(&self, addr: usize) -> usize {
        self.mem[addr]
    }

    pub fn poke(&mut self, addr: usize, val: usize) -> usize {
        std::mem::replace(&mut self.mem[addr], val)
    }
}
