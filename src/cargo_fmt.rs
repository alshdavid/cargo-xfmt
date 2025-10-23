use std::process;

use crate::rustfmt_toml::RustfmtToml;
use crate::vec_ext::split_at_first_occurrence;

pub struct CargofmtOptions {
    pub check: bool,
    pub additional_args: Vec<String>,
}

pub fn cargo_fmt(options: CargofmtOptions) -> anyhow::Result<()> {
    let rustfmt_toml = RustfmtToml::read_from(&std::env::current_dir()?)?;
    let (additional_cargo_args, additional_rustfmt_args) =
        split_at_first_occurrence(options.additional_args, &"--".to_string());

    let mut cmd = process::Command::new("cargo");

    cmd.arg("fmt".to_string());

    if options.check {
        cmd.arg("--check".to_string());
    }

    for arg in &additional_cargo_args {
        cmd.arg(arg.to_string());
    }

    cmd.arg("--".to_string());

    for (key, value) in rustfmt_toml.iter() {
        cmd.arg("--config".to_string());
        cmd.arg(format!("{}={}", key, value));
    }

    for arg in &additional_rustfmt_args {
        cmd.arg(arg.to_string());
    }

    cmd.stdin(process::Stdio::inherit());
    cmd.stdout(process::Stdio::inherit());
    cmd.stderr(process::Stdio::inherit());

    rustfmt_toml.hide()?;
    let mut child = cmd.spawn()?;

    let status = child.wait()?;

    rustfmt_toml.unhide()?;
    std::process::exit(status.code().unwrap_or(1))
}
