#[macro_use]
pub mod mos6502;

/// Backend represents the backend targets currently supported by spasm.
#[derive(Debug)]
pub enum Backend {
    MOS6502,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Backend::MOS6502 => format!("mos6502"),
        };

        write!(f, "{}", output)
    }
}
