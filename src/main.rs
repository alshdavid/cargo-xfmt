#![deny(unused_crate_dependencies)]
mod exec;
mod platform;

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

  /// Do not rename config to hide it
  #[arg(long = "preserve-config")]
  pub preserve_config: bool,

  /// Read source code from stdin and write formatted output to stdout
  #[arg(long = "stdin")]
  pub stdin: bool,

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

  // If --stdin is passed, read from stdin and format
  if command.stdin {
    exec::rustfmt_stdio(RustfmtStdioOptions {
      check: command.check,
      preserve_config: command.preserve_config,
      additional_args: command.additional_args,
    })
  }
  // If files are specified, use "rustfmt"
  else if !command.files.is_empty() {
    exec::rustfmt(RustfmtOptions {
      check: command.check,
      files: command.files,
      preserve_config: command.preserve_config,
      additional_args: command.additional_args,
    })
  }
  // Otherwise use "cargo fmt"
  else {
    exec::cargo_fmt(CargoFmtOptions {
      check: command.check,
      preserve_config: command.preserve_config,
      additional_args: command.additional_args,
    })
  }
}
