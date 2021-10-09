use chip8rs::core::*;

#[test]
fn it_adds_two() {
    let mut opcode = opcode::OpCode::new(0x00E0);
    assert_eq!(opcode.instr, opcode::Instrs::cls);
}