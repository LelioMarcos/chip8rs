#[derive(Debug, PartialEq)]
#[repr(u16)]
pub enum Instrs {
    Cls = 0x00E0,
    Ret = 0x00EE,
}

pub struct OpCode {
    pub instr: Instrs,
    pub x: usize,
    pub y: usize,
    pub nnn: usize
}

impl OpCode {
    pub fn new(instr4: u16) -> OpCode {
        let instr: Instrs = unsafe {::std::mem::transmute(instr4)};
        OpCode { instr , x: 0, y: 0, nnn: 0}
    }
}