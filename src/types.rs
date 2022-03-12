#[derive(Debug)]
pub enum IssueType {
    Todo,
    Fixme,
}

impl IssueType {
    pub fn to_str(&self) -> String {
        match &self {
            IssueType::Todo => String::from("Todo"),
            IssueType::Fixme => String::from("Fixme"),
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
        todo!();
    }
}
