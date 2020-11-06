extern crate scrap;
use scrap::prelude::v1::*;
use spasm::assemble;
use spasm::Backend;
use spasm::Emitter;
use std::convert::TryFrom;
use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const EXIT_SUCCESS: i32 = 0;

const ROOT_HELP_STRING: &str = "Usage: spasm [OPTIONS]
An experimental multi-target assembler.

Flags:
    --help, -h          print help string
    --version, -v       

Subcommands:
    assemble            assemble a source file into it's corresponding binary format";

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
        .description("An experimental multi-target assembler.")
        .author("Nate Catelli <ncatelli@packetfire.org>")
        .version("0.1.0")
        .flag(
            Flag::new()
                .name("version")
                .short_code("v")
                .action(Action::StoreTrue)
                .value_type(ValueType::Bool),
        )
        .handler(Box::new(|_| {
            println!("{}", ROOT_HELP_STRING);
            Ok(0)
        }))
        .subcommand(
            Cmd::new()
                .name("assemble")
                .description("assemble a source file into its corresponding binary format")
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
                        .help_string("an output path for the corresponding binary file")
                        .value_type(ValueType::Str)
                        .default_value(Value::Str("a.out".to_string())),
                )
                .flag(
                    Flag::new()
                        .name("backend")
                        .short_code("b")
                        .help_string("specify the target backend to assemble")
                        .value_type(ValueType::Str)
                        .default_value(Value::Str("mos6502".to_string())),
                )
                .handler(Box::new(|c| {
                    {
                        match (c.get("in-file"), c.get("out-file"), c.get("backend")) {
                            (
                                Some(Value::Str(in_f)),
                                Some(Value::Str(out_f)),
                                Some(Value::Str(b_f)),
                            ) => Ok((in_f, out_f, b_f)),
                            _ => Err(RuntimeError::InvalidArguments),
                        }
                        .map(|(in_f, out_f, backend_f)| {
                            read_src_file(&in_f)
                                .map(|input| assemble_object(backend_f, &input))?
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

fn assemble_object(backend_str: &str, asm_src: &str) -> RuntimeResult<Vec<u8>> {
    let backend: Backend =
        Backend::try_from(backend_str).map_err(|_| RuntimeError::InvalidArguments)?;

    let obj = assemble(backend, asm_src).map_err(RuntimeError::Undefined)?;

    let bin: Vec<u8> = obj.emit();

    Ok(bin)
}
