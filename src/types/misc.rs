use std::collections::HashMap;

use super::issue::Issue;

pub type VectorHashMap = HashMap<String, Vec<Issue>>;

#[derive(Debug)]
pub struct CreateIssueRequestBody<'a> {
    pub title: &'a String,
    pub description: &'a String,
}
