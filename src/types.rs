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
            IssueType::Todo => format!("{}Todo ", CYAN),
            IssueType::Fixme => format!("{}Fixme ", MAGENTA),
        }
    }

    pub fn from_str(string: &str) -> Self {
        match string {
            "Todo" | "TODO" => IssueType::Todo,
            "Fixme" | "FIXME" => IssueType::Fixme,
            _ => IssueType::Todo,
        }
    }
}

#[derive(Debug)]
pub struct Issue {
    pub issue_type: IssueType,
    pub priority: usize,
    pub description: String,
}

impl Issue {
    pub fn to_str(&self) -> String {
        format!(
            "{}({}) {}{}",
            self.issue_type.to_colored_str(),
            self.priority,
            GREEN,
            self.description,
        )
    }
}

pub type VectorHashMap = HashMap<String, Vec<Issue>>;
