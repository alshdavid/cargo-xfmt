use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::{self};

use clap::Parser;
use normalize_path::NormalizePath;

#[derive(Debug, Parser)]
pub struct Command {
    /// Target config to use for formatting
    #[arg(long = "config")]
    pub config: Option<PathBuf>,

    /// Only check formatting
    #[arg(long = "check")]
    pub check: bool,

    #[arg(raw = true)]
    remaining: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().into_iter().collect::<Vec<String>>();
    if let Some(arg) = args.get(1)
        && arg == "xfmt"
    {
        args.remove(0);
    }

    let command = Command::parse_from(&args);

    let Ok(cwd) = env::current_dir() else {
        return Err(anyhow::anyhow!("Unable to get cwd"));
    };

    let config_path = {
        let config_path = match command.config {
            Some(config_path) => config_path,
            None => {
                if std::fs::exists(cwd.join(".rustfmt.toml"))? {
                    cwd.join(".rustfmt.toml")
                } else if std::fs::exists(cwd.join("rustfmt.toml"))? {
                    cwd.join("rustfmt.toml")
                } else {
                    return Err(anyhow::anyhow!("Unable to find rust format config"));
                }
            }
        };
        if config_path.is_absolute() {
            config_path.normalize()
        } else {
            cwd.join(&config_path).normalize()
        }
    };

    if !std::fs::exists(&config_path)? {
        return Err(anyhow::anyhow!("Unable to find rust format config"));
    }

    let config_str = std::fs::read_to_string(&config_path)?;
    let config = toml::from_str::<HashMap<String, toml::Value>>(&config_str)?;

    let mut fmt_options = Vec::<(String, String)>::new();

    for (key, value) in config {
        let value = match value {
            toml::Value::String(v) => v.to_string(),
            toml::Value::Integer(v) => v.to_string(),
            toml::Value::Float(v) => v.to_string(),
            toml::Value::Boolean(v) => v.to_string(),
            toml::Value::Datetime(v) => v.to_string(),
            toml::Value::Array(_v) => {
                return Err(anyhow::anyhow!("Unsupported config type: Array"));
            }
            toml::Value::Table(_v) => {
                return Err(anyhow::anyhow!("Unsupported config type: Table"));
            }
        };
        fmt_options.push((key, value));
    }

    let mut args = Vec::<String>::new();

    args.push("fmt".to_string());
    if command.check {
        args.push("--check".to_string());
    }

    for arg in command.remaining {
        args.push(arg);
    }

    args.push("--".to_string());

    for (key, value) in fmt_options {
        args.push("--config".to_string());
        args.push(format!("{}={}", key, value));
    }

    // Temporarily move config
    if config_path.ends_with(".rustfmt.toml") || config_path.ends_with("rustfmt.toml") {
        std::fs::rename(
            &config_path,
            config_path.parent().unwrap().join("_rustfmt.toml"),
        )?
    }

    let mut cmd = process::Command::new("cargo");
    cmd.args(args);

    cmd.stdin(process::Stdio::inherit());
    cmd.stdout(process::Stdio::inherit());
    cmd.stderr(process::Stdio::inherit());

    let mut status_code = None::<i32>;
    if let Ok(mut child) = cmd.spawn() {
        status_code = child.wait()?.code();
    };

    if config_path.ends_with(".rustfmt.toml") || config_path.ends_with("rustfmt.toml") {
        std::fs::rename(
            config_path.parent().unwrap().join("_rustfmt.toml"),
            config_path,
        )?
    }

    if let Some(exit_code) = status_code {
        std::process::exit(exit_code);
    }

    Err(anyhow::anyhow!("Child process failed"))
}
