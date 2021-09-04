enum Instructions {
    Cls = 0x00E0, 
    Ret = 0x00EE,
    Sys()
}

struct Opcode {
    instr: u16;
    params: Vec<u16>;
}