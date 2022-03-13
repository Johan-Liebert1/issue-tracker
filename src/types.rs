use std::collections::HashMap;

use crate::constants::{CYAN, GREEN, MAGENTA};

#[derive(Debug)]
pub enum IssueType {
    Todo,
    Fixme,
}

impl IssueType {
    pub fn to_colored_str(&self) -> String {
        match &self {
            IssueType::Todo => format!("{}Todo  ", CYAN),
            IssueType::Fixme => format!("{}Fixme ", MAGENTA),
        }
    }

    pub fn from_str(string: &str) -> Self {
        if string.to_lowercase().starts_with("fixme") {
            IssueType::Fixme
        } else {
            IssueType::Todo
        }
    }
}

#[derive(Debug)]
pub struct Issue {
    pub issue_type: IssueType,
    pub priority: usize,
    pub description: String,
    pub line_number: usize,
}

impl Issue {
    pub fn to_str(&self) -> String {
        format!(
            "{}{:>7}{:>10} {}{}",
            self.issue_type.to_colored_str(),
            format!("({})", self.priority),
            format!("Line: {}", self.line_number),
            GREEN,
            if self.description.len() < 300 {
                &self.description
            } else {
                &self.description[..300]
            }
        )
    }
}

pub type VectorHashMap = HashMap<String, Vec<Issue>>;
