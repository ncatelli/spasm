use crate::instruction_set::InstructionOrSymbol;
use crate::parser::instructions;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

#[test]
fn should_generate_expected_opcode() {
    let input = "nop
lda #%00010010
sta 4660
jmp $1234\n";

    let insts = match instructions().parse(&input).unwrap() {
        MatchStatus::Match((_, insts)) => insts,
        _ => panic!("pattern should match."),
    };

    assert_eq!(
        vec![0xea, 0xa9, 0x12, 0x8d, 0x34, 0x12, 0x4c, 0x34, 0x12],
        insts
            .into_iter()
            .map(|ios| match ios {
                InstructionOrSymbol::Instruction(i) => i,
                noti @ _ => panic!("each item in test shoudl match instructions: {:?}", noti),
            })
            .map(|i| Into::<Vec<u8>>::into(i))
            .flatten()
            .collect::<Vec<u8>>()
    )
}
