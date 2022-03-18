use std::io::Write;

use crate::{
    constants::{RESET, YELLOW},
    github,
    types::{Config, CreateIssueParams, VectorHashMap},
};

// TODO: make this return either a bool or a string so we can check if input == "exit" to exit the program
pub fn prompt_yes_or_no(prompt: &str) -> bool {
    let mut create_issue = String::new();

    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut create_issue).unwrap();

    if create_issue.trim().to_lowercase() == "y" {
        true
    } else {
        false
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
    let create_issue_prompt = "Create an issue? (y/n)";

    let create_issue_params = CreateIssueParams {
        repo_url: &String::from_utf8(config.repo_url.as_ref().unwrap().stdout.to_owned())
            .unwrap()
            .trim()
            .to_string(),
        github_username: &String::from_utf8(
            config.git_username.as_ref().unwrap().stdout.to_owned(),
        )
        .unwrap()
        .trim()
        .to_string(),
        access_token: &String::from(
            std::env::var("GITHUB_ACCESS_TOKEN").expect("Cannot find variable GITHUB_ACCESS_TOKEN"),
        ),
    };

    println!("{:?}", &create_issue_params);

    for (file, all_issues) in all_files_issues {
        color_print(YELLOW, file, true);

        all_issues.sort_by(|item1, item2| item2.priority.cmp(&item1.priority));

        for issue in all_issues {
            println!("{}{}", issue.to_str(), RESET);

            if create_issue {
                // color_print(LIGHT_GREEN, create_issue_prompt, false);
                let create = prompt_yes_or_no(create_issue_prompt);

                if create {
                    println!(
                        "{:?}",
                        github::create_issue(issue, &create_issue_params).unwrap()
                    );
                }
            }
        }

        println!("");
    }
    println!("{}", RESET);
}
