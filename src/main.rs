use spasm::assemble;
use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::process;

const EXIT_SUCCESS: i32 = 0;
const DATA_PADDING: usize = 32768;
const HELP_STRING: &str = "A command-line 6502 assembler

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
        (&"assemble", filename) => run(&read_src_file(filename).unwrap()),
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

fn write_dest_file(filename: &str, data: &[u8]) -> RuntimeResult<()> {
    let mut f = OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(filename)
        .map_err(|_| RuntimeError::FileUnreadable)?;

    match f.write_all(data) {
        Ok(_) => Ok(()),
        Err(e) => Err(RuntimeError::Undefined(e.to_string())),
    }
}

fn help() -> RuntimeResult<i32> {
    println!("{}", HELP_STRING);
    Ok(0)
}

fn run(source: &str) -> RuntimeResult<i32> {
    let obj = assemble(source)
        .map_err(RuntimeError::Undefined)
        .map(|bin| bin)?;
    let data_len = obj.len();
    let padding = DATA_PADDING - data_len;
    let mut bin: Vec<u8> = obj.into_iter().chain((0..padding).map(|_| 0xea)).collect();

    // reset vector
    bin[0x7ffc] = 0x00;
    bin[0x7ffd] = 0x80;

    write_dest_file("a.out", &bin)?;
    Ok(EXIT_SUCCESS)
}
