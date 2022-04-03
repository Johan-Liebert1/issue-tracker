use std::io::Write;

use crate::{
    constants::{RESET, YELLOW},
    github,
    types::{config::Config, misc::VectorHashMap},
};

// TODO: make this return either a bool or a string so we can check if input == "exit" to exit the program
pub fn prompt_yes_or_no(prompt: &str) -> (bool, String) {
    let mut user_input = String::new();

    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_input).unwrap();

    if user_input.trim().to_lowercase() == "y" {
        (true, user_input)
    } else {
        (false, user_input)
    }
}

// TODO: Use Generics here
pub fn color_print(color: &'static str, string: &String, new_line: bool) {
    let string = format!("{}{}{}", color, string, RESET);

    if new_line {
        println!("{}", string);
    } else {
        print!("{}", string);
        std::io::stdout().flush().unwrap();
    }
}

pub fn print_all_issues(all_files_issues: &mut VectorHashMap, config: &Config, create_issue: bool) {
    let create_issue_prompt = "Create an issue? (y/n/exit) ";

    'outer: for (file, all_issues) in all_files_issues {
        color_print(YELLOW, file, true);

        all_issues.sort_by(|item1, item2| item2.priority.cmp(&item1.priority));

        for issue in all_issues {
            println!("{}{}", issue.to_str(&config), RESET);

            if create_issue {
                // color_print(LIGHT_GREEN, create_issue_prompt, false);
                let (create, input) = prompt_yes_or_no(create_issue_prompt);
                if create {
                    println!("{:?}", github::create_issue(issue, &config).unwrap());
                } else if input.trim().to_lowercase() == "exit" {
                    break 'outer;
                }
            }
        }

        println!("");
    }
    println!("{}", RESET);
}

pub fn string_from_vecu8(string: &mut String, vector: &Vec<u8>) {
    let str1 = String::from(std::str::from_utf8(vector).unwrap());
    string.push_str(&str1.trim());
}
