use eyre::Result;
use std::{
    env,
    process::{Child, Command},
    time::Duration,
};

#[cfg(target_os = "windows")]
mod windows;

struct Measurement {
    time: Result<Duration>,
    memory: Result<usize>,
    exit_code: Option<i32>,
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        help();
        return Ok(());
    }

    let stats = measure_process(&args[1], &args[2..])?;

    print_stats(&stats);

    Ok(())
}

fn measure_process(name: &String, args: &[String]) -> Result<Measurement> {
    let mut proc = Command::new(name).args(args).spawn()?;

    let res = proc.wait()?;

    let (peak_memory, time) = get_stats(&proc);

    Ok(Measurement {
        time,
        memory: peak_memory,
        exit_code: res.code(),
    })
}

fn get_stats(proc: &Child) -> (Result<usize>, Result<Duration>) {
    #[cfg(target_os = "windows")]
    windows::get_stats(proc)
}

fn print_stats(stats: &Measurement) {
    match stats.time {
        Ok(t) => println!("Time: {:?}", t),
        Err(_) => println!("Failed to get time"),
    }

    println!();

    match stats.memory {
        Ok(m) => println!("Memory: {}", get_mem_string(m)),
        Err(_) => println!("Failed to get memory"),
    }

    println!();

    match stats.exit_code {
        Some(e) => println!("Exit code: {}", e),
        None => println!("No exit code"),
    };
}

fn get_mem_string(mem: usize) -> String {
    match mem {
        m if m < 1024 => format!("{} B", mem),
        m if m < 1024 * 1024 => format!("{} KiB", mem as f64 / 1024.),
        m if m < usize::pow(1024, 3) => {
            format!("{:.3} MiB", mem as f64 / f64::powf(1024., 2.))
        }
        m if m < usize::pow(1024, 4) => {
            format!("{:.3} GiB", mem as f64 / f64::powf(1024., 3.))
        }
        m if m < usize::pow(1024, 5) => {
            format!("{:.3} TiB", mem as f64 / f64::powf(1024., 4.))
        }
        m if m < usize::pow(1024, 6) => {
            format!("{:.3} EiB", mem as f64 / f64::powf(1024., 5.))
        }
        _ => format!("{:.3} PiB", mem as f64 / f64::powf(1024., 6.)),
    }
}

fn help() {}