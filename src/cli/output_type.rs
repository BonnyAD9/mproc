use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::err::{Result, file_create};

use super::ColorMode;

#[derive(Debug, Default)]
pub enum OutputType {
    #[default]
    Stderr,
    Stdout,
    FilePath(String),
    File(BufWriter<File>),
}

impl OutputType {
    pub fn validate(&mut self) -> Result<()> {
        if let Self::FilePath(f) = self {
            *self = Self::File(BufWriter::new(file_create(f)?));
        }
        Ok(())
    }

    pub fn print(&mut self, s: impl AsRef<str>) -> Result<()> {
        match self {
            Self::Stderr => {
                eprint!("{}", s.as_ref());
            }
            Self::Stdout => {
                print!("{}", s.as_ref());
            }
            Self::FilePath(f) => {
                let mut f = BufWriter::new(file_create(f)?);
                write!(f, "{}", s.as_ref())?;
                *self = Self::File(f);
            }
            Self::File(f) => {
                write!(f, "{}", s.as_ref())?;
            }
        }

        Ok(())
    }

    pub fn color(&self, color: ColorMode) -> bool {
        match self {
            Self::Stderr => color.stderr(),
            Self::Stdout => color.stdout(),
            _ => color.file(),
        }
    }
}
