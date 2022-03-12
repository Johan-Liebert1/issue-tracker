use crate::types::Issue;

pub const RED: &str = "\u{001b}[31m";
pub const BLACK: &str = "\u{001b}[30m";
pub const GREEN: &str = "\u{001b}[32m";
pub const RESET: &str = "\u{001b}[0m";

fn color_print(color: &'static str, string: &String) {
    let string = format!("{}{}{}", color, string, RESET);

    println!("{}", string);
}

pub fn print_all_issues(all_issues: &Vec<Issue>) {
    for issue in all_issues {
        color_print(GREEN, &issue.description);
    }
}
