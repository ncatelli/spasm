use scrap::prelude::v1::*;
use spasm::assemble;
use spasm::Backend;
use spasm::Emitter;
use std::convert::TryFrom;
use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const CMD_VERSION: &str = "1.0.0";

type RuntimeResult<T> = Result<T, RuntimeError>;

enum RuntimeError {
    InvalidArguments(String),
    FileUnreadable,
    Undefined(String),
}

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArguments(hs) => write!(f, "{}", hs),
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

fn main() -> RuntimeResult<()> {
    let raw_args: Vec<String> = env::args().into_iter().collect::<Vec<String>>();
    let args = raw_args.iter().map(|a| a.as_str()).collect::<Vec<&str>>();

    let help_flag = scrap::Flag::store_true("help", "h", "display usage information.").optional();
    let version_flag =
        scrap::Flag::store_true("version", "v", "output version information.").optional();
    let output_flag = scrap::FlagWithValue::new(
        "out-file",
        "o",
        "an output path for the corresponding binary.",
        scrap::FileValue::new(true, true, false),
    )
    .optional()
    .with_default("a.out".to_string());
    let backend_flag = scrap::Flag::with_choices(
        "backend",
        "b",
        "a target architecture backend.",
        ["mos6502".to_string()],
        scrap::StringValue,
    )
    .optional()
    .with_default("mos6502".to_string());

    let cmd_group = scrap::CmdGroup::new("spasm")
        .description("An experimental multi-target assembler.")
        .author("Nate Catelli <ncatelli@packetfire.org>")
        .version(CMD_VERSION)
        .with_command(
            scrap::Cmd::new("assemble")
                .description("assemble a source file into its corresponding binary format")
                .with_flag(version_flag)
                .with_flag(output_flag)
                .with_flag(backend_flag)
                .with_flag(help_flag)
                .with_args_handler(|args, (((version, output), backend), _)| {
                    if version.is_some() {
                        println!("{}", CMD_VERSION);
                        Ok(())
                    } else {
                        args.into_iter()
                            .map(|path| {
                                let in_f = path.unwrap();
                                read_src_file(&in_f)
                                    .and_then(|input| assemble_object(&backend, &input))
                                    .and_then(|bin_data| write_dest_file(&output, &bin_data))
                            })
                            .collect::<Result<Vec<()>, _>>()
                            .map(|_| ())
                    }
                }),
        );

    let help_cmd = cmd_group.help();
    cmd_group
        .evaluate(&args[..])
        .map_err(|e| RuntimeError::Undefined(format!("{}\n{}", e, help_cmd)))
        .and_then(
            |scrap::Value {
                 span,
                 value: (flags, help),
             }| {
                if help.is_some() {
                    println!("{}", help_cmd);
                    Ok(())
                } else {
                    let unmatched_args = scrap::return_unused_args(&args[..], &span);
                    cmd_group.dispatch_with_args(unmatched_args, Value::new(span, (flags, help)))
                }
            },
        )
}

fn read_src_file<F: AsRef<str>>(filename: F) -> RuntimeResult<String> {
    let mut f = File::open(filename.as_ref()).map_err(|_| RuntimeError::FileUnreadable)?;

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(RuntimeError::Undefined(e.to_string())),
    }
}

fn write_dest_file<F: AsRef<str>>(filename: F, data: &[u8]) -> RuntimeResult<()> {
    let mut f = OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(filename.as_ref())
        .map_err(|_| RuntimeError::FileUnreadable)?;

    match f.write_all(data) {
        Ok(_) => Ok(()),
        Err(e) => Err(RuntimeError::Undefined(e.to_string())),
    }
}

fn assemble_object<B, S>(backend: B, asm_src: S) -> RuntimeResult<Vec<u8>>
where
    B: AsRef<str>,
    S: AsRef<str>,
{
    let backend: Backend = Backend::try_from(backend.as_ref()).map_err(|_| {
        RuntimeError::InvalidArguments(format!("unknown backend: {}", backend.as_ref()))
    })?;

    let obj = assemble(backend, asm_src.as_ref()).map_err(RuntimeError::Undefined)?;
    let bin: Vec<u8> = obj.emit();

    Ok(bin)
}
