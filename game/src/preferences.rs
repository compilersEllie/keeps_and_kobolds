use crate::dirs;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Preferences {
    // TODO(feat): Support keybindings
    // TODO(feat): Support Language
    // TODO(feat): Support save files?
    // TODO(feat): Save player name?
}

impl Preferences {
    pub fn prefs_file() -> PathBuf {
        let dirs = crate::dirs();
        let dir = dirs.preference_dir();
        dir.join("prefs.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Preferences::prefs_file();
        let content = read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = Preferences::prefs_file();
        let mut prefs_file = File::create(path)?;
        let prefs = toml::to_string(self)?;
        Ok(prefs_file.write_all(prefs.as_bytes())?)
    }
}
