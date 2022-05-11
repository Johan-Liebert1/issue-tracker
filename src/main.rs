use constants::{BLUE, RED};
use files::walk_dirs;
use github::placeholder_api_call;
use helpers::{color_print, print_all_issues, prompt_yes_or_no};
use std::{collections::HashMap, env};
use types::misc::VectorHashMap;

use crate::types::config::Config;

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

    let mut config: Config = Config::new(cwd);

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
