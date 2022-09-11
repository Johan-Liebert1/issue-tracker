use crate::{
    constants::NUM_FILE_LINES,
    types::{
        config::Config,
        issue::{FileLines, Issue, IssueType},
        misc::VectorHashMap,
    },
};

use regex::Regex;
use std::fs;

pub fn find_todos(file_contents: &String, file_name: &str) -> Vec<Issue> {
    let re = Regex::new(r#"([/#"{*-]*)[\s]*(TOD(O*)|FIXM(E*)):(.*)"#).unwrap();
    let file_name_str = String::from(file_name);

    // Capture 0 - Entire match
    // Capture 1 - Comment symbol
    // Capture 2 - TODO | FIXME
    // Capture 3 - O | None
    // Capture 4 - E | None
    // Capture 5 - Description

    let mut vector: Vec<Issue> = Vec::new();

    let enumerated_file_contents: Vec<&str> = file_contents.split("\n").collect();

    for (line_number, line) in enumerated_file_contents.iter().enumerate() {
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

                let mut file_lines: Vec<FileLines> = Vec::new();

                for i in 0..=NUM_FILE_LINES {
                    let index = line_number + i;

                    if index >= enumerated_file_contents.len() {
                        break;
                    }

                    file_lines.push(FileLines {
                        line_number: index,
                        line_text: enumerated_file_contents[index].to_string(),
                    })
                }

                vector.push(Issue {
                    issue_type,
                    priority,
                    description: description.as_str().to_string(),
                    line_number: line_number + 1,
                    file_name: file_name_str.clone(), // this cloning is fine
                    more_info: file_lines,
                });
            };
        }
    }

    vector
}

pub fn is_file_ext_valid(path: &str, config: &Config) -> bool {
    for ext in &config.allowed_extensions {
        if path.ends_with(ext) {
            return true;
        }
    }

    false
}

pub fn walk_dirs(
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
                    let issues_in_file = find_todos(&file_content, current_path_str);

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
