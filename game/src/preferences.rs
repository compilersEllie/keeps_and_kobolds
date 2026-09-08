use crate::dirs;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Preferences {
    // TODO(feat): Support keybindings #3
    // TODO(feat): Support Language #4
    // TODO(feat): Support save files? #2
    // TODO(feat): Save player name? #2
}

impl Preferences {
    pub fn prefs_file() -> PathBuf {
        let dirs = crate::dirs();
        let dir = dirs.preference_dir();
        dir.join("prefs.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Preferences::prefs_file();
        let content = read_to_string(path);
        match content {
            Ok(content) => {
                // TODO(io): Handle errors #2
                Ok(toml::from_str(&content)?)
            }
            Err(err) => {
                log::warn!("{}", err);
                Ok(Preferences::default())
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Preferences::prefs_file();
        // TODO(io): Handle file create errors #2
        let mut prefs_file = File::create(path)?;
        // TODO(io): Handle serialize errors #2
        let prefs = toml::to_string(self)?;
        // TODO(io): Handle write errors #2
        Ok(prefs_file.write_all(prefs.as_bytes())?)
    }
}
