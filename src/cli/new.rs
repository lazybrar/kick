use minijinja::Environment;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::Path;

use crate::cli::cmd::CmdHandler;
use crate::config;
use crate::config::Config;
use crate::utils;

fn error(msg: &str) {
    eprintln!("{}", msg);
    std::process::exit(1);
}

fn no_template_found_error(lang_name: &str) {
    eprintln!("template not found: {}", lang_name);
    std::process::exit(1);
}

pub struct CmdNew {
    args: Vec<String>,
    config: Config,
    project_name: String,
    template_name: String,
}
impl CmdHandler for CmdNew {
    fn new(mut arg_list: Vec<String>) -> Self {
        // temporary args
        let mut temp_config: Config = Config::empty();
        let mut temp_project_name: String = String::new();
        let mut temp_template_name: String = String::new();

        if let Some(template_name) = arg_list.get(1) {
            temp_template_name = template_name.to_string();
            if Config::list().contains(template_name) {
                temp_config = Config::parse(template_name);
                // remove lang name from args
                arg_list.remove(1);
                if let Some(project_name) = arg_list.get(1) {
                    // To check if project name is valid
                    // we will create th dir as we also need it in init func
                    if !Path::new(project_name).exists() {
                        fs::create_dir(project_name).expect("Cannot create directory");
                        temp_project_name = project_name.to_string();
                    } else {
                        error("directory or file alerdy exits with same name");
                    }
                }
            } else {
                no_template_found_error(template_name);
            }
        } else {
            error("pass language name");
        }
        Self {
            args: arg_list,
            config: temp_config,
            project_name: temp_project_name,
            template_name: temp_template_name,
        }
    }

    fn init(&mut self) {
        // parse all args and put the vals in hashmap
        collect_args(&mut self.config.variables, &self.args);
        // also include some custom stuff
        self.config
            .variables
            .insert(String::from("projectName"), self.project_name.clone());
        // copy all files from template and paste it in target dir while replacing vars
        let config_dir = config::get_config_dir().expect("Config dir not found");
        let template_dir = config_dir.join(self.template_name.clone());
        // from: template dir to project dir
        copy_file_while_replacing(
            &template_dir,
            &Path::new(&self.project_name),
            &self.config.variables,
        )
        .expect("Unable to create template");
        //lastly execute setup_cmd if exists
        let _ = utils::exec_cmd(self.config.setup_cmd.clone());
    }
}

/*
take arguments as args and put them to hashmap
valid formats
1) --name=value
2) --name value
*/
fn collect_args(vars: &mut HashMap<String, String>, args: &Vec<String>) {
    let mut i = 0;
    while i < args.len() {
        if let Some(arg) = args[i].strip_prefix("--") {
            if let Some((k, v)) = arg.split_once("=") {
                if vars.contains_key(k) {
                    vars.insert(k.to_string(), v.to_string());
                }
            } else if i + 1 < args.len() {
                let k = arg;
                let v = &args[i + 1];
                if vars.contains_key(k) {
                    vars.insert(k.to_string(), v.to_string());
                }
                i += 1;
            }
        }
        i += 1;
    }
}

fn copy_file_while_replacing(
    from: &Path,
    to: &Path,
    vars: &HashMap<String, String>,
) -> Result<(), Error> {
    let env = Environment::new();
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap();
        if file_name == "config.toml" {
            continue;
        }
        let target = to.join(file_name);
        if file_path.is_dir() {
            let _ = fs::create_dir_all(&target);
            copy_file_while_replacing(&file_path, &target, vars)?;
        } else {
            let content = fs::read_to_string(&file_path)?;
            let tmpl = env.template_from_str(&content).unwrap();
            let rendered = tmpl.render(vars).unwrap();
            fs::write(&target, rendered)?;
        }
    }
    Ok(())
}
