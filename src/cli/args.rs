use pareg::{Pareg, Result, check::InRangeI, has_any_key};

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
                v if has_any_key!(v, '=', "--color", "--colour") => {
                    res.color_mode = args.cur_val_or_next('=')?;
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
                    res.repeat = args.next_arg::<
                        InRangeI<_, 1, { usize::MAX as i128 + 1 }>
                    >()?.0;
                }
                "--" => {
                    res.program = args.next().map(str::to_string);
                    res.args.extend(
                        args.remaining().iter().map(|a| a.to_string()),
                    );
                }
                a if a.starts_with('-') => {
                    let hint = format!(
                        "Use `--` to run program with the name `{a}`."
                    );
                    return args.err_unknown_argument().hint(hint).err();
                }
                _ => {
                    res.program = Some(args.cur_arg()?);
                    res.args.extend(
                        args.remaining().iter().map(|a| a.to_string()),
                    );
                }
            }
        }

        Ok(res)
    }
}
