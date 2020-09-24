use crate::assemble;

#[test]
fn should_generate_expected_opcode() {
    let input = "nop
lda #%00010010
sta 4660
bpl *$1a
bpl *-16
jmp $1234\n";

    assert_eq!(
        Ok(vec![
            0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x10, 0x1a, 0x10, 0xf0, 0x4c, 0x34, 0x12
        ]),
        assemble(input)
    )
}

#[test]
fn should_generate_expected_opcodes_with_label() {
    let input = "
nop
lda #$12

init:
  nop
  lda #%00010010
  sta 4660
  jmp init
";

    assert_eq!(
        Ok(vec![
            0xea, 0xa9, 0x12, 0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x4c, 0x03, 0x00
        ]),
        assemble(input)
    )
}

#[test]
fn should_throw_an_error_if_a_label_doesnt_exist() {
    let input = "
nop

init:
  nop
  lda #%00010010
  sta 4660
  jmp notinit
";

    assert_eq!(
        Err("label notinit, undefined at line: 6".to_string()),
        assemble(input)
    )
}

#[test]
fn should_generate_expected_opcodes_from_address_mode_symbols() {
    let input = "
define test $12

nop
lda #test
sta $1234
jmp $1234
";

    assert_eq!(
        Ok(vec![0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x4c, 0x34, 0x12]),
        assemble(input)
    )
}

#[test]
fn should_throw_an_error_if_a_symbol_doesnt_exist() {
    let input = "
nop
lda #test
sta $1234
jmp $1234
";

    assert_eq!(
        Err("symbol test, undefined at line: 2".to_string()),
        assemble(input)
    )
}

#[test]
fn should_differentate_between_label_and_symbol_definitions() {
    let input = "
define thisisatest $12

init:
    nop
    lda #thisisatest
    sta $1234
    jmp init
";

    assert_eq!(
        Ok(vec![0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x4c, 0x00, 0x00]),
        assemble(input)
    )
}
