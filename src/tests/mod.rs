use crate::assemble;
use crate::backends::Backend;
use crate::Emitter;

macro_rules! zero_origin {
    ($insts:expr) => {
        $crate::Origin::new($insts)
    };
}

#[test]
fn should_generate_expected_origin() {
    let input = "nop
lda #0b00010010
sta 4660
bpl *0x1a
bpl *-16
jmp 0x1234\n";

    assert_eq!(
        Ok(vec![zero_origin!(vec![
            0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x10, 0x1a, 0x10, 0xf0, 0x4c, 0x34, 0x12
        ])]),
        assemble(Backend::MOS6502, input)
    )
}

#[test]
fn should_generate_expected_opcodes_with_label() {
    let input = "
nop
lda #0x12

init:
  nop
  lda #0b00010010
  sta 4660
  jmp init
";

    assert_eq!(
        Ok(vec![zero_origin!(vec![
            0xea, 0xa9, 0x12, 0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x4c, 0x03, 0x00
        ])]),
        assemble(Backend::MOS6502, input)
    )
}

#[test]
fn should_throw_an_error_if_a_label_doesnt_exist() {
    let input = "
nop

init:
  nop
  lda #0b00010010
  sta 4660
  jmp notinit
";

    assert_eq!(
        Err("label notinit undefined".to_string()),
        assemble(Backend::MOS6502, input)
    )
}

#[test]
fn should_generate_expected_opcodes_from_address_mode_symbols() {
    let input = "
.define byte test 0x12

nop
lda #test
sta 0x1234
jmp 0x1234
";

    assert_eq!(
        Ok(vec![zero_origin!(vec![
            0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x4c, 0x34, 0x12
        ])]),
        assemble(Backend::MOS6502, input)
    )
}

#[test]
fn should_throw_an_error_if_a_symbol_doesnt_exist() {
    let input = "
nop
lda #test
sta 0x1234
jmp 0x1234
";

    assert_eq!(
        Err("symbol test undefined".to_string()),
        assemble(Backend::MOS6502, input)
    )
}

#[test]
fn should_differentate_between_label_and_symbol_definitions() {
    let input = "
.define byte thisisatest 0x12

init:
    nop
    lda #thisisatest
    sta 0x1234
    jmp init
";

    assert_eq!(
        Ok(vec![zero_origin!(vec![
            0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x4c, 0x00, 0x00
        ])]),
        assemble(Backend::MOS6502, input)
    )
}

#[test]
fn should_ignore_comments_on_each_statement_type() {
    let input = "
.define byte test 0x12 ; test

init: ; test
    nop ; test
    lda #test ; test 
    sta 0x1234 ; test
    jmp init ; test
";

    assert_eq!(
        Ok(vec![zero_origin!(vec![
            0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x4c, 0x00, 0x00
        ])]),
        assemble(Backend::MOS6502, input)
    )
}

#[test]
fn should_pad_space_between_origins_in_assembled_output() {
    let input = "
nop
.origin 0x00000003
  nop
.origin 0x00000006
  nop
";

    assert_eq!(
        Ok(vec![0xea, 0x00, 0x00, 0xea, 0x00, 0x00, 0xea]),
        assemble(Backend::MOS6502, input).map(|res| res.emit())
    );
}

#[test]
fn constants_should_emit_with_instructions() {
    let input = "
nop
.origin 0x00000003
  nop
  .word 0x1a2b
.origin 0x00000006
  nop
";

    assert_eq!(
        Ok(vec![0xea, 0x00, 0x00, 0xea, 0x2b, 0x1a, 0xea]),
        assemble(Backend::MOS6502, input).map(|res| res.emit())
    );
}
