use std::{
    fmt::Display,
    io::{self, IsTerminal},
    process::Command,
    time::Duration,
};

use termal::writemcln;

use crate::{err::Result, get_mem_string};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

pub struct Measurement {
    pub time: Duration,
    pub memory: Result<usize>,
    pub exit_code: Option<i32>,
}

impl Measurement {
    pub fn measure(cmd: &mut Command) -> Result<Self> {
        #[cfg(target_os = "windows")]
        {
            windows::measure_one(cmd)
        }

        #[cfg(target_os = "linux")]
        {
            linux::measure_one(cmd)
        }
    }
}

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = f
            .precision()
            .map(|p| p != 0)
            .unwrap_or_else(|| io::stderr().is_terminal());

        if f.sign_minus() {
            writemcln!(
                f,
                color,
                "
{'gr}===============<< {'y}mproc results {'gr}>>==============={'_}"
            )?;
        }

        let w = f.width().unwrap_or_default();
        if w > 0 {
            write!(f, "{:>w$}", ' ')?;
        }

        writemcln!(f, color, "{'dm}Time: {'m bold}{:?}{'_}", self.time)?;

        if w > 0 {
            write!(f, "{:>w$}", ' ')?;
        }

        match self.memory {
            Ok(m) => writemcln!(
                f,
                color,
                "{'dc}Memory: {'c bold}{}{'_}",
                get_mem_string(m)
            )?,
            Err(_) => writemcln!(f, color, "{'dr}Failed to get memory{'_}")?,
        }

        if w > 0 {
            write!(f, "{:>w$}", ' ')?;
        }

        match self.exit_code {
            Some(e) => {
                if e == 0 {
                    writemcln!(f, color, "{'dg}Exit code: {'g bold}{}{'_}", e)
                } else {
                    writemcln!(f, color, "{'dr}Exit code: {'r bold}{}{'_}", e)
                }
            }
            None => writemcln!(f, color, "{'dr}No exit code{'_}"),
        }
    }
}
