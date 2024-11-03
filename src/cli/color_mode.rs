use std::io::{self, IsTerminal};

use pareg::FromArg;

#[derive(Copy, Clone, FromArg, Default, Debug)]
pub enum ColorMode {
    #[default]
    Auto,
    Always,
    Never,
}

impl ColorMode {
    pub fn stdout(&self) -> bool {
        match self {
            ColorMode::Auto => io::stdout().is_terminal(),
            ColorMode::Always => true,
            ColorMode::Never => false,
        }
    }

    pub fn stderr(&self) -> bool {
        match self {
            ColorMode::Auto => io::stderr().is_terminal(),
            ColorMode::Always => true,
            ColorMode::Never => false,
        }
    }

    pub fn file(&self) -> bool {
        match self {
            ColorMode::Auto => false,
            ColorMode::Always => true,
            ColorMode::Never => false,
        }
    }
}
