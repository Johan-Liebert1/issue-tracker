use std::io::Write;

use crate::{
    constants::{RESET, YELLOW},
    github,
    types::{config::Config, misc::VectorHashMap},
};

#[inline]
pub fn prompt_user(prompt: &str) -> String {
    let mut user_input = String::new();

    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

// TODO: make this return either a bool or a string so we can check if input == "exit" to exit the program
pub fn prompt_yes_or_no(prompt: &str) -> (bool, String) {
    let user_input = prompt_user(prompt);

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
                let (create, _) = prompt_yes_or_no(create_issue_prompt);

                if create {
                    github::create_issue(issue, &config).unwrap();
                } else {
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

pub fn get_starting_whitespace_len(s: &str) -> usize {
    let mut index = 0;

    for char in s.chars() {
        if !char.is_whitespace() {
            break;
        }

        index += 1;
    }

    return index as usize;
}

pub fn min<T>(a: T, b: T) -> T
where
    T: PartialEq + PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}
