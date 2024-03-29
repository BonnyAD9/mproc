use eyre::{Report, Result};
use procfs::process::Process;
use std::{
    process::Child,
    time::{Duration, Instant},
};

pub fn get_stats(proc: &Child) -> (Result<usize>, Result<Duration>) {
    let proc = match Process::new(proc.id() as i32) {
        Ok(p) => p,
        Err(e) => return (Err(Report::new(e)), Err(Report::msg("msg"))),
    };

    let time_start = Instant::now();

    // this implicitly waits for the process to end
    let mem = get_peak_memory(&proc);

    let time = Instant::now() - time_start;

    (mem, Ok(time))
}

fn get_peak_memory(proc: &Process) -> Result<usize> {
    let mut max = 0;

    loop {
        match get_cur_memory(proc) {
            Ok(m) => {
                if m == 0 {
                    if !proc.is_alive() {
                        if max == 0 {
                            return Err(Report::msg("couldn't get memory"));
                        }
                        return Ok(max);
                    }
                } else if m > max {
                    max = m;
                }
            }
            Err(e) => {
                if max == 0 {
                    return Err(e);
                }
                return Ok(max);
            }
        }
    }
}

fn get_cur_memory(proc: &Process) -> Result<usize> {
    let mem = proc.stat()?.rss_bytes()?;
    Ok(mem as usize)
}
