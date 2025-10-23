#![deny(unused_crate_dependencies)]
mod exec;
mod platform;

use std::io::IsTerminal;
use std::path::PathBuf;

use clap::Parser;

use crate::exec::CargoFmtOptions;
use crate::exec::RustfmtOptions;
use crate::exec::RustfmtStdioOptions;

#[derive(Debug, Parser)]
pub struct Command {
    /// Target config to use for formatting
    #[arg(short = 'c', long = "config")]
    pub config: Option<PathBuf>,

    /// Only check formatting
    #[arg(long = "check")]
    pub check: bool,

    #[arg(short = 'f', long = "file")]
    pub files: Vec<PathBuf>,

    #[arg(raw = true)]
    additional_args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().collect::<Vec<String>>();

    // Support for calling "cargo xfmt" and "cargo-xfmt"
    if let Some(arg) = args.get(1)
        && arg == "xfmt"
    {
        args.remove(0);
    }

    // Parse CLI Arguments
    let command = Command::parse_from(&args);

    // If file is passed in through stdin
    if !std::io::stdin().is_terminal() {
        exec::rustfmt_stdio(RustfmtStdioOptions {
            check: command.check,
            additional_args: command.additional_args,
        })
    }
    // If files are specified, use "rustfmt"
    else if !command.files.is_empty() {
        exec::rustfmt(RustfmtOptions {
            check: command.check,
            files: command.files,
            additional_args: command.additional_args,
        })
    }
    // Otherwise use "cargo fmt"
    else {
        exec::cargo_fmt(CargoFmtOptions {
            check: command.check,
            additional_args: command.additional_args,
        })
    }
}
