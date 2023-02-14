use eyre::Result;
use std::{
    env,
    process::{Child, Command},
    time::Duration,
};

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
        let ec = proc.wait()?;
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
    println!(
        "===============<< {yellow}mproc results{reset} >>===============",
        yellow = "\x1b[93m",
        reset = "\x1b[0m"
    );

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

fn help() {
    println!(
        "Welcome in {g}{i}mproc{r} by {}{}{}

{g}Usage:{r} {w}mproc{r} {w}[program]{r} {d}[arguments]{r}",
        // BonnyAD9 gradient in 3 strings
        "\x1b[38;2;250;50;170mB\x1b[38;2;240;50;180mo\x1b[38;2;230;50;190mn",
        "\x1b[38;2;220;50;200mn\x1b[38;2;210;50;210my\x1b[38;2;200;50;220mA",
        "\x1b[38;2;190;50;230mD\x1b[38;2;180;50;240m9\x1b[0m",
        g = "\x1b[92m", // green
        i = "\x1b[23m", // italic
        r = "\x1b[0m",  // reset
        w = "\x1b[97m", // white
        d = "\x1b[90m"  // dark gray
    );
}
