use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use crate::path_ext::find_ancestor_file;
use crate::path_ext::path_to_absolute;

pub struct CargoToml {
    pub edition: String,
}

impl CargoToml {
    pub fn read_from(start_from: &Path) -> anyhow::Result<Self> {
        let Some(cargo_toml_path) = Self::find(start_from)? else {
            return Err(anyhow::anyhow!("Unable to find Cargo.toml"));
        };

        Self::read(&cargo_toml_path)
    }

    pub fn find(start_from: &Path) -> anyhow::Result<Option<PathBuf>> {
        let mut results = find_ancestor_file(start_from, "Cargo.toml")?;
        if let Some(result) = results.pop() {
            return Ok(Some(path_to_absolute(&result)?));
        }

        Ok(None)
    }

    pub fn read(cargo_toml_path: &Path) -> anyhow::Result<Self> {
        let cargo_toml_str = std::fs::read_to_string(&cargo_toml_path)?;
        let cargo_toml = toml::from_str::<HashMap<String, toml::Value>>(&cargo_toml_str)?;
        let Some(toml::Value::Table(package)) = cargo_toml.get("package") else {
            return Err(anyhow::anyhow!("Cargo.toml missing 'package'"));
        };
        let Some(toml::Value::String(edition)) = package.get("edition") else {
            return Err(anyhow::anyhow!("Cargo.toml missing 'package.edition'"));
        };

        Ok(Self {
            edition: edition.to_string(),
        })
    }
}
