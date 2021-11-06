#[derive(Debug, PartialEq)]
pub enum Instruction {
    Cls,
    Ret,
    Jp(u16),
    Call(u16),
    Se(u8, u8),
    Sne(u8, u8),
    SeXY(u8, u8),
    LdX(u8, u8),
    AddX(u8, u8),
    LdXY(u8, u8),
    Or(u8, u8),
    And(u8, u8),
    Xor(u8, u8),
    Add(u8, u8),
    Sub(u8, u8),
    Shr(u8),
    Subn(u8, u8),
    Shl(u8),
    SneXY(u8, u8),
    LdI(u16),
    JpV0(u16),
    Rnd(u8, u8),
    Drw(u8, u8, u8),
    Skp(u8),
    Sknp(u8),
    LdVxDt(u8),
    LdK(u8),
    LdDtVx(u8),
    LdSt(u8),
    AddI(u8),
    LdF(u8),
    LdB(u8),
    LdIx(u8),
    LdVxI(u8),
}

impl Instruction {
    pub fn from_u16(opcode: u16) -> Option<Instruction> {
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => Some(Instruction::Cls),
                0x00EE => Some(Instruction::Ret),
                _ => None,
            },
            0x1000 => Some(Instruction::Jp(nnn)),
            0x2000 => Some(Instruction::Call(nnn)),
            0x3000 => Some(Instruction::Se(vx, kk)),
            0x4000 => Some(Instruction::Sne(vx, kk)),
            0x5000 => Some(Instruction::SeXY(vx, vy)),
            0x6000 => Some(Instruction::LdX(vx, kk)),
            0x7000 => Some(Instruction::AddX(vx, kk)),
            0x8000 => match opcode & 0x000F {
                0x0000 => Some(Instruction::LdXY(vx, vy)),
                0x0001 => Some(Instruction::Or(vx, vy)),
                0x0002 => Some(Instruction::And(vx, vy)),
                0x0003 => Some(Instruction::Xor(vx, vy)),
                0x0004 => Some(Instruction::Add(vx, vy)),
                0x0005 => Some(Instruction::Sub(vx, vy)),
                0x0006 => Some(Instruction::Shr(vx)),
                0x0007 => Some(Instruction::Subn(vx, vy)),
                0x000E => Some(Instruction::Shl(vx)),
                _ => None,
            },
            0x9000 => Some(Instruction::SneXY(vx, vy)),
            0xA000 => Some(Instruction::LdI(nnn)),
            0xB000 => Some(Instruction::JpV0(nnn)),
            0xC000 => Some(Instruction::Rnd(vx, kk)),
            0xD000 => Some(Instruction::Drw(vx, vy, (opcode & 0x000F) as u8)),
            0xE000 => match opcode & 0x00FF {
                0x009E => Some(Instruction::Skp(vx)),
                0x00A1 => Some(Instruction::Sknp(vx)),
                _ => None,
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => Some(Instruction::LdVxDt(vx)),
                0x000A => Some(Instruction::LdK(vx)),
                0x0015 => Some(Instruction::LdDtVx(vx)),
                0x0018 => Some(Instruction::LdSt(vx)),
                0x001E => Some(Instruction::AddI(vx)),
                0x0029 => Some(Instruction::LdF(vx)),
                0x0033 => Some(Instruction::LdB(vx)),
                0x0055 => Some(Instruction::LdIx(vx)),
                0x0065 => Some(Instruction::LdVxI(vx)),
                _ => None,
            },
            _ => None,
        }
    }
}
