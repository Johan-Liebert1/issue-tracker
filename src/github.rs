use std::{
    collections::HashMap,
    process::{self, Output},
};

use crate::{types::{config::Config, issue::Issue}, helpers::color_print, constants::GREEN};

pub fn get_username() -> Result<Output, std::io::Error> {
    process::Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .output()
}

pub fn get_repo_url(cwd: &String) -> Result<Output, std::io::Error> {
    process::Command::new("git")
        .current_dir(cwd)
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
}

// TODOOOOOO: Return proper result from this function
pub fn create_issue(issue: &Issue, config: &Config) -> Result<(), ()> {
    let issue_comment = issue.get_issue_comment(config);

    let mut json_body = HashMap::new();

    json_body.insert("title", &issue.description);
    json_body.insert("body", &issue_comment);

    let client = reqwest::blocking::Client::new();

    // https://api.github.com/repos/OWNER/REPO/issues 
    let request_url = format!(
        "https://api.github.com/repos/{}/{}/issues",
        config.git_username, config.repo_name
    );

    let token = format!("Bearer {}", &config.git_access_token);

    println!("request_url = {}, token = {}", &request_url, &token);

    let request_builder = client
        .post(&request_url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("Authorization", &token)
        .header(
            "User-Agent", 
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36"
        )
        .json(&json_body);

    let res = request_builder.send();

    match res {
        Ok(resp) => {
            resp.json::<serde_json::Value>().unwrap();
            color_print(GREEN, &String::from("Successfully created issue"), true);
        },

        Err(error) => {
            println!("Request errored out with error {}", error);
        }
    }

    Ok(())
}

pub fn placeholder_api_call() {
    let client = reqwest::blocking::Client::new();

    let res = client
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .send();

    let resp_json = res.unwrap().json::<serde_json::Value>().unwrap();

    println!("res = {:#?}\n {}", resp_json, resp_json["title"]);
}
