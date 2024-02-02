use eyre::Result;
use std::{
    env,
    io::{stderr, stdout, IsTerminal},
    process::{Child, Command},
    time::Duration,
};
use termal::{eprintmcln, gradient, printmcln};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

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

    get_stats(&mut proc)
}

fn get_stats(proc: &mut Child) -> Result<Measurement> {
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

fn print_stats(stats: &Measurement) {
    let color = stderr().is_terminal();

    eprintmcln!(
        color,
        "===============<< {'y}mproc results{'_} >>==============="
    );

    match stats.time {
        Ok(t) => eprintln!("Time: {:?}", t),
        Err(_) => eprintln!("Failed to get time"),
    }

    eprintln!();

    match stats.memory {
        Ok(m) => eprintln!("Memory: {}", get_mem_string(m)),
        Err(_) => eprintln!("Failed to get memory"),
    }

    eprintln!();

    match stats.exit_code {
        Some(e) => eprintln!("Exit code: {}", e),
        None => eprintln!("No exit code"),
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

fn help() {
    printmcln!(
        stdout().is_terminal(),
        "Welcome in {'g i}mproc{'_} by {}{'_}

{'g}Usage:
  {'w}mproc [program] {'gr}[arguments]{'_}
",
        gradient("BonnyAD9", (250, 50, 170), (180, 50, 240))
    );
}
