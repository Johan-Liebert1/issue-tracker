use constants::BLUE;
use helpers::{color_print, print_all_issues, prompt_yes_or_no};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};
use types::{Config, Issue, IssueType, VectorHashMap};

use crate::constants::RED;

mod constants;
mod github;
mod helpers;
mod types;

fn find_todos(file_contents: &String) -> Vec<Issue> {
    let re = Regex::new(r#"([/#"-]*)[\s]*(TOD(O*)|FIXM(E*)):(.*)"#).unwrap();

    // Capture 0 - Entire match
    // Capture 1 - Comment symbol
    // Capture 2 - TODO | FIXME
    // Capture 3 - O | None
    // Capture 4 - E | None
    // Capture 5 - Description

    let mut vector: Vec<Issue> = Vec::new();

    for (line_number, line) in file_contents.split("\n").enumerate() {
        if let Some(captures) = re.captures(&line) {
            if let Some(description) = captures.get(5) {
                // only add if description exists

                let issue_type = IssueType::from_str(&captures[2]);

                let priority = match issue_type {
                    IssueType::Todo => {
                        let string = &captures[3];
                        string.len()
                    }

                    IssueType::Fixme => {
                        let string = &captures[4];
                        string.len()
                    }
                };

                vector.push(Issue {
                    issue_type,
                    priority,
                    description: description.as_str().to_string(),
                    line_number: line_number + 1,
                });
            };
        }
    }

    vector
}

fn is_file_ext_valid(path: &str, config: &Config) -> bool {
    for ext in config.allowed_extensions.iter() {
        if path.ends_with(ext) {
            return true;
        }
    }

    false
}

fn walk_dirs(
    path: &str,
    config: &Config,
    all_issues: &mut VectorHashMap,
    num_files_scanned: &mut i32,
) {
    let files = fs::read_dir(path).unwrap();

    for file in files {
        let current_path = file.unwrap().path();

        let current_path_str = current_path.to_str().unwrap();

        if current_path.is_file() && is_file_ext_valid(current_path_str, config) {
            match fs::read_to_string(&current_path) {
                Ok(file_content) => {
                    let issues_in_file = find_todos(&file_content);

                    if issues_in_file.len() > 0 {
                        all_issues.insert(current_path_str.to_string(), issues_in_file);
                    }

                    *num_files_scanned += 1;
                }

                Err(error) => {
                    println!("Failed to read file {}. Error: {}", current_path_str, error);
                    continue;
                }
            }
        } else if current_path.is_dir() {
            let splits: Vec<&str> = current_path_str.split("/").collect();
            let dir_name = *splits.last().unwrap();

            // ignore hidden files
            if config.folders_to_ignore.contains(dir_name) || dir_name.starts_with(".") {
                println!("Ignoring {}", &current_path_str[config.cwd.len() + 1..]);
                continue;
            }

            walk_dirs(
                &String::from(current_path_str),
                config,
                all_issues,
                num_files_scanned,
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir().unwrap();

    let mut cwd = current_dir.to_str().unwrap();

    if args.len() > 1 {
        cwd = &args[1];
    }

    let mut config: Config = Config {
        folders_to_ignore: HashSet::from(["node_modules", "target", "dist", "env", "tests"]),
        allowed_extensions: vec![
            ".py", ".rs", ".c", ".cpp", ".js", ".ts", ".tsx", ".sql", ".go",
        ],
        cwd: &String::from(cwd),
        config_file_name: &String::from("it.conf"),
        git_username: &mut String::from(""),
        repo_url: &mut String::from(""),
        repo_name: &mut String::from(""),
        git_access_token: &mut String::from(""),
        all_git_creds_available: true,
        git_creds_unavailable: Vec::new(),
    };

    config.set_from_file();
    config.set_git_credentials();

    // println!("Config = {:#?}", config);

    std::env::set_current_dir(&config.cwd).unwrap();

    let mut num_files_scanned = 0;
    let mut hash: VectorHashMap = HashMap::new();

    walk_dirs(&cwd, &config, &mut hash, &mut num_files_scanned);

    print_all_issues(&mut hash, &config, false);

    color_print(
        BLUE,
        &format!("Successfully scanned {} files", num_files_scanned),
        true,
    );

    if prompt_yes_or_no("\nCreate issues? (y/n) > ").0 {
        if !config.all_git_creds_available {
            color_print(
                RED,
                &String::from("The following credentials not found. Cannot proceed"),
                true,
            );

            for cred in &config.git_creds_unavailable {
                println!("{} ", cred);
            }

            return;
        }

        print_all_issues(&mut hash, &config, true);
    }
}
