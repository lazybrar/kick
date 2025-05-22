use crate::io;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "name")]
    pub language_name: String,
    #[serde(default)]
    pub discription: String,
    #[serde(rename = "setup", default)]
    pub setup_cmd: String,
    #[serde(rename = "variables", default)]
    pub variables: HashMap<String, String>,
}

impl Config {
    pub fn list() -> Vec<String> {
        let config_dir = get_config_dir().unwrap();
        match list_dirs_with_config_file(&config_dir) {
            Ok(result) => {
                return result;
            }
            Err(_) => {
                return Vec::<String>::new();
            }
        };
    }

    pub fn empty() -> Config {
        Self {
            language_name: String::new(),
            setup_cmd: String::new(),
            variables: HashMap::new(),
            discription: String::new(),
        }
    }

    pub fn parse(name: &String) -> Config {
        let config_dir = get_config_dir().unwrap();
        let template_dir = config_dir.join(name);
        // checked aleardy handled by Config.list()
        let config_file_content =
            fs::read_to_string(template_dir.join("config.toml")).expect("Expected config.toml");
        let config: Config =
            toml::from_str(&config_file_content).expect("Failed to parse config.toml");
        return config;
    }
}

// Get config directory top handle tempates
// ~/.config/kick/
pub fn get_config_dir() -> Result<PathBuf, io::Error> {
    let root_config_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Couldn't find config directory"));

    let config_dir = root_config_dir.unwrap().join("kick");

    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    } else if !config_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Config path {:?} exists but is not a directory", config_dir),
        ));
    }
    Ok(config_dir)
}

// Using simplest way, just go into each dir and check for config.toml
fn list_dirs_with_config_file(config_dir: &PathBuf) -> Result<Vec<String>, io::Error> {
    let mut dirs: Vec<String> = Vec::new();
    let entries = fs::read_dir(config_dir)?;

    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(_) => {
                //skip if can't get file info
                continue;
            }
        };
        let file_type = match entry.file_type() {
            Ok(result) => result,
            Err(_) => {
                // skip if can't get file type
                continue;
            }
        };
        let path = entry.path();
        if file_type.is_dir() {
            let config_file_path = path.join("config.toml");
            match config_file_path.metadata() {
                Ok(meta) => {
                    if meta.is_file() {
                        let dir_name = entry.file_name().to_string_lossy().into_owned();
                        dirs.push(dir_name);
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        };
    }
    Ok(dirs)
}
