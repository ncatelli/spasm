use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::process;

const HELP_STRING: &'static str = "A command-line 6502 assembler
Usage: spasm <command> [*.asm]
Commands:
  assemble: assemble a source 6502 assembly file to an object file.
  help:     print this help message
";

enum RuntimeError {
    InvalidArguments,
    FileUnreadable,
    Undefined(String),
}

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArguments => write!(f, "invalid number of arguments"),
            Self::FileUnreadable => write!(f, "source file unreadable"),
            Self::Undefined(s) => write!(f, "{}", s),
        }
    }
}

type RuntimeResult<T> = Result<T, RuntimeError>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        help().unwrap();
        process::exit(1);
    }

    match (&args[1].as_ref(), &args[2]) {
        (&"assemble", filename) => run(read_src_file(filename).unwrap()),
        _ => process::exit(help().unwrap()),
    }
    .unwrap();
}

fn read_src_file(filename: &str) -> RuntimeResult<String> {
    let mut f = File::open(filename).map_err(|_| RuntimeError::FileUnreadable)?;

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(RuntimeError::Undefined(e.to_string())),
    }
}

fn help() -> RuntimeResult<i32> {
    println!("{}", HELP_STRING);
    Ok(0)
}

fn run(_source: String) -> RuntimeResult<i32> {
    Ok(0)
}
