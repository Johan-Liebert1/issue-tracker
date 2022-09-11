use std::{
    collections::{HashMap, HashSet},
    path::Path,
    process::exit,
};

use crate::{
    constants::{BLUE, RED},
    github,
    helpers::{self, color_print},
};

#[derive(Debug)]
pub struct Config {
    pub folders_to_ignore: HashSet<&'static str>,
    pub allowed_extensions: Vec<&'static str>,
    pub cwd: String,
    pub config_file_name: String,
    pub git_username: String,
    pub repo_url: String,
    pub repo_name: String,
    pub git_access_token: String,
    pub all_git_creds_available: bool,
    pub git_creds_unavailable: Vec<&'static str>,
    pub file_ext_to_markdown: HashMap<&'static str, &'static str>,
}

impl Config {
    pub fn new(cwd: &str) -> Self {
        let config = Config {
            folders_to_ignore: HashSet::from(["node_modules", "target", "dist", "env"]),
            allowed_extensions: vec![
                ".py", ".rs", ".c", ".cpp", ".js", ".ts", ".tsx", ".sql", ".go",
            ],
            cwd: String::from(cwd),
            config_file_name: String::from("it.conf"),
            git_username: String::from(""),
            repo_url: String::from(""),
            repo_name: String::from(""),
            git_access_token: String::from(""),
            all_git_creds_available: true,
            git_creds_unavailable: Vec::new(),
            file_ext_to_markdown: HashMap::from([
                ("py", "python"),
                ("rs", "rust"),
                ("c", "c"),
                ("cpp", "cpp"),
                ("js", "js"),
                ("ts", "ts"),
                ("go", "go"),
                ("java", "java"),
                ("html", "html"),
                ("css", "css"),
                ("tsx", "tsx"),
                ("jsx", "jsx"),
                ("sql", "sql"),
            ]),
        };

        config
    }

    /// 1. Checks if the provided cwd exists or not
    /// 2. Checks if a config file exists, and if it does, gets the config from that file
    pub fn set_from_file(&mut self) {
        let cwd_path = Path::new(&self.cwd);

        if !cwd_path.exists() {
            color_print(
                RED,
                &format!("Path '{}' does not exist", cwd_path.to_str().unwrap()),
                true,
            );
            exit(1);
        }

        let config_file_exists = cwd_path.join(&self.config_file_name).exists();

        if !config_file_exists {
            color_print(
                BLUE,
                &String::from("Config file not found. Using default config\n"),
                true,
            );
            return;
        }
    }

    pub fn set_git_credentials(&mut self) {
        match github::get_username() {
            Ok(u) => {
                helpers::string_from_vecu8(&mut self.git_username, &u.stdout);
            }
            Err(_error) => {
                self.all_git_creds_available = false;
                self.git_creds_unavailable.push("username");
            }
        };

        match github::get_repo_url(&self.cwd) {
            Ok(u) => {
                helpers::string_from_vecu8(&mut self.repo_url, &u.stdout);

                self.repo_name
                    .push_str(&self.repo_url.split("/").last().unwrap().replace(".git", ""));
            }
            Err(_error) => {
                self.all_git_creds_available = false;
                self.git_creds_unavailable.push("repo_url");
            }
        };

        match std::env::var("GITHUB_ACCESS_TOKEN") {
            Ok(access_tok) => self.git_access_token.push_str(&access_tok),
            Err(_err) => {
                self.all_git_creds_available = false;
                self.git_creds_unavailable.push("access token");
            }
        }
    }
}
