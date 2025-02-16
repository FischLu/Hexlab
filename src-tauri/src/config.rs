use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::{format::FormatRadix, options::Options};
use anyhow::Result as AResult;
use colored::Colorize;
use getset::{Getters, Setters};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Getters, PartialEq, Eq, Setters)]
#[getset(get = "pub")]
pub struct Config {
    #[serde(default = "default_prompt")]
    prompt: String,

    #[serde(default = "default_header")]
    header: bool,

    #[serde(default = "default_history")]
    history: bool,

    #[serde(default)]
    output_radix: FormatRadix,

    #[serde(default)]
    punctuate_output: bool,

    #[serde(default = "default_mode")]
    #[getset(set = "pub")]
    mode: String,
}

impl Config {
    pub fn override_from_options(&mut self, options: &Options) {
        if options.punctuate_output {
            self.punctuate_output = true;
        }

        if options.hex {
            self.output_radix = FormatRadix::Hex;
        } else if options.dec {
            self.output_radix = FormatRadix::Decimal;
        } else if options.oct {
            self.output_radix = FormatRadix::Octal;
        } else if options.bin {
            self.output_radix = FormatRadix::Binary;
        }

        if options.history {
            self.history = true;
        }

        self.mode = options.mode.clone();
    }

    #[allow(dead_code)]
    pub fn new() -> Config {
        Config{
            prompt: "cork> ".yellow().to_string(),
            header: true,
            history: false,
            output_radix: FormatRadix::Hex,
            punctuate_output: false,
            mode: "hex".to_string()
        }
    }
}

fn default_prompt() -> String {
    "cork> ".yellow().to_string()
}

fn default_header() -> bool {
    true
}

fn default_history() -> bool {
    false
}

fn default_mode() -> String {
    "hex".to_string()
}

fn config_locations() -> Vec<PathBuf> {
    match home::home_dir() {
        Some(home) => {
            let mut at_home = home.clone();
            at_home.push(".cork.yml");
            let mut at_cork = home.clone();
            at_cork.push(".cork");
            at_cork.push("cork.yml");
            let mut at_config = home;
            at_config.push(".config");
            at_config.push("cork");
            at_config.push("cork.yml");
            vec![at_home, at_cork, at_config]
        }
        None => Vec::new(),
    }
}

pub fn read_config<T: AsRef<Path>>(user_path: Option<T>) -> AResult<Config> {
    let mut content = String::new();
    if let Some(user_path) = user_path {
        let mut file = File::open(user_path)?;
        file.read_to_string(&mut content)?;
    } else {
        let locations = config_locations();
        for loc in &locations {
            if loc.exists() && loc.is_file() {
                let mut file = File::open(loc)?;
                file.read_to_string(&mut content)?;
            }
        }
    }
    if content.is_empty() {
        content = String::from("")
    }
    let config = serde_yaml::from_str(&content)?;
    Ok(config)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_deserialize() {
        let config_str = "prompt: $
header: false
history: true
output_radix: Octal
mode: dec
punctuate_output: true";
        let config: Config = serde_yaml::from_str(config_str).unwrap();
        let expected_config = Config {
            prompt: String::from("$"),
            header: false,
            history: true,
            output_radix: FormatRadix::Octal,
            punctuate_output: true,
            mode: String::from("dec"),
        };
        assert_eq!(config, expected_config);
    }

    #[test]
    fn test_config_deserialize_missing_values() {
        let config_str = "prompt: $
output_radix: Octal";
        let config: Config = serde_yaml::from_str(config_str).unwrap();
        let expected_config = Config {
            prompt: String::from("$"),
            header: default_header(),
            history: default_history(),
            output_radix: FormatRadix::Octal,
            punctuate_output: false,
            mode: String::from("hex"),
        };
        assert_eq!(config, expected_config);
    }

    #[test]
    fn test_config_deserialize_empty() {
        let config_str = "";
        let config: Config = serde_yaml::from_str(config_str).unwrap();
        let expected_config = Config {
            prompt: default_prompt(),
            header: default_header(),
            history: default_history(),
            output_radix: FormatRadix::default(),
            punctuate_output: false,
            mode: String::from("hex"),
        };
        assert_eq!(config, expected_config);
    }
}
