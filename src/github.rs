use std::{
    collections::HashMap,
    process::{self, Output},
};

use crate::types::{Config, Issue};

pub fn get_username() -> Result<Output, std::io::Error> {
    process::Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .output()
}

pub fn get_repo_url() -> Result<Output, std::io::Error> {
    process::Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
}

// TODOOOOOO: Return proper result from this function
pub fn create_issue(issue: &Issue, config: &Config) -> Result<(), ()> {
    let mut json_body = HashMap::new();

    json_body.insert("title", &issue.description);

    let client = reqwest::blocking::Client::new();
    let request_builder = client
        .post(&format!(
            "https://api.github.com/repos/{}/{}/issues",
            config.git_username, config.repo_name
        ))
        .header("Accept", "application/vnd.github.v3+json")
        .basic_auth(&config.git_username, Some(&config.git_access_token))
        .header(
            "User-Agent", 
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36"
        )
        .json(&json_body);

    println!("builder = {:?}\n", request_builder);

    let _res = request_builder.send();

    Ok(())
}
