use std::{
    process::{Child, Command},
    time::{Duration, Instant},
};
use winapi::{
    ctypes::c_void,
    shared::minwindef::FILETIME,
    um::{
        processthreadsapi::{GetProcessTimes, OpenProcess},
        psapi::{K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS},
        winnt::PROCESS_ALL_ACCESS,
    },
};

use super::Measurement;

use crate::err::{Error, Result, child_wait, cmd_spawn};

pub fn measure_one(cmd: &mut Command) -> Result<Measurement> {
    let mut proc = cmd_spawn(cmd)?;
    let start = Instant::now();
    let res = child_wait(&mut proc, cmd)?;
    let fallback_time = Instant::now() - start;

    let (peak_memory, time) = get_stats(&proc);

    Ok(Measurement {
        time: time.unwrap_or(fallback_time),
        memory: peak_memory,
        exit_code: res.code(),
    })
}

fn get_stats(proc: &Child) -> (Result<usize>, Result<Duration>) {
    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, proc.id()) };

    (get_peak_memory(handle), get_time(handle))
}

fn get_peak_memory(handle: *mut c_void) -> Result<usize> {
    let mut proc_mem = PROCESS_MEMORY_COUNTERS {
        cb: 0,
        PageFaultCount: 0,
        PeakWorkingSetSize: 0,
        WorkingSetSize: 0,
        QuotaNonPagedPoolUsage: 0,
        QuotaPeakPagedPoolUsage: 0,
        QuotaPagedPoolUsage: 0,
        QuotaPeakNonPagedPoolUsage: 0,
        PagefileUsage: 0,
        PeakPagefileUsage: 0,
    };

    let res = unsafe {
        K32GetProcessMemoryInfo(
            handle,
            &mut proc_mem,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        )
    };

    if res == 0 {
        return Err(Error::FailedToGetMemory(
            std::io::Error::last_os_error().into(),
        ));
    }

    Ok(proc_mem.PeakWorkingSetSize)
}

fn get_time(handle: *mut c_void) -> Result<Duration> {
    const EMPTY_FILETIME: FILETIME = FILETIME {
        dwHighDateTime: 0,
        dwLowDateTime: 0,
    };

    let mut creation = EMPTY_FILETIME;
    let mut exit = EMPTY_FILETIME;
    let mut kernel = EMPTY_FILETIME;
    let mut user = EMPTY_FILETIME;

    let res = unsafe {
        GetProcessTimes(
            handle,
            &mut creation,
            &mut exit,
            &mut kernel,
            &mut user,
        )
    };

    if res == 0 {
        return Err(Error::FailedToGetTime(
            std::io::Error::last_os_error().into(),
        ));
    }

    Ok(filetime_elapsed(creation, exit))
}

fn filetime_to_u64(time: FILETIME) -> u64 {
    // FILETIME is 64-bit value representing number of 100-nanosecond intervals
    time.dwLowDateTime as u64 | (time.dwHighDateTime as u64) << 32
}

fn u64_to_duration(time: u64) -> Duration {
    // 1 nanosecond is 1/10^9 seconds
    const DIV: u64 = u64::pow(10, 7);

    Duration::new(time / DIV, ((time % DIV) * 100) as u32)
}

fn filetime_elapsed(start: FILETIME, end: FILETIME) -> Duration {
    u64_to_duration(filetime_to_u64(end) - filetime_to_u64(start))
}
