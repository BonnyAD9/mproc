use std::{
    borrow::Cow,
    io::{stdout, IsTerminal},
};

use termal::{gradient, printmcln};

pub fn print_help() {
    let color = stdout().is_terminal();
    let v = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let signature: Cow<str> = if color {
        gradient("BonnyAD9", (250, 50, 170), (180, 50, 240)).into()
    } else {
        "BonnyAD9".into()
    };

    printmcln!(
        color,
        "Welcome to {'i g}mproc{'_} help by {signature}{'_}
Version: {v}

{'g}Usage:
  {'c}mproc{'_}
    Show this help.

  {'c}mproc {'gr}[flags] [--] {'w}<program> {'gr}[program-args]{'_}
    Run the given program with the given arguments and measure its run time and
    peak memory.

{'g}Flags:
  {'y}-h  -?  --help{'_}
    Print this help.

  {'y}-o  --out  --output {'w}<path>{'_}
    Output to the given file instead of stderr.

  {'y}--stdout{'_}
    Output to stdout instead of stderr.

  {'y}--stderr{'_}
    Output to stderr. This is the default.

  {'y}--color  --colour {'w}auto|always|never
  {'y}--color  --colour{'w}=auto|always|never
    Set the color mode. This will enable/disable color or automatically choose
    color when the output is terminal. This is `{'i}auto{'_}` by default.
"
    )
}
