use crate::constants::{CYAN, GREEN, MAGENTA, RESET};

use super::config::Config;

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
pub struct FileLines {
    pub line_number: usize,
    pub line_text: String,
    pub starting_whitespace_len: usize,
}

#[derive(Debug)]
pub struct Issue {
    pub issue_type: IssueType,
    pub priority: usize,
    pub description: String,
    pub line_number: usize,
    pub file_name: String,
    pub more_info: Vec<FileLines>,
}

impl Issue {
    pub fn to_str(&self, config: &Config) -> String {
        format!(
            "{}{:>7}{:>10} {}{} \n{}{}\n",
            self.issue_type.to_colored_str(),
            format!("({})", self.priority),
            format!("Line: {}", self.line_number),
            GREEN,
            if self.description.len() < 300 {
                &self.description
            } else {
                &self.description[..300]
            },
            RESET,
            self.get_issue_comment(config)
        )
    }

    fn get_file_ext(&self) -> String {
        let final_dot_pos = &self.file_name.rfind(".").unwrap();

        String::from(&self.file_name[*final_dot_pos + 1..])
    }

    pub fn get_issue_comment(&self, config: &Config) -> String {
        let backticks = String::from("```");

        let mut comment = String::new();

        let file_ext = self.get_file_ext();
        let file_type = config.file_ext_to_markdown.get(file_ext.as_str()).unwrap();

        comment.push_str("## ");
        comment.push_str(&self.file_name);
        comment.push_str("\n\n");

        comment.push_str(&backticks);
        comment.push_str(*file_type);

        for line_info in &self.more_info {
            comment.push('\n');
            comment.push_str(&line_info.line_number.to_string());
            comment.push(' ');
            comment.push_str(&line_info.line_text);
        }

        comment.push('\n');
        comment.push_str(&backticks);

        comment
    }
}
