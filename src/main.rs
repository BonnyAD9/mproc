use args::Args;
use eyre::Result;
use help::print_help;
use measurement::Measurement;
use pareg::Pareg;
use std::process::ExitCode;

mod args;
mod help;
mod measurement;
mod output;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    let args = Args::parse(Pareg::args())?;
    args.output.validate()?;

    if args.help {
        print_help();
        return Ok(());
    }

    let Some(program) = args.program else {
        print_help();
        return Ok(());
    };

    let stats = Measurement::measure(&program, &args.args)?;

    args.output.print_measurement(stats)?;

    Ok(())
}
