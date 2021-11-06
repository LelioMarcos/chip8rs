use rand::Rng;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const ENTRY_POINT: u16 = 0x200;

pub mod instructions;
mod video;

pub use instructions::Instruction;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Debug)]
pub struct Chip8 {
    pub memory: Vec<u8>, // [u8; 4096],
    pub v: Vec<u8>,
    pub i: u16,
    pub pc: u16,
    pub gfx: Vec<u8>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: Vec<u16>,
    pub sp: u16,
    pub key: Vec<u8>,
    pub draw_flag: u8,
    pub key_flag: bool,
    pub pause: bool,
}

impl Chip8 {
    pub fn init() -> Chip8 {
        Chip8 {
            memory: vec![0; 4096],
            v: vec![0; 16],
            i: 0x0000,
            pc: ENTRY_POINT,
            gfx: vec![0; WIDTH * HEIGHT],
            delay_timer: 0x00,
            sound_timer: 0x00,
            stack: vec![0; 16],
            sp: 0x0000,
            key: vec![0; 16],
            draw_flag: 0x01,
            key_flag: false,
            pause: false,
        }
    }

    fn load_game(&mut self, game_buffer: &[u8]) {
        for i in (0..80).into_iter() {
            self.memory[i] = FONT_SET[i];
        }

        for i in (0..game_buffer.len()).into_iter() {
            self.memory[i + ENTRY_POINT as usize] = game_buffer[i];
        }
    }

    fn fetch_and_execute(&mut self, opcode: u16) {
        if let Some(instr) = Instruction::from_u16(opcode) {
            match instr {
                Instruction::Cls => self.gfx = vec![0; WIDTH * HEIGHT],
                Instruction::Ret => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize] - 2;
                }
                Instruction::Jp(addr) => self.pc = addr - 2,
                Instruction::Call(addr) => {
                    self.stack[self.sp as usize] = self.pc;
                    self.sp += 1;
                    self.pc = addr - 2;
                }
                Instruction::Se(x, byte) => {
                    if self.v[x as usize] == byte {
                        self.pc += 2;
                    }
                }
                Instruction::Sne(x, byte) => {
                    if self.v[x as usize] != byte {
                        self.pc += 2;
                    }
                }
                Instruction::SeXY(x, y) => {
                    if self.v[x as usize] == self.v[y as usize] {
                        self.pc += 2;
                    }
                }
                Instruction::LdX(x, byte) => self.v[x as usize] = byte,
                Instruction::AddX(x, byte) => self.v[x as usize] = self.v[x as usize].wrapping_add(byte),
                Instruction::LdXY(x, y) => self.v[x as usize] = self.v[y as usize],
                Instruction::Or(x, y) => self.v[x as usize] |= self.v[y as usize],
                Instruction::And(x, y) => self.v[x as usize] &= self.v[y as usize],
                Instruction::Xor(x, y) => self.v[x as usize] ^= self.v[y as usize],
                Instruction::Add(x, y) => {
                    let (result, overflow) = self.v[x as usize].overflowing_add(self.v[y as usize]);
                    self.v[x as usize] = result;
                    self.v[0xF] = if overflow { 1 } else { 0 };
                }
                Instruction::Sub(x, y) => {
                    let (result, overflow) = self.v[x as usize].overflowing_sub(self.v[y as usize]);
                    self.v[x as usize] = result;
                    self.v[0xF] = if overflow { 0 } else { 1 };
                }
                Instruction::Shr(x) => {
                    self.v[0xF] = self.v[x as usize] & 0x01;
                    self.v[x as usize] >>= 1;
                }
                Instruction::Subn(x, y) => {
                    let (result, overflow) = self.v[y as usize].overflowing_sub(self.v[x as usize]);
                    self.v[x as usize] = result;
                    self.v[0xF] = if overflow { 0 } else { 1 };
                }
                Instruction::Shl(x) => {
                    self.v[0xF] = (self.v[x as usize] & 0x80) >> 7;
                    self.v[x as usize] <<= 1;
                }
                Instruction::SneXY(x, y) => {
                    if self.v[x as usize] != self.v[y as usize] {
                        self.pc += 2;
                    }
                }
                Instruction::LdI(addr) => self.i = addr,
                Instruction::JpV0(addr) => self.pc = addr + self.v[0] as u16,
                Instruction::Rnd(x, byte) => {
                    let mut r = rand::thread_rng();
                    self.v[x as usize] = r.gen_range(0..=255) & byte as u8;
                }
                Instruction::Drw(x, y, n) => {
                    let px = self.v[x as usize] as u16;
                    let py = self.v[y as usize] as u16;
                    let height = n;
                    let mut pixel: u8;

                    self.v[0xF] = 0;

                    for yline in 0..height {
                        pixel = self.memory[(self.i + yline as u16) as usize];

                        for xline in 0..8 {
                            if (pixel & (0x80 >> xline)) != 0 {
                                if (self.gfx[(px + xline + ((py + yline as u16) * 64)) as usize])
                                    == 1
                                {
                                    self.v[0xF] = 1;
                                }
                                self.gfx[(px + xline + ((py + yline as u16) * 64)) as usize] ^= 1;
                            }
                        }
                    }

                    self.draw_flag = 1;
                }
                Instruction::Skp(x) => {
                    if self.key[self.v[x as usize] as usize] != 0 {
                        self.pc += 2;
                    }
                }
                Instruction::Sknp(x) => {
                    if self.key[self.v[x as usize] as usize] == 0 {
                        self.pc += 2;
                    }
                }
                Instruction::LdVxDt(x) => self.delay_timer = self.v[x as usize],
                Instruction::LdK(x) => {
                    self.key_flag = true;
                    for i in 0..16 {
                        if self.key[i] == 1 {
                            self.v[x as usize] = i as u8;
                            self.key_flag = false;
                            break;
                        }
                    }
                }
                Instruction::LdDtVx(x) => self.delay_timer = self.v[x as usize],
                Instruction::LdSt(x) => self.sound_timer = self.v[x as usize],
                Instruction::AddI(x) => self.i = self.i.wrapping_add(self.v[x as usize] as u16),
                Instruction::LdF(x) => self.i = self.v[x as usize] as u16 * 5,
                Instruction::LdB(x) => {
                    let num = self.v[x as usize];
                    let pos = self.i as usize;

                    self.memory[pos] = num / 100;
                    self.memory[pos + 1] = (num / 10) % 10;
                    self.memory[pos + 2] = num % 10;
                }
                Instruction::LdIx(x) => self.memory.copy_from_slice(&self.v[0..x as usize]),
                Instruction::LdVxI(x) => self.v[0..x as usize].copy_from_slice(
                    &self.memory[(self.i as usize)..(self.i as usize + x as usize)],
                ),
            }
        } else {
            println!("Instruction: {:X} not found!", opcode);
        }
    }

    fn get_opcode(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16
    }

    fn next_opcode(&mut self) {
        self.pc += 2;
    }

    fn emulate_cycle(&mut self) {
        let opcode = self.get_opcode();
        self.fetch_and_execute(opcode);
        video::draw(&self.gfx);
    }

    pub fn run_game(&mut self, path: &str) {
        let game_buffer = fs::read(path).unwrap();
        self.load_game(&game_buffer);

        loop {
            limit_fps(|| {
                self.emulate_cycle();
            });

            if !self.key_flag {
                self.next_opcode();
            }
        }
    }
}

fn limit_fps<F: FnMut()>(mut f: F) {
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();

    if (elapsed.as_millis() as u16) < (1000 / 60) {
        thread::sleep(Duration::from_millis((1000 / 60) - (elapsed.as_millis() as u16) as u64));
    }
}