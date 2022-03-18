use std::process::{self, Output};

/*
curl
-X POST
-H "Accept: application/vnd.github.v3+json"
https://api.github.com/repos/Johan-Liebert1/issue_tracker/issues
-d '{"title":"Issue create to test api"}'
-u Johan-Liebert1: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
*/

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
