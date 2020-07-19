use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::process;

enum RuntimeError {
    InvalidArguments,
    Other(String),
}

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArguments => write!(f, "invalid number of arguments."),
            Self::Other(s) => write!(f, "{}", s),
        }
    }
}

type RuntimeResult<T> = Result<T, RuntimeError>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    match args_len {
        2 => run_file(&args[1]).expect("Unable to parse file"),
        _ => {
            println!("Usage: spasm [*.asm]");
            process::exit(64);
        }
    }
}

fn run_file(filename: &str) -> Result<(), RuntimeError> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => {
            run(contents).unwrap();
            Ok(())
        }
        Err(e) => Err(RuntimeError::Other(e.to_string())),
    }
}

fn run(_source: String) -> RuntimeResult<()> {
    Ok(())
}
