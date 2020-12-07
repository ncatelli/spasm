extern crate parcel;
use crate::parser::{hex_u16, hex_u32, hex_u8};
use parcel::prelude::v1::*;

#[test]
fn should_parse_a_matching_u8_hex_value_into_any_encompassing_unsigned_integer() {
    let input: Vec<char> = "0xFF".chars().collect();

    // u8
    assert_eq!(
        Ok(MatchStatus::Match((&input[4..], 0xff))),
        hex_u8().parse(&input)
    );

    // u16
    assert_eq!(
        Ok(MatchStatus::Match((&input[4..], 0xff))),
        hex_u16().parse(&input)
    );

    // u32
    assert_eq!(
        Ok(MatchStatus::Match((&input[4..], 0xff))),
        hex_u32().parse(&input)
    );
}
