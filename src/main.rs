use cli::{print_help, Args, Output};
use com_measure::ComMeasure;
use err::Result;
use measurement::Measurement;
use pareg::Pareg;
use std::process::{Command, ExitCode, Stdio};

mod cli;
mod com_measure;
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

    if args.program.is_none() {
        print_help();
        return Ok(());
    };

    if args.repeat == 0 {
        measure_single(args)
    } else {
        measure_multiple(args)
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

pub fn get_mem_string(mem: usize) -> String {
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

fn measure_single(args: Args) -> Result<()> {
    let program = args.program.as_deref().unwrap_or_default();
    let mut cmd = prepare_cmd(program, &args);
    let mut output = Output::new(args.output, args.color_mode);

    let stats = Measurement::measure(&mut cmd)?;
    output.print_measurement(&stats)
}

fn measure_multiple(args: Args) -> Result<()> {
    let program = args.program.as_deref().unwrap_or_default();
    let cmd = prepare_cmd(program, &args);
    let mut output = Output::new(args.output, args.color_mode);

    let mut stats = ComMeasure::new(cmd);
    for i in 0..args.repeat {
        _ = output.print_res_with(i + 1, stats.measure());
    }

    output.print_com_measure(&stats)
}
