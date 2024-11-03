use pareg::{starts_any, Pareg, Result};

use crate::{color_mode::ColorMode, output::Output};

#[derive(Debug, Default)]
pub struct Args {
    pub program: Option<String>,
    pub args: Vec<String>,
    pub output: Output,
    pub help: bool,
    pub color_mode: ColorMode,
}

impl Args {
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut res = Args::default();

        while let Some(arg) = args.next() {
            match arg {
                "-h" | "--help" | "-?" => res.help = true,
                "-o" | "--out" | "--output" => {
                    res.output = Output::FilePath(args.next_arg()?)
                }
                "--stdout" => res.output = Output::Stdout,
                "--stderr" => res.output = Output::Stderr,
                "--color" | "--colour" => {
                    res.color_mode = args.next_arg()?;
                }
                v if starts_any!(v, "--color=", "--colour=") => {
                    res.color_mode = args.cur_val('=')?;
                }
                "--" => {
                    res.program = args.next().map(ToString::to_string);
                    while let Some(arg) = args.next() {
                        res.args.push(arg.to_string());
                    }
                }
                a if a.starts_with('-') => {
                    let hint = format!(
                        "Use `--` to run program with the name `{a}`."
                    );
                    return Err(args.err_unknown_argument().hint(hint));
                }
                _ => {
                    res.program = Some(args.cur_arg()?);
                    while let Some(arg) = args.next() {
                        res.args.push(arg.to_string());
                    }
                }
            }
        }

        Ok(res)
    }
}
