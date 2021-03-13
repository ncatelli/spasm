use crate::preparser::{types, PreParser, PrimitiveOrReference, Token};
use parcel::prelude::v1::*;

macro_rules! chars {
    ($input:expr) => {
        $input.chars().collect::<Vec<char>>()
    };
}

macro_rules! zero_origin {
    ($insts:expr) => {
        $crate::Origin::new($insts)
    };
}

#[test]
fn should_parse_instruction_to_string() {
    let input = chars!("nop");

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![zero_origin!(vec![Token::Instruction("nop".to_string())])]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_label() {
    let input = chars!("test:");

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![zero_origin!(vec![Token::Symbol("test".to_string(), None)])]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_single_byte_constant() {
    let input = chars!(".define byte test 255");

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![zero_origin!(vec![Token::Symbol(
                "test".to_string(),
                Some(types::LEByteEncodedValue::from(255u8))
            )])]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_two_byte_constant() {
    let input = chars!(".define word test 65535");

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![zero_origin!(vec![Token::Symbol(
                "test".to_string(),
                Some(types::LEByteEncodedValue::from(65535u16))
            )])]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_four_byte_constant() {
    let input = chars!(".define doubleword test 4294967295");

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![zero_origin!(vec![Token::Symbol(
                "test".to_string(),
                Some(types::LEByteEncodedValue::from(4294967295u32))
            )])]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_origin() {
    let input = chars!("nop\n.origin 0x00001a2b\nnop");

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![
                crate::Origin::new(vec![Token::Instruction("nop".to_string())]),
                crate::Origin::with_offset(0x1a2b, vec![Token::Instruction("nop".to_string())])
            ]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_constants() {
    let input = chars!(
        "
.byte       0x1a
.word       0x1a2b
.doubleword 0x1a2b3c4d
"
    );

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![crate::Origin::new(vec![
                Token::Constant(PrimitiveOrReference::Primitive(
                    types::LEByteEncodedValue::from(0x1au8)
                )),
                Token::Constant(PrimitiveOrReference::Primitive(
                    types::LEByteEncodedValue::from(0x1a2bu16)
                )),
                Token::Constant(PrimitiveOrReference::Primitive(
                    types::LEByteEncodedValue::from(0x1a2b3c4du32)
                ))
            ]),]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_constants_as_origin_statement() {
    let input = chars!(
        "
.origin 0x03
  .byte       0x1a
"
    );

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![crate::Origin::with_offset(
                0x03,
                vec![Token::Constant(PrimitiveOrReference::Primitive(
                    types::LEByteEncodedValue::from(0x1au8)
                )),]
            ),]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_labels_as_constant_arguments() {
    let input = chars!(
        "
.define byte test 0xff
init:
.origin 0x03
  .word       init
  .byte       test
"
    );

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![
                crate::Origin::new(vec![
                    Token::Symbol(
                        "test".to_string(),
                        Some(types::LEByteEncodedValue::from(0xffu8))
                    ),
                    Token::Symbol("init".to_string(), None)
                ]),
                crate::Origin::with_offset(
                    0x03,
                    vec![
                        Token::Constant(PrimitiveOrReference::Reference("init".to_string())),
                        Token::Constant(PrimitiveOrReference::Reference("test".to_string()))
                    ]
                ),
            ]
        ))),
        PreParser::new().parse(&input)
    );
}
