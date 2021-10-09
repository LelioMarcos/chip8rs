use rand::Rng;
use std::fs;

mod video;
pub mod opcode;

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
    pub pause: bool,
}

impl Chip8 {
    pub fn init() -> Chip8 {
        Chip8 {
            memory: vec![0; 4096],
            v: vec![0; 16],
            i: 0x0000,
            pc: 0x0200,
            gfx: vec![0; 64 * 32],
            delay_timer: 0x00,
            sound_timer: 0x00,
            stack: vec![0; 16],
            sp: 0x0000,
            key: vec![0; 16],
            draw_flag: 0x01,
            pause: false,
        }
    }

    fn load_game(&mut self, game_buffer: Vec<u8>) {
        for i in (0..80).into_iter() {
            self.memory[i] = FONT_SET[i];
        }

        for i in (0..game_buffer.len()).into_iter() {
            self.memory[i + 512] = game_buffer[i];
        }
    }

    fn fetch_and_execute(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => {
                    self.gfx = vec![0; 64 * 32];
                    self.pc += 2;
                }
                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize] + 2;
                }
                _ => println!("Unrecognized!!"),
            },
            0x1000 => self.pc = opcode & 0x0FFF,
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            }
            0x3000 => {
                if self.v[x] == (opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.v[x] != (opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.v[x] == self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6000 => {
                self.v[x] = (opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            0x7000 => {
                self.v[x] += (opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => self.v[x] = self.v[y],
                    0x0001 => self.v[x] |= self.v[y],
                    0x0002 => self.v[x] &= self.v[y],
                    0x0003 => self.v[x] ^= self.v[y],
                    0x0004 => {
                        self.v[0xF] = (self.v[x] as u16 + self.v[y] as u16 > 255) as u8;
                        self.v[x] += self.v[y];
                    }
                    0x0005 => {
                        self.v[0xF] = (self.v[x] > self.v[y]) as u8;
                        self.v[x] -= self.v[y];
                    }
                    0x0006 => {
                        self.v[0xF] = self.v[x] & 0x01;
                        self.v[x] >>= 1;
                    }
                    0x0007 => {
                        self.v[0xF] = (self.v[y] > self.v[x]) as u8;
                        self.v[x] = self.v[y] - self.v[x];
                    }
                    0x000E => {
                        self.v[0xF] = self.v[x] & 0x80;
                        self.v[x] <<= 1;
                    }
                    _ => println!("Unrecognized!!"),
                }
                self.pc += 2;
            }
            0x9000 => {
                if self.v[x] != self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA000 => {
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            }
            0xB000 => self.pc = (opcode & 0x0FFF) + self.v[0] as u16,
            0xC000 => {
                let mut r = rand::thread_rng();
                self.v[x] = (r.gen_range(0..=255) & (opcode & 0x00FF)) as u8;
                self.pc += 2;
            }
            0xD000 => {
                let px = self.v[x] as u16;
                let py = self.v[y] as u16;
                let height = opcode & 0x000F;
                let mut pixel: u8;

                self.v[0xF] = 0;

                for yline in 0..height {
                    pixel = self.memory[(self.i + yline) as usize];

                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            if (self.gfx[(px + xline + ((py + yline) * 64)) as usize]) == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[(px + xline + ((py + yline) * 64)) as usize] ^= 1;
                        }
                    }
                }

                self.draw_flag = 1;
                self.pc += 2;
            }
            0xE000 => match opcode & 0x00FF {
                0x009E => {
                    if self.key[self.v[x] as usize] == 1 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                0x00A1 => {
                    if self.key[self.v[x] as usize] == 0 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => println!("Unrecognized!!"),
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => {
                    self.v[x] = self.delay_timer;
                    self.pc += 2;
                }
                0x000A => {
                    for i in 0..16 {
                        if self.key[i] == 1 {
                            self.v[x] = i as u8;
                            self.pc += 2;
                            break;
                        }
                    }
                }
                0x0015 => {
                    self.delay_timer = self.v[x];
                    self.pc += 2;
                }
                0x0018 => {
                    self.sound_timer = self.v[x];
                    self.pc += 2;
                }
                0x001E => {
                    self.i += self.v[x] as u16;
                    self.pc += 2;
                }

                0x0029 => {
                    self.i = (self.v[x] * 5) as u16;
                    self.pc += 2;
                }

                0x0033 => {
                    let num = self.v[x];
                    let pos = self.i as usize;

                    self.memory[pos] = num / 100;
                    self.memory[pos + 1] = (num / 10) % 10;
                    self.memory[pos + 2] = num % 10;

                    self.pc += 2;
                }

                0x0055 => {
                    for pos in 0..=x {
                        self.memory[(self.i as usize) + pos] = self.v[pos];
                    }
                    self.pc += 2;
                }

                0x0065 => {
                    for pos in 0..=x {
                        self.v[pos] = self.memory[(self.i as usize) + pos];
                    }
                    self.pc += 2;
                }
                _ => println!("Unrecognized!!"),
            },
            _ => println!("Unrecognized!!"),
        }
    }

    fn get_opcode(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16
    }

    pub fn run_game(&mut self, path: &str) {
        let game_buffer = fs::read(path).unwrap();

        self.load_game(game_buffer);

        loop {
            let opcode = self.get_opcode();

            self.fetch_and_execute(opcode);

            video::draw(&self.gfx);
        }
    }
}
