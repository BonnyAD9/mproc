use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    color_mode::ColorMode,
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

    pub fn print_measurement(
        &self,
        measurement: Measurement,
        color: ColorMode,
    ) -> Result<()> {
        match self {
            Self::Stderr => {
                let color = color.stderr() as usize;
                eprintln!("{measurement:-.color$}");
            }
            Self::Stdout => {
                let color = color.stdout() as usize;
                println!("{measurement:-.color$}");
            }
            Self::FilePath(f) => {
                let color = color.file() as usize;
                let mut f = BufWriter::new(file_create(f)?);
                writeln!(f, "{measurement:-.color$}")?;
            }
            Self::File(f) => {
                let color = color.file() as usize;
                let mut f = BufWriter::new(f);
                writeln!(f, "{measurement:-.color$}")?;
            }
        }
        Ok(())
    }
}
