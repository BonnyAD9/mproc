use cli::print_help;
use cli::Args;
use err::Result;
use measurement::Measurement;
use pareg::Pareg;
use termal::eprintacln;
use std::process::Command;
use std::process::ExitCode;
use std::process::Stdio;
use std::time::Duration;

mod cli;
mod err;
mod measurement;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            e.print();
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    let mut args = Args::parse(Pareg::args())?;
    args.output.validate()?;

    if args.help {
        print_help();
        return Ok(());
    }

    let Some(program) = &args.program else {
        print_help();
        return Ok(());
    };

    if args.repeat == 0 {
        measure_single(program, &args)
    } else {
        measure_multiple(program, &args)
    }
}

pub fn prepare_cmd(program: &str, args: &Args) -> Command {
    let mut cmd = Command::new(program);
    cmd.args(&args.args);

    if args.capture_stdout {
        cmd.stdout(Stdio::null());
    }

    if args.capture_stderr {
        cmd.stderr(Stdio::null());
    }
    cmd
}

fn measure_single(program: &str, args: &Args) -> Result<()> {
    let stats = Measurement::measure(&mut prepare_cmd(program, &args))?;
    args.output.print_measurement(&stats, args.color_mode)
}

fn measure_multiple(program: &str, args: &Args) -> Result<()> {
    let mut cmd = prepare_cmd(program, args);

    let mut measured = 0;
    let mut total_time = Duration::ZERO;
    let mut best_time = Duration::ZERO;
    let mut worst_time = Duration::MAX;
    let mut success = 0;
    let mut failure = 0;
    let mut memory_cnt = 0;
    let mut total_memory = 0;
    let mut best_memory = 0;
    let mut worst_memory = usize::MAX;

    let mut cmd = prepare_cmd(program, args);
    for _ in 0..args.repeat {
        let m = match Measurement::measure(&mut cmd) {
            Ok(m) => m,
            Err(e) => {
                eprintacln!("{e}");
                continue;
            },
        };

        measured += 1;
        total_time += m.time;
        best_time = best_time.min(m.time);
        worst_time = worst_time.max(m.time);

        match m.exit_code {
            Some(0) => success += 1,
            Some(_) => failure += 1,
            _ => {}
        }

        let Ok(m) = m.memory else {
            continue;
        };

        memory_cnt += 1;
        total_memory += m;
        best_memory = best_memory.min(m);
        worst_memory = worst_memory.max(m);
    }

    // TODO

    Ok(())
}
