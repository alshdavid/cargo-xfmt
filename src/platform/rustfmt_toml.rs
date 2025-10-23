use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

use parking_lot::Mutex;

use crate::platform::path_ext::find_ancestor_file;
use crate::platform::path_ext::path_to_absolute;

pub type RustfmtOptions = Vec<(String, String)>;

pub struct RustfmtToml {
    hidden: Mutex<bool>,
    config_dir: PathBuf,
    config_path: PathBuf,
    config: RustfmtOptions,
}

impl RustfmtToml {
    pub fn read_from(start_from: &Path) -> anyhow::Result<Self> {
        let Some(config_path) = Self::find(start_from)? else {
            return Err(anyhow::anyhow!("Unable to find Cargo.toml"));
        };

        Self::read(&config_path)
    }

    pub fn find(start_from: &Path) -> anyhow::Result<Option<PathBuf>> {
        let mut results = find_ancestor_file(start_from, ".rustfmt.toml")?;
        if let Some(result) = results.pop() {
            return Ok(Some(path_to_absolute(&result)?));
        }

        let mut results = find_ancestor_file(start_from, "rustfmt.toml")?;
        if let Some(result) = results.pop() {
            return Ok(Some(path_to_absolute(&result)?));
        }

        Ok(None)
    }

    pub fn read(config_path: &Path) -> anyhow::Result<Self> {
        let config_str = std::fs::read_to_string(config_path)?;
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

        let Some(config_dir) = config_path.parent() else {
            return Err(anyhow::anyhow!("Unable to find parent of config file"));
        };

        Ok(Self {
            hidden: Mutex::new(false),
            config_dir: config_dir.to_path_buf(),
            config_path: config_path.to_path_buf(),
            config: fmt_options,
        })
    }

    /// rustfmt will automatically detect the rustfmt file and will throw unless nightly is used
    pub fn hide(&self) -> anyhow::Result<()> {
        let mut hidden = self.hidden.lock();

        std::fs::rename(&self.config_path, self.config_dir.join("_rustfmt.toml"))?;

        (*hidden) = true;

        Ok(())
    }

    pub fn unhide(&self) -> anyhow::Result<()> {
        let mut hidden = self.hidden.lock();

        std::fs::rename(self.config_dir.join("_rustfmt.toml"), &self.config_path)?;

        (*hidden) = false;

        Ok(())
    }
}

impl Deref for RustfmtToml {
    type Target = RustfmtOptions;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl Drop for RustfmtToml {
    fn drop(&mut self) {
        drop(self.unhide())
    }
}
