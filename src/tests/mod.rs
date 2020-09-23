use crate::assemble;

#[test]
fn should_generate_expected_opcode() {
    let input = "nop
lda #%00010010
sta 4660
jmp $1234\n";

    assert_eq!(
        vec![0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x4c, 0x34, 0x12],
        assemble(input).unwrap()
    )
}

#[test]
fn should_generate_expected_with_label() {
    let input = "
nop
nop

init:
  nop
  lda #%00010010
  sta 4660
  jmp init\n";

    assert_eq!(
        vec![0xea, 0xea, 0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x4c, 0x02, 0x00],
        assemble(input).unwrap()
    )
}
