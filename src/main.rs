extern crate scrap;
use scrap::prelude::v1::*;
use spasm::assemble;
use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const EXIT_SUCCESS: i32 = 0;
const DATA_PADDING: usize = 32768;

type RuntimeResult<T> = Result<T, RuntimeError>;

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

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

fn main() {
    let args: Vec<String> = env::args().into_iter().collect();

    let res = Cmd::new()
        .name("spasm")
        .description("An experimental 6502 assembler.")
        .author("Nate Catelli <ncatelli@packetfire.org>")
        .version("0.1.0")
        .handler(Box::new(|c| {
            println!("root dispatched with config: {:?}", c);
            Ok(0)
        }))
        .subcommand(
            Cmd::new()
                .name("assemble")
                .description("assemble a source file into it's corresponding binary format")
                .flag(
                    Flag::new()
                        .name("infile")
                        .short_code("i")
                        .help_string("an asm source filepath to assemble")
                        .value_type(ValueType::Str),
                )
                .flag(
                    Flag::new()
                        .name("outfile")
                        .short_code("o")
                        .help_string("an output path for a the corresponding binary file")
                        .value_type(ValueType::Str)
                        .default_value(Value::Str("a.out".to_string())),
                )
                .handler(Box::new(|c| {
                    match (c.get("infile"), c.get("outfile")) {
                        (Some(Value::Str(in_f)), Some(Value::Str(out_f))) => read_src_file(&in_f)
                            .map(|input| run(&input, &out_f))
                            .and_then(std::convert::identity),
                        _ => Err(RuntimeError::InvalidArguments),
                    }
                    .map_err(|e| format!("{}", e))
                })),
        )
        .run(args)
        .unwrap()
        .dispatch();

    match res {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    }
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

fn run(source: &str, dest: &str) -> RuntimeResult<i32> {
    let obj = assemble(source)
        .map_err(RuntimeError::Undefined)
        .map(|bin| bin)?;
    let data_len = obj.len();
    let padding = DATA_PADDING - data_len;
    let mut bin: Vec<u8> = obj.into_iter().chain((0..padding).map(|_| 0xea)).collect();

    // reset vector
    bin[0x7ffc] = 0x00;
    bin[0x7ffd] = 0x80;

    write_dest_file(dest, &bin)?;
    Ok(EXIT_SUCCESS)
}
