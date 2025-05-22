use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Result,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    cli::cmd::CmdHandler,
    config::{self, Config},
};

#[derive(Serialize, Deserialize, Default)]
struct TemplateCacheEntry {
    template_name: String,
    discription: String,
    path: String,
    #[serde(rename = "lastModified")]
    last_modified: u64,
}

#[derive(Serialize, Deserialize, Default)]
struct TemplateCache {
    #[serde(flatten)]
    templates: HashMap<String, Vec<TemplateCacheEntry>>,
}

impl TemplateCache {
    pub fn empty() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }
    // file name: .cache.json
    pub fn load() -> Result<Self> {
        let path = TemplateCache::get_file_path()?;
        if path.metadata()?.len() > 0 {
            let file = File::open(path)?;
            let cache: TemplateCache = serde_json::from_reader(file)?;
            Ok(cache)
        } else {
            Ok(Self::empty())
        }
    }
    fn get_file_path() -> Result<PathBuf> {
        let config_path = config::get_config_dir().expect("Failed to access config dir");
        let path = config_path.join(".cache.json");
        if !path.exists() {
            let _ = File::create(&path);
        }
        Ok(path)
    }
    pub fn save(&self) -> Result<()> {
        let path = TemplateCache::get_file_path()?;
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
    pub fn refresh(&mut self) -> Result<bool> {
        let config_dir = config::get_config_dir()?;
        let mut changed = false;
        let mut seen = HashSet::<(String, String)>::new();
        for entry in fs::read_dir(config_dir)? {
            let path = entry?.path();
            let config_file = path.join("config.toml");
            if !config_file.exists() {
                continue;
            }
            let modified = fs::metadata(&config_file)
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let template_name = path
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .expect("invalid Path")
                .to_string();

            let cfg: Config = Config::parse(&template_name);
            let lang = cfg.language_name.clone();
            let desc = cfg.discription.clone();
            seen.insert((lang.clone(), template_name.clone()));

            let lang_list = self.templates.entry(lang.clone()).or_default();

            match lang_list
                .iter_mut()
                .find(|t| t.template_name == template_name)
            {
                Some(existing) => {
                    if existing.last_modified < modified {
                        existing.discription = desc;
                        existing.last_modified = modified;
                        existing.path = path.clone().to_string_lossy().to_string();
                        changed = true;
                    }
                }
                None => {
                    lang_list.push(TemplateCacheEntry {
                        template_name: template_name.clone(),
                        discription: desc,
                        path: path.clone().to_string_lossy().to_string(),
                        last_modified: modified,
                    });
                    changed = true;
                }
            }
        }
        for (_lang, list) in self.templates.iter_mut() {
            list.retain(|e| {
                let path = Path::new(&e.path);
                if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                    seen.contains(&(e.template_name.clone(), folder_name.to_string()))
                } else {
                    false
                }
            });
        }
        Ok(changed)
    }
}

pub struct CmdList;
impl CmdHandler for CmdList {
    fn new(_cmd: Vec<String>) -> Self {
        // Nothing to do here...
        Self {}
    }
    fn init(&mut self) {
        let mut templates_cache = TemplateCache::load().expect("Unable to load cache");
        let changed = templates_cache.refresh();
        if changed.unwrap() == true {
            let _ = templates_cache.save();
        }
        for (lang, entries) in &templates_cache.templates {
            println!("{}", lang);
            for entry in entries {
                println!(" - {}: {}", entry.template_name, entry.discription);
            }
        }
    }
}
