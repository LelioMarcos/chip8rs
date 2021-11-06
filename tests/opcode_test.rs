use chip8rs::core::*;

#[test]
fn recognize_opcode_for_instruction() {
    let opcode = Instruction::from_u16(0x10E0).unwrap();
    assert_eq!(opcode, Instruction::Jp(0xE0));
}
