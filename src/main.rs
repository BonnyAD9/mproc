use cli::print_help;
use cli::Args;
use err::Result;
use measurement::Measurement;
use pareg::Pareg;
use std::process::ExitCode;

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

    let stats = Measurement::measure(program, &args)?;

    args.output.print_measurement(stats, args.color_mode)?;

    Ok(())
}
