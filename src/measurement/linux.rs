use std::{
    mem,
    process::Command,
    time::{Duration, Instant},
};

use libc::{RUSAGE_CHILDREN, getrusage, timeval};

use crate::err::{Error, Result, child_wait, cmd_spawn};

use super::Measurement;

pub fn measure_one(cmd: &mut Command) -> Result<Measurement> {
    let mut proc = cmd_spawn(cmd)?;
    let start = Instant::now();
    let code = child_wait(&mut proc, cmd)?.code();
    let end = Instant::now();
    let time = end - start;

    let peak_memory = get_stats().map(|a| a.0);

    Ok(Measurement {
        time,
        memory: peak_memory,
        exit_code: code,
    })
}

fn get_stats() -> Result<(usize, Duration)> {
    let mut stats = unsafe { mem::zeroed() };

    let res = unsafe { getrusage(RUSAGE_CHILDREN, &mut stats) };
    if res == -1 {
        return Err(Error::Io(std::io::Error::last_os_error()));
    }

    let mem = stats.ru_maxrss as usize;
    let time = get_duration(stats.ru_stime) + get_duration(stats.ru_stime);

    Ok((mem * 1024, time))
}

fn get_duration(t: timeval) -> Duration {
    Duration::new(t.tv_sec as u64, t.tv_usec as u32 * 1000)
}
