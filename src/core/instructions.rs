#[derive(Debug, PartialEq)]
pub enum Instrs {
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

impl Instrs {
    pub fn from_u16(opcode: u16) -> Option<Instrs> {
        let vx = (opcode & 0x0F00 >> 8) as u8;
        let vy = (opcode & 0x00F0 >> 4) as u8;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => Some(Instrs::Cls),
                0x00EE => Some(Instrs::Ret),
                _ => None,
            },
            0x1000 => Some(Instrs::Jp(nnn)),
            0x2000 => Some(Instrs::Call(nnn)),
            0x3000 => Some(Instrs::Se(vx, kk)),
            0x4000 => Some(Instrs::Sne(vx, kk)),
            0x5000 => Some(Instrs::SeXY(vx, vy)),
            0x6000 => Some(Instrs::LdX(vx, kk)),
            0x7000 => Some(Instrs::AddX(vx, kk)),
            0x8000 => match opcode & 0x000F {
                0x0000 => Some(Instrs::LdXY(vx, vy)),
                0x0001 => Some(Instrs::Or(vx, vy)),
                0x0002 => Some(Instrs::And(vx, vy)),
                0x0003 => Some(Instrs::Xor(vx, vy)),
                0x0004 => Some(Instrs::Add(vx, vy)),
                0x0005 => Some(Instrs::Sub(vx, vy)),
                0x0006 => Some(Instrs::Shr(vx)),
                0x0007 => Some(Instrs::Subn(vx, vy)),
                0x000E => Some(Instrs::Shl(vx)),
                _ => None,
            },
            0x9000 => Some(Instrs::SneXY(vx, vy)),
            0xA000 => Some(Instrs::LdI(nnn)),
            0xB000 => Some(Instrs::JpV0(nnn)),
            0xC000 => Some(Instrs::Rnd(vx, kk)),
            0xD000 => Some(Instrs::Drw(vx, vy, (opcode & 0x000F) as u8)),
            0xE000 => match opcode & 0x00FF {
                0x009E => Some(Instrs::Skp(vx)),
                0x00A1 => Some(Instrs::Sknp(vx)),
                _ => None,
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => Some(Instrs::LdVxDt(vx)),
                0x000A => Some(Instrs::LdK(vx)),
                0x0015 => Some(Instrs::LdDtVx(vx)),
                0x0018 => Some(Instrs::LdSt(vx)),
                0x001E => Some(Instrs::AddI(vx)),
                0x0029 => Some(Instrs::LdF(vx)),
                0x0033 => Some(Instrs::LdB(vx)),
                0x0055 => Some(Instrs::LdIx(vx)),
                0x0065 => Some(Instrs::LdVxI(vx)),
                _ => None,
            },
            _ => None,
        }
    }
}
