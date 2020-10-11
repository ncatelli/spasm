use crate::preparser::parser::{statement, Token};
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
            &input[3..],
            vec![Token::Instruction("nop".to_string())]
        ))),
        statement().parse(&input)
    );
}
