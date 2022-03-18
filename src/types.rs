use std::{
    collections::{HashMap, HashSet},
    path::Path,
    process::exit,
};

use crate::{
    constants::{BLUE, CYAN, GREEN, MAGENTA, RED},
    helpers::color_print,
};

#[derive(Debug)]
pub enum IssueType {
    Todo,
    Fixme,
}

impl IssueType {
    pub fn to_colored_str(&self) -> String {
        match &self {
            IssueType::Todo => format!("{}Todo  ", CYAN),
            IssueType::Fixme => format!("{}Fixme ", MAGENTA),
        }
    }

    pub fn from_str(string: &str) -> Self {
        if string.to_lowercase().starts_with("fixme") {
            IssueType::Fixme
        } else {
            IssueType::Todo
        }
    }
}

#[derive(Debug)]
pub struct Issue {
    pub issue_type: IssueType,
    pub priority: usize,
    pub description: String,
    pub line_number: usize,
}

impl Issue {
    pub fn to_str(&self) -> String {
        format!(
            "{}{:>7}{:>10} {}{}",
            self.issue_type.to_colored_str(),
            format!("({})", self.priority),
            format!("Line: {}", self.line_number),
            GREEN,
            if self.description.len() < 300 {
                &self.description
            } else {
                &self.description[..300]
            }
        )
    }
}

pub type VectorHashMap = HashMap<String, Vec<Issue>>;

#[derive(Debug)]
pub struct Config {
    pub folders_to_ignore: HashSet<&'static str>,
    pub allowed_extensions: Vec<&'static str>,
    pub cwd: String,
    pub config_file_name: String,
    pub git_username: Result<std::process::Output, std::io::Error>,
    pub repo_url: Result<std::process::Output, std::io::Error>,
}

impl Config {
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
}

#[derive(Debug)]
pub struct CreateIssueParams<'a> {
    pub repo_url: &'a String,
    pub github_username: &'a String,
    pub access_token: &'a String,
}
