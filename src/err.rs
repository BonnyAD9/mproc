use std::{
    fs::File,
    process::{Child, Command, ExitStatus},
};

use termal::eprintacln;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(
        "Failed to get memory because the process finished before it could be \
        obtained."
    )]
    #[cfg(target_os = "linux")]
    ProcessTooFast,
    #[error("Failed to write to file `{0}`: {1}")]
    FailedToWrite(String, std::io::Error),
    #[error("Failed to spawn the process `{0}`: {1}")]
    FailedToSpawn(String, std::io::Error),
    #[error("Failed to wait for the process `{0}`: {1}")]
    FailedToWait(String, std::io::Error),
    #[error("Failed to get process memory: {0}")]
    FailedToGetMemory(anyhow::Error),
    #[error("Failed to get process time: {0}")]
    #[cfg(target_os = "windows")]
    FailedToGetTime(anyhow::Error),
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn print(&self) {
        if matches!(self, Self::Pareg(_)) {
            eprintln!("{self}");
        } else {
            eprintacln!("{'r}Error: {'_}{self}");
        }
    }
}

pub fn file_create(p: &str) -> Result<File> {
    File::create(p).map_err(|e| Error::FailedToWrite(p.to_string(), e))
}

pub fn cmd_spawn(cmd: &mut Command) -> Result<Child> {
    cmd.spawn().map_err(|e| {
        Error::FailedToSpawn(
            cmd.get_program().to_string_lossy().into_owned(),
            e,
        )
    })
}

pub fn child_wait(child: &mut Child, cmd: &Command) -> Result<ExitStatus> {
    child.wait().map_err(|e| {
        Error::FailedToWait(
            cmd.get_program().to_string_lossy().into_owned(),
            e,
        )
    })
}
