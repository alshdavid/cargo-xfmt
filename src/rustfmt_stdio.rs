use std::io::Read;
use std::io::Write;
use std::process::{self};

use crate::cargo_toml::CargoToml;
use crate::rustfmt_toml::RustfmtToml;

pub struct RustfmtStdioOptions {
    pub check: bool,
    pub additional_args: Vec<String>,
}

pub fn rustfmt_stdio(options: RustfmtStdioOptions) -> anyhow::Result<()> {
    let cargo_toml = CargoToml::read_from(&std::env::current_dir()?)?;
    let rustfmt_toml = RustfmtToml::read_from(&std::env::current_dir()?)?;

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
    cmd.stdout(process::Stdio::inherit());
    cmd.stderr(process::Stdio::inherit());

    rustfmt_toml.hide()?;
    let mut child = cmd.spawn()?;

    let input = {
        let mut stdin = std::io::stdin();
        let mut buf = vec![];
        stdin.read_to_end(&mut buf)?;
        String::from_utf8(buf)?
    };

    let Some(mut stdin) = child.stdin.take() else {
        return Err(anyhow::anyhow!("Unable to get stdin for rustfmt"));
    };

    stdin.write_all(input.as_bytes())?;
    drop(stdin);

    let status = child.wait()?;

    rustfmt_toml.unhide()?;
    std::process::exit(status.code().unwrap_or(1))
}
