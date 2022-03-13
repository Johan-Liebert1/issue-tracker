use helpers::print_all_issues;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

mod constants;
mod helpers;
mod types;

use types::{Issue, IssueType, VectorHashMap};

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

fn is_file_ext_valid(path: &str) -> bool {
    let allowed_extensions = [
        ".py", ".rs", ".c", ".cpp", ".js", ".ts", ".tsx", ".sql", ".go",
    ];

    for ext in allowed_extensions {
        if path.ends_with(ext) {
            return true;
        }
    }

    false
}

fn walk_dirs(path: &String, folders_to_ignore: &HashSet<&str>, all_issues: &mut VectorHashMap) {
    let files = fs::read_dir(path).unwrap();

    for file in files {
        let current_path = file.unwrap().path();

        let current_path_str = current_path.to_str().unwrap();

        if current_path.is_file() && is_file_ext_valid(current_path_str) {
            match fs::read_to_string(&current_path) {
                Ok(file_content) => {
                    let issues_in_file = find_todos(&file_content);

                    if issues_in_file.len() > 0 {
                        all_issues.insert(current_path_str.to_string(), issues_in_file);
                    }
                }

                Err(error) => {
                    println!("Failed to read file {}. Error: {}", current_path_str, error);
                    continue;
                }
            }
        } else if current_path.is_dir() {
            let splits: Vec<&str> = current_path_str.split("/").collect();
            let dir_name = *splits.last().unwrap();

            // ignore hiddent files
            if folders_to_ignore.contains(dir_name) || dir_name.starts_with(".") {
                println!("{} in ignore list. Igonoring", dir_name);
                continue;
            }

            walk_dirs(
                &String::from(current_path_str),
                folders_to_ignore,
                all_issues,
            );
        }
    }
}

fn main() {
    let folders_to_ignore: HashSet<&str> = HashSet::from(["node_modules", "target", "dist", "env"]);

    let args: Vec<String> = env::args().collect();

    let mut cwd = &String::from(env::current_dir().unwrap().to_str().unwrap());

    if args.len() > 1 {
        cwd = &args[1];
    }

    let mut hash: VectorHashMap = HashMap::new();

    walk_dirs(cwd, &folders_to_ignore, &mut hash);

    print_all_issues(&hash);
}
