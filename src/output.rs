use std::{
    fs::File,
    io::{stdout, BufWriter, IsTerminal, Write},
};

use crate::{
    err::{file_create, Result},
    measurement::Measurement,
};

#[derive(Debug, Default)]
pub enum Output {
    #[default]
    Stderr,
    Stdout,
    FilePath(String),
    File(File),
}

impl Output {
    pub fn validate(&mut self) -> Result<()> {
        if let Self::FilePath(f) = self {
            *self = Self::File(file_create(f)?);
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
            Self::FilePath(f) => {
                let mut f = BufWriter::new(file_create(f)?);
                writeln!(f, "{measurement:-.0}")?;
            }
            Self::File(f) => {
                let mut f = BufWriter::new(f);
                writeln!(f, "{measurement:-.0}")?;
            }
        }
        Ok(())
    }
}
