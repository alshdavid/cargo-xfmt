#![deny(unused_crate_dependencies)]
mod cargo_fmt;
mod cargo_toml;
mod os_string_ext;
mod path_ext;
mod rustfmt;
mod rustfmt_stdio;
mod rustfmt_toml;
mod vec_ext;

use std::io::IsTerminal;
use std::path::PathBuf;

use clap::Parser;

use crate::cargo_fmt::CargofmtOptions;
use crate::rustfmt::RustfmtOptions;
use crate::rustfmt_stdio::RustfmtStdioOptions;

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
    let mut args = std::env::args().into_iter().collect::<Vec<String>>();

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
        return rustfmt_stdio::rustfmt_stdio(RustfmtStdioOptions {
            check: command.check,
            additional_args: command.additional_args,
        });
    }
    // If files are specified, use "rustfmt"
    else if command.files.len() > 0 {
        return rustfmt::rustfmt(RustfmtOptions {
            check: command.check,
            files: command.files,
            additional_args: command.additional_args,
        });
    }
    // Otherwise use "cargo fmt"
    else {
        return cargo_fmt::cargo_fmt(CargofmtOptions {
            check: command.check,
            additional_args: command.additional_args,
        });
    }
}
