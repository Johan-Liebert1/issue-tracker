use constants::{BLUE, RED};
use files::walk_dirs;
use github::placeholder_api_call;
use helpers::{color_print, print_all_issues, prompt_yes_or_no};
use std::{
    collections::{HashMap, HashSet},
    env,
};
use types::{Config, VectorHashMap};

mod constants;
mod files;
mod github;
mod helpers;
mod types;

fn main() {
    if false {
        placeholder_api_call();
        return;
    }

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
        ]),
    };

    config.set_from_file();
    config.set_git_credentials();

    println!("Config = {:#?}", config);

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
