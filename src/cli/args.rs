use pareg::{ArgErrCtx, ArgError, Pareg, Result, starts_any};

use super::{ColorMode, OutputType, print_help, print_version};

#[derive(Debug, Default)]
pub struct Args {
    pub program: Option<String>,
    pub args: Vec<String>,
    pub output: OutputType,
    pub helped: bool,
    pub color_mode: ColorMode,
    pub capture_stdout: bool,
    pub capture_stderr: bool,
    pub repeat: usize,
}

impl Args {
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut res = Args::default();

        while let Some(arg) = args.next() {
            match arg {
                "-h" | "--help" | "-?" => {
                    res.helped = true;
                    print_help(res.color_mode.stdout());
                }
                "--version" => {
                    res.helped = true;
                    print_version();
                }
                "-o" | "--out" | "--output" => {
                    res.output = OutputType::FilePath(args.next_arg()?)
                }
                "--stdout" => res.output = OutputType::Stdout,
                "--stderr" => res.output = OutputType::Stderr,
                "--color" | "--colour" => {
                    res.color_mode = args.next_arg()?;
                }
                v if starts_any!(v, "--color=", "--colour=") => {
                    res.color_mode = args.cur_val('=')?;
                }
                "-c" | "--cout" | "--capture-stdout" => {
                    res.capture_stdout = true;
                }
                "--cerr" | "--capture-stderr" => {
                    res.capture_stderr = true;
                }
                "-cc" | "--capture-all" => {
                    res.capture_stderr = true;
                    res.capture_stdout = true;
                }
                "-r" | "--repeat" => {
                    res.repeat = args.next_arg()?;
                    args.cur_manual(|a| {
                        if res.repeat == 0 {
                            Err(ArgError::FailedToParse(Box::new(
                                ArgErrCtx::from_msg(
                                    "Invalid value.".into(),
                                    a.to_owned(),
                                )
                                .hint("Value must be positive."),
                            )))
                        } else {
                            Ok(())
                        }
                    })?;
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
