use std::io::{self, Write};

use crate::{constants::RESET, types::Issue};

#[allow(dead_code)]
fn color_print(color: &'static str, string: &String) {
    let string = format!("{}{}{}", color, string, RESET);

    println!("{}", string);
}

pub fn print_all_issues(all_issues: &Vec<Issue>) {
    for issue in all_issues {
        println!("{}", issue.to_str());
    }
    print!("{}", RESET);
    io::stdout().flush().unwrap();
}
