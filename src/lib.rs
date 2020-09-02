use parcel::prelude::v1::*;

#[cfg(test)]
mod tests;

pub mod instruction_set;
mod parser;

pub type AssemblerResult = Result<Vec<u8>, String>;

pub fn assemble(source: &str) -> AssemblerResult {
    let insts = match parser::instructions().parse(&source).unwrap() {
        parcel::MatchStatus::Match((_, insts)) => insts,
        _ => return Err("match error".to_string()),
    };

    Ok(insts
        .into_iter()
        .map(|i| Into::<Vec<u8>>::into(i))
        .flatten()
        .collect::<Vec<u8>>())
}
