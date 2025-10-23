use std::process;

use crate::platform::rustfmt_toml::RustfmtToml;
use crate::platform::vec_ext::split_at_first_occurrence;

pub struct CargoFmtOptions {
    pub check: bool,
    pub additional_args: Vec<String>,
}

pub fn cargo_fmt(options: CargoFmtOptions) -> anyhow::Result<()> {
    let rustfmt_toml = RustfmtToml::read_from(&std::env::current_dir()?)?;
    let (additional_cargo_args, additional_rustfmt_args) =
        split_at_first_occurrence(options.additional_args, &"--".to_string());

    let mut cmd = process::Command::new("cargo");

    cmd.arg("fmt");

    if options.check {
        cmd.arg("--check");
    }

    for arg in &additional_cargo_args {
        cmd.arg(arg);
    }

    cmd.arg("--");

    for (key, value) in rustfmt_toml.iter() {
        cmd.arg("--config");
        cmd.arg(format!("{}={}", key, value));
    }

    for arg in &additional_rustfmt_args {
        cmd.arg(arg);
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
