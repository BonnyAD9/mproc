use std::{
    fmt::Display,
    io::{self, IsTerminal},
    process::{Child, Command},
    time::Duration,
};

use eyre::Result;
use termal::writemcln;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

pub struct Measurement {
    pub time: Result<Duration>,
    pub memory: Result<usize>,
    pub exit_code: Option<i32>,
}

impl Measurement {
    pub fn measure(name: &String, args: &[String]) -> Result<Self> {
        let mut proc = Command::new(name).args(args).spawn()?;

        Self::get_stats(&mut proc)
    }

    pub fn get_stats(proc: &mut Child) -> Result<Self> {
        #[cfg(target_os = "windows")]
        {
            let res = proc.wait()?;
            let (peak_memory, time) = windows::get_stats(proc);
            Ok(Measurement {
                time,
                memory: peak_memory,
                exit_code: res.code(),
            })
        }

        #[cfg(target_os = "linux")]
        {
            let (peak_memory, time) = linux::get_stats(proc);
            Ok(Measurement {
                time,
                memory: peak_memory,
                exit_code: proc.wait()?.code(),
            })
        }
    }
}

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = f
            .precision()
            .map(|p| p != 0)
            .unwrap_or_else(|| io::stderr().is_terminal());

        writemcln!(
            f,
            color,
            "
{'gr}===============<< {'y}mproc results {'gr}>>==============={'_}"
        )?;

        match self.time {
            Ok(t) => writemcln!(f, color, "{'dm}Time: {'m bold}{:?}{'_}", t)?,
            Err(_) => writemcln!(f, color, "{'dr}Failed to get time{'_}")?,
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

fn get_mem_string(mem: usize) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "EiB", "PiB"];

    let mut level = 0;
    let mut v = mem;
    while v > 1024 {
        level += 1;
        v >>= 10;
    }

    format!(
        "{} {}",
        mem as f64 / (1 << (level * 10)) as f64,
        UNITS[level]
    )
}
