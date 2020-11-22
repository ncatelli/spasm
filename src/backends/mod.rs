use std::convert::TryFrom;

#[macro_use]
pub mod mos6502;

/// Error type returned from backends.
pub enum BackendErr {
    ParseError(String),
}

impl std::fmt::Display for BackendErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            BackendErr::ParseError(input) => format!("unable to parse: {}", input),
        };

        write!(f, "{}", output)
    }
}

/// Backend represents the backend targets currently supported by spasm.
#[derive(Debug)]
pub enum Backend {
    MOS6502,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Backend::MOS6502 => "mos6502".to_string(),
        };

        write!(f, "{}", output)
    }
}

impl TryFrom<&str> for Backend {
    type Error = String;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        match src {
            "mos6502" => Ok(Backend::MOS6502),
            _ => Err(format!("unknown backend: {}", &src)),
        }
    }
}
