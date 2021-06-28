use std::convert::TryFrom;

#[macro_use]
pub mod mos6502;

/// Error type returned from backends.
#[allow(dead_code)]
pub enum BackendErr {
    Parse(String),
    UndefinedReference(String),
    UndefinedInstruction(String),
    Unspecified(String),
}

impl std::fmt::Display for BackendErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Parse(input) => input.clone(),
            Self::UndefinedReference(input) => format!("reference undefined: {}", input),
            Self::UndefinedInstruction(input) => input.clone(),
            Self::Unspecified(input) => input.clone(),
        };

        write!(f, "{}", output)
    }
}

/// Backend represents the backend targets currently supported by spasm.
#[derive(Debug)]
pub enum Backend {
    Mos6502,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Backend::Mos6502 => "mos6502".to_string(),
        };

        write!(f, "{}", output)
    }
}

impl TryFrom<&str> for Backend {
    type Error = String;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        match src {
            "mos6502" => Ok(Backend::Mos6502),
            _ => Err(format!("unknown backend: {}", &src)),
        }
    }
}
