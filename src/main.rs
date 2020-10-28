extern crate scrap;
use scrap::prelude::v1::*;
use spasm::assemble;
use spasm::Backend;
use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const EXIT_SUCCESS: i32 = 0;

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
                        .name("in-file")
                        .short_code("i")
                        .help_string("an asm source filepath to assemble")
                        .value_type(ValueType::Str),
                )
                .flag(
                    Flag::new()
                        .name("out-file")
                        .short_code("o")
                        .help_string("an output path for a the corresponding binary file")
                        .value_type(ValueType::Str)
                        .default_value(Value::Str("a.out".to_string())),
                )
                .flag(
                    Flag::new()
                        .name("padding")
                        .short_code("p")
                        .help_string("size in bytes to pad the output binary to")
                        .value_type(ValueType::Integer)
                        .default_value(Value::Integer(32768)),
                )
                .handler(Box::new(|c| {
                    {
                        match (c.get("in-file"), c.get("out-file"), c.get("padding")) {
                            (
                                Some(Value::Str(in_f)),
                                Some(Value::Str(out_f)),
                                Some(&Value::Integer(bin_size)),
                            ) => Ok((in_f, out_f, bin_size as usize)),
                            _ => Err(RuntimeError::InvalidArguments),
                        }
                        .map(|(in_f, out_f, padding)| {
                            read_src_file(&in_f)
                                .map(|input| assemble_object(&input, padding))?
                                .map(|bin_data| {
                                    write_dest_file(&out_f, &bin_data).map(|_| EXIT_SUCCESS)
                                })?
                        })
                    }
                    .and_then(std::convert::identity)
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

fn assemble_object(asm_src: &str, bin_size: usize) -> RuntimeResult<Vec<u8>> {
    let obj = assemble(Backend::MOS6502, asm_src)
        .map_err(RuntimeError::Undefined)
        .map(|bin| bin)?;

    let data_len = obj.len();
    let padding = bin_size - data_len;

    let bin: Vec<u8> = obj.into_iter().chain((0..padding).map(|_| 0xea)).collect();

    Ok(bin)
}
