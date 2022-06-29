#![feature(ptr_const_cast)]

mod instructions {
    pub const NOP: u16 = 0;
    pub const HLT: u16 = 1;
    pub const ADD: u16 = 2;
    pub const SUB: u16 = 3;
    pub const DIV: u16 = 4;
    pub const MUL: u16 = 5;
    pub const STO: u16 = 6;
    pub const LOD: u16 = 7;
    pub const COM: u16 = 8;
    pub const JE: u16 = 9;
    pub const JNE: u16 = 10;
    pub const JG: u16 = 11;
    pub const JGE: u16 = 12;
    pub const JL: u16 = 13;
    pub const JLE: u16 = 14;
    pub const EXEC: u16 = 15;
}

mod registers {
    pub const RA: u16 = 100;
    pub const RB: u16 = 101;
    pub const RC: u16 = 102;
    pub const RD: u16 = 103;
    pub const RE: u16 = 104;
    pub const RF: u16 = 105;
    pub const RJ: u16 = 106;
    pub const RK: u16 = 107;
}

pub enum Flags {
    E,  // equal
    NE, // not equal
    GE, // greater or equal
    G,  // greater
    LE, // less or equal
    L,  // less
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    NOP,
    HLT,
    PUSH,
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    EXEC,
    Imm(i64),
    Reg(Register),
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Register {
    RA = 0,
    RB,
    RC,
    RD,
    RE,
    RF,
    RJ,
    RK,
}

pub struct Memory<const U: usize> {
    buffer: [Instruction; U],
}

impl<const U: usize> Memory<U> {
    pub fn new() -> Self {
        Self {
            buffer: [Instruction::HLT; U],
        }
    }

    pub fn set(&mut self, index: usize, value: Instruction) -> Result<(), &'static str> {
        if index >= U {
            Err("address is beyond the memory limit")
        } else {
            self.buffer[index] = value;
            Ok(())
        }
    }

    pub fn get(&self, index: usize) -> Option<Instruction> {
        self.buffer.get(index).map(|v| v.clone())
    }
}

pub struct RegisterStruct {
    RA: i64,
    RB: i64,
    RC: i64,
    RD: i64,
    RE: i64,
    RF: i64,
    RJ: i64,
    RK: i64
}

impl RegisterStruct {
    pub fn new() -> Self {
        Self {
            RA: 0,
            RB: 0,
            RC: 0,
            RD: 0,
            RE: 0,
            RF: 0,
            RJ: 0,
            RK: 0
        }
    }
}

pub struct Cpu {
    memory: Memory<1024>,
    registers: RegisterStruct,
    run: bool,
    ip: usize,
}

impl Cpu {
    pub fn new(mem: Memory<1024>) -> Self {
        Self {
            memory: mem,
            registers: RegisterStruct::new(),
            run: true,
            ip: 0,
        }
    }

    pub fn fetch_next(&mut self) -> Option<Instruction> {
        let out = self.memory.get(self.ip);
        self.ip += 1;
        out
    }

    pub fn exec(&mut self, instruction: Instruction) -> Result<(), &'static str> {
        use Instruction::*;
        match instruction {
            NOP => (),
            HLT => self.run = false,
            ADD => {
                let dest = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => reg,
                    _ => return Err("add: ok_ored register in the dest operand"),
                };
                let src0 = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => self.get_register_value(reg),
                    Imm(v) => v,
                    _ => return Err("add: invalid operand"),
                };
                let src1 = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => self.get_register_value(reg),
                    Imm(v) => v,
                    _ => return Err("add: invalid operand"),
                };
                self.register_insert_imm(dest, src0 + src1);
            }
            SUB => {
                let dest = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => reg,
                    _ => return Err("sub: ok_ored register in the dest operand"),
                };
                let src0 = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => self.get_register_value(reg),
                    Imm(v) => v,
                    _ => return Err("sub: invalid operand"),
                };
                let src1 = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => self.get_register_value(reg),
                    Imm(v) => v,
                    _ => return Err("sub: invalid operand"),
                };
                self.register_insert_imm(dest, src0 - src1);
            }
            MUL => {
                let dest = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => reg,
                    _ => return Err("mul: ok_ored register in the dest operand"),
                };
                let src0 = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => self.get_register_value(reg),
                    Imm(v) => v,
                    _ => return Err("mul: invalid operand"),
                };
                let src1 = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => self.get_register_value(reg),
                    Imm(v) => v,
                    _ => return Err("mul: invalid operand"),
                };
                self.register_insert_imm(dest, src0 * src1);
            }
            DIV => {
                let dest = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => reg,
                    _ => return Err("div: ok_ored register in the dest operand"),
                };
                let src0 = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => self.get_register_value(reg),
                    Imm(v) => v,
                    _ => return Err("div: invalid operand"),
                };
                let src1 = match self.fetch_next().ok_or("unok_ored end of input")? {
                    Reg(reg) => self.get_register_value(reg),
                    Imm(v) => v,
                    _ => return Err("div: invalid operand"),
                };
                self.register_insert_imm(dest, src0 / src1);
            }
            LOD => {
                let dest = match self.fetch_next().ok_or("ok")? {
                    Reg(reg) => reg,
                    _ => return Err("lod: dest must be a register")
                };
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    pub fn get_register_value(&self, reg: Register) -> i64 {
        let ptr = std::ptr::addr_of!(self.registers.RA);
        unsafe { ptr.offset(reg as _).read() }
    }

    pub fn get_register_addr(&mut self, reg: Register) -> *mut i64 {
        unsafe {
            let ptr = std::ptr::addr_of!(self.registers.RA).as_mut();
            ptr.offset(reg as _) 
        }

    }

    pub fn register_insert_imm(&mut self, reg: Register, imm: i64) {
        unsafe { self.get_register_addr(reg).write(imm) };
    }

    pub fn fetch_n_exec(&mut self) -> Result<(), &'static str> {
        let n = self.fetch_next().ok_or("unok_ored end of input")?;
        self.exec(n)
    }


    pub fn start(&mut self) -> Result<(), &'static str> {
        while self.run {
            self.fetch_n_exec()?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() -> Result<(), &'static str> {
        let mut mem = Memory::<1024>::new();
        mem.set(0x00, Instruction::ADD)?;
        mem.set(0x01, Instruction::Reg(Register::RA))?;
        mem.set(0x02, Instruction::Imm(1))?;
        mem.set(0x03, Instruction::Imm(2))?;
        mem.set(0x04, Instruction::MUL)?;
        mem.set(0x05, Instruction::Reg(Register::RA))?;
        mem.set(0x06, Instruction::Reg(Register::RA))?;
        mem.set(0x07, Instruction::Imm(2))?;
        let mut cpu = Cpu::new(mem);
        cpu.start()?;
        assert_eq!(cpu.get_register_value(Register::RA), 6);
        Ok(())
    }
}
