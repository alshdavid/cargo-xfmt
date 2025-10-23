use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{self};

use crate::cargo_toml::CargoToml;
use crate::path_ext::path_to_absolute;
use crate::rustfmt_toml::RustfmtToml;

pub struct RustfmtOptions {
    pub check: bool,
    pub files: Vec<PathBuf>,
    pub additional_args: Vec<String>,
}

pub fn rustfmt(options: RustfmtOptions) -> anyhow::Result<()> {
    for file in options.files.clone() {
        rustfmt_one(file, &options)?;
    }
    Ok(())
}

fn rustfmt_one(
    file: PathBuf,
    options: &RustfmtOptions,
) -> anyhow::Result<()> {
    let cargo_toml = CargoToml::read_from(&file)?;
    let rustfmt_toml = RustfmtToml::read_from(&file)?;

    let mut cmd = process::Command::new("rustfmt");

    if options.check {
        cmd.arg("--check".to_string());
    }

    if let Some(edition) = cargo_toml.edition {
        cmd.arg("--edition".to_string());
        cmd.arg(edition.clone());
    }

    for (key, value) in rustfmt_toml.iter() {
        cmd.arg("--config".to_string());
        cmd.arg(format!("{}={}", key, value));
    }

    for arg in &options.additional_args {
        cmd.arg(arg.to_string());
    }

    cmd.stdin(process::Stdio::piped());
    cmd.stdout(process::Stdio::piped());
    cmd.stderr(process::Stdio::inherit());

    rustfmt_toml.hide()?;
    let mut child = cmd.spawn()?;

    let input_path = path_to_absolute(&file)?;
    let input = std::fs::read_to_string(&input_path)?;

    let Some(mut stdin) = child.stdin.take() else {
        return Err(anyhow::anyhow!("Unable to get stdin for rustfmt"));
    };

    stdin.write_all(input.as_bytes())?;
    drop(stdin);

    let result = child.wait_with_output()?;
    if result.status.success() {
        fs::write(&input_path, result.stdout)?;
    }

    rustfmt_toml.unhide()?;
    std::process::exit(result.status.code().unwrap_or(1))
}
