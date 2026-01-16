use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub lastuser: Option<String>,
    pub quiet: bool,
    pub verbose: bool,
    pub nocolor: bool,
    pub editor: Option<String>,
    pub org: Option<String>,
}

impl Config {
    #[must_use]
    pub fn path() -> PathBuf {
        PathBuf::from(".").join("log.toml")
    }

    #[must_use]
    pub fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| toml::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "quiet" => Some(self.quiet.to_string()),
            "verbose" => Some(self.verbose.to_string()),
            "nocolor" => Some(self.nocolor.to_string()),
            "editor" => self.editor.clone(),
            "org" => self.org.clone(),
            "lastuser" => self.lastuser.clone(),
            _ => None,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "quiet" => self.quiet = value == "true" || value == "1",
            "verbose" => self.verbose = value == "true" || value == "1",
            "nocolor" => self.nocolor = value == "true" || value == "1",
            "editor" => self.editor = Some(value.to_string()),
            "org" => self.org = Some(value.to_string()),
            _ => anyhow::bail!("Unknown setting: {key}"),
        }
        self.save()
    }
}

// global flags
static mut QUIET: bool = false;
static mut VERBOSE: bool = false;
static mut NOCOLOR: bool = false;

pub fn setquiet(q: bool) {
    unsafe {
        QUIET = q;
    }
}

#[must_use]
pub fn isquiet() -> bool {
    unsafe { QUIET }
}

pub fn setverbose(v: bool) {
    unsafe {
        VERBOSE = v;
    }
}

#[must_use]
pub fn isverbose() -> bool {
    unsafe { VERBOSE }
}

pub fn setnocolor(c: bool) {
    unsafe {
        NOCOLOR = c;
    }
}

#[must_use]
pub fn isnocolor() -> bool {
    unsafe { NOCOLOR }
}

/// Check if this is the first run
#[must_use]
pub fn isfirstrun() -> bool {
    !Config::path().exists()
}
