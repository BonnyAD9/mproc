use anyhow::anyhow;
use procfs::process::Process;
use std::{
    process::{Child, Command},
    time::{Duration, Instant},
};

use crate::err::{child_wait, cmd_spawn, Error, Result};

use super::Measurement;

pub fn measure_one(cmd: &mut Command) -> Result<Measurement> {
    let mut proc = cmd_spawn(cmd)?;
    let start = Instant::now();
    let (peak_memory, time) = get_stats(&proc);

    let code = child_wait(&mut proc, cmd)?.code();
    let fallback_time = Instant::now() - start;

    Ok(Measurement {
        time: time.unwrap_or(fallback_time),
        memory: peak_memory,
        exit_code: code,
    })
}

fn get_stats(proc: &Child) -> (Result<usize>, Result<Duration>) {
    let proc = match Process::new(proc.id() as i32) {
        Ok(p) => p,
        Err(e) => {
            let err = anyhow!("{e}");
            return (Err(Error::Other(e.into())), Err(err.into()));
        }
    };

    let time_start = Instant::now();

    // this implicitly waits for the process to end
    let mem = get_peak_memory(&proc);

    let time = Instant::now() - time_start;

    (mem, Ok(time))
}

fn get_peak_memory(proc: &Process) -> Result<usize> {
    let mut max = None;

    loop {
        match get_cur_memory(proc) {
            Ok(m) => {
                if m == 0 && !proc.is_alive() {
                    return max.ok_or(Error::ProcessTooFast);
                }
                max = Some(max.unwrap_or_default().max(m));
            }
            Err(e) => {
                return max.ok_or(e);
            }
        }
    }
}

fn get_cur_memory(proc: &Process) -> Result<usize> {
    proc.stat()
        .map_err(|e| Error::FailedToGetMemory(e.into()))?
        .rss_bytes()
        .map_err(|e| Error::FailedToGetMemory(e.into()))
        .map(|m| m as usize)
}
