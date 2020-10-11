use crate::preparser::{ByteValue, PreParser, Token};
use parcel::prelude::v1::*;

macro_rules! chars {
    ($input:expr) => {
        $input.chars().collect::<Vec<char>>()
    };
}

#[test]
fn should_parse_instruction_to_string() {
    let input = chars!("nop");

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![Token::Instruction("nop".to_string())]
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
            vec![Token::Label("test".to_string())]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_single_byte_constant() {
    let input = chars!(".1byte test $ff");

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![Token::Symbol(("test".to_string(), ByteValue::One(255)))]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_two_byte_constant() {
    let input = chars!(".2byte test $FFFF");

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![Token::Symbol(("test".to_string(), ByteValue::Two(65535)))]
        ))),
        PreParser::new().parse(&input)
    );
}

#[test]
fn should_parse_four_byte_constant() {
    let input = chars!(".4byte test $FFFFFFFF");

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[input.len()..],
            vec![Token::Symbol((
                "test".to_string(),
                ByteValue::Four(4294967295)
            ))]
        ))),
        PreParser::new().parse(&input)
    );
}
