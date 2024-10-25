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
"
    )
}
