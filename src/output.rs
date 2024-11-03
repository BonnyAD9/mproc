use std::{
    fs::File,
    io::{stdout, BufWriter, IsTerminal, Write},
};

use eyre::Result;

use crate::measurement::Measurement;

#[derive(Debug, Clone, Default)]
pub enum Output {
    #[default]
    Stderr,
    Stdout,
    File(String),
}

impl Output {
    pub fn validate(&self) -> Result<()> {
        if let Self::File(f) = self {
            File::create(f)?;
        }
        Ok(())
    }

    pub fn print_measurement(&self, measurement: Measurement) -> Result<()> {
        match self {
            Self::Stderr => {
                eprintln!("{measurement:-}");
            }
            Self::Stdout => {
                let color = stdout().is_terminal() as usize;
                println!("{measurement:-.color$}");
            }
            Self::File(f) => {
                let mut f = BufWriter::new(File::create(f)?);
                writeln!(f, "{measurement:-.0}")?;
            }
        }
        Ok(())
    }
}
