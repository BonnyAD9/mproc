use pareg::{Pareg, Result};

#[derive(Debug, Default)]
pub struct Args {
    pub program: Option<String>,
    pub args: Vec<String>,
    pub help: bool,
}

impl Args {
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut res = Args::default();

        while let Some(arg) = args.next() {
            match arg {
                "-h" | "--help" | "-?" => res.help = true,
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
