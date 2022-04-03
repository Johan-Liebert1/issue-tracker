use std::{
    collections::{HashMap, HashSet},
    path::Path,
    process::exit,
};

use crate::{
    constants::{BLUE, CYAN, GREEN, MAGENTA, RED, RESET},
    github,
    helpers::{self, color_print},
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
pub struct FileLines {
    pub line_number: usize,
    pub line_text: String,
}

#[derive(Debug)]
pub struct Issue {
    pub issue_type: IssueType,
    pub priority: usize,
    pub description: String,
    pub line_number: usize,
    pub file_name: String,
    pub more_info: Vec<FileLines>,
}

impl Issue {
    pub fn to_str(&self, config: &Config) -> String {
        format!(
            "{}{:>7}{:>10} {}{} \n{}{}\n",
            self.issue_type.to_colored_str(),
            format!("({})", self.priority),
            format!("Line: {}", self.line_number),
            GREEN,
            if self.description.len() < 300 {
                &self.description
            } else {
                &self.description[..300]
            },
            RESET,
            self.get_issue_comment(config)
        )
    }

    fn get_file_ext(&self) -> String {
        let final_dot_pos = &self.file_name.rfind(".").unwrap();

        String::from(&self.file_name[*final_dot_pos + 1..])
    }

    pub fn get_issue_comment(&self, config: &Config) -> String {
        let backticks = String::from("```");

        let mut comment = String::new();

        let file_ext = self.get_file_ext();
        let file_type = config.file_ext_to_markdown.get(file_ext.as_str()).unwrap();

        comment.push_str("## ");
        comment.push_str(&self.file_name);
        comment.push_str("\n\n");

        comment.push_str(&backticks);
        comment.push_str(*file_type);

        for line_info in &self.more_info {
            comment.push('\n');
            comment.push_str(&line_info.line_number.to_string());
            comment.push(' ');
            comment.push_str(&line_info.line_text);
        }

        comment.push('\n');
        comment.push_str(&backticks);

        comment
    }
}

pub type VectorHashMap = HashMap<String, Vec<Issue>>;

#[derive(Debug)]
pub struct Config<'a> {
    pub folders_to_ignore: HashSet<&'static str>,
    pub allowed_extensions: Vec<&'static str>,
    pub cwd: &'a String,
    pub config_file_name: &'a String,
    pub git_username: &'a mut String,
    pub repo_url: &'a mut String,
    pub repo_name: &'a mut String,
    pub git_access_token: &'a mut String,
    pub all_git_creds_available: bool,
    pub git_creds_unavailable: Vec<&'static str>,
    pub file_ext_to_markdown: HashMap<&'static str, &'static str>,
}

impl Config<'_> {
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

        match github::get_repo_url() {
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

#[derive(Debug)]
pub struct CreateIssueRequestBody<'a> {
    pub title: &'a String,
    pub description: &'a String,
}
