use crate::preparser::{ByteValue, ByteValueOrReference, PreParser, Token};
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
            vec![zero_origin!(vec![Token::Label("test".to_string())])]
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
            vec![zero_origin!(vec![Token::Symbol((
                "test".to_string(),
                ByteValue::Byte(255)
            ))])]
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
            vec![zero_origin!(vec![Token::Symbol((
                "test".to_string(),
                ByteValue::Word(65535)
            ))])]
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
            vec![zero_origin!(vec![Token::Symbol((
                "test".to_string(),
                ByteValue::DoubleWord(4294967295)
            ))])]
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
                Token::Constant(ByteValueOrReference::ByteValue(ByteValue::Byte(0x1a))),
                Token::Constant(ByteValueOrReference::ByteValue(ByteValue::Word(0x1a2b))),
                Token::Constant(ByteValueOrReference::ByteValue(ByteValue::DoubleWord(
                    0x1a2b3c4d
                )))
            ]),]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_constants_as_origin_statement() {
    let input = chars!(
        "
.origin 0x00000003
  .byte       0x1a
"
    );

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![crate::Origin::with_offset(
                0x03,
                vec![Token::Constant(ByteValueOrReference::ByteValue(
                    ByteValue::Byte(0x1a)
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
.origin 0x00000003
  .word       init
  .byte       test
"
    );

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![
                crate::Origin::new(vec![
                    Token::Symbol(("test".to_string(), ByteValue::Byte(0xff))),
                    Token::Label("init".to_string())
                ]),
                crate::Origin::with_offset(
                    0x03,
                    vec![
                        Token::Constant(ByteValueOrReference::Reference("init".to_string())),
                        Token::Constant(ByteValueOrReference::Reference("test".to_string()))
                    ]
                ),
            ]
        ))),
        PreParser::new().parse(&input)
    );
}
