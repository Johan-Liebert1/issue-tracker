use regex::Regex;
use std::{collections::HashSet, env, fs};

const FILE_EXT: &str = ".rs";

enum IssueType {
    Todo,
    Fixme,
    Other,
}

impl IssueType {
    fn to_str(&self) -> String {
        match &self {
            IssueType::Todo => String::from("Todo"),
            IssueType::Fixme => String::from("Fixme"),
            IssueType::Other => String::from("Other"),
        }
    }

    fn from_str(&self, string: &String) -> Self {
        match string.as_str() {
            "Todo" | "TODO" => IssueType::Todo,
            "Fixme" | "FIXME" => IssueType::Fixme,
            _ => IssueType::Other,
        }
    }
}

struct Issue {
    issue_type: IssueType,
    priority: i16,
    description: String,
}

// TODO: first todo
// TODO: second todo

fn find_todos(file_contents: &String) {
    let re = Regex::new("^(//|#|--) (TOD(O*)|FIXM(E*)):(.*)").unwrap();

    // Capture 0 - Entire match
    // Capture 1 - Comment symbol
    // Capture 2 - TODO | FIXME
    // Capture 3 - O | None
    // Capture 4 - E | None
    // Capture 5 - Description

    for line in file_contents.split("\n") {
        if let Some(captures) = re.captures(&line) {
            println!("\n\n{} \nCaptures = {:?}", line, captures);

            let todos = &captures[0];
            let todo_len = &captures[3];
            let todo_desc = &captures[3];

            println!(
                "todos = {}, todo_len = {}, todo_desc = {}",
                todos, todo_len, todo_desc
            );
        }
    }
}

fn walk_dirs(path: &String, folders_to_ignore: &HashSet<&str>) {
    let files = fs::read_dir(path).unwrap();

    for file in files {
        let current_path = file.unwrap().path();

        let current_path_str = current_path.to_str().unwrap();

        // TODO: Refactor this thing
        if current_path.is_file()
            && (current_path_str.ends_with(FILE_EXT) || current_path_str.ends_with(".py"))
        {
            match fs::read_to_string(&current_path) {
                Ok(file_content) => {
                    find_todos(&file_content);
                }

                Err(error) => {
                    println!("Failed to read file {}. Error: {}", current_path_str, error);
                }
            }
        } else if current_path.is_dir() {
            // ignore symlinks
            let splits: Vec<&str> = current_path_str.split("/").collect();
            let dir_name = *splits.last().unwrap();

            if folders_to_ignore.contains(dir_name) {
                println!("{} in ignore list. Igonoring", dir_name);
                continue;
            }

            walk_dirs(&String::from(current_path_str), folders_to_ignore);
        }
    }
}

fn main() {
    let folders_to_ignore: HashSet<&str> =
        HashSet::from([".git", "node_modules", "target", "dist", "env"]);

    let args: Vec<String> = env::args().collect();

    let mut cwd = &String::from(env::current_dir().unwrap().to_str().unwrap());

    if args.len() > 1 {
        cwd = &args[1];
    }

    walk_dirs(cwd, &folders_to_ignore);
}
