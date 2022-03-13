use crate::{
    constants::{RESET, YELLOW},
    types::VectorHashMap,
};

// TODO: Use Generics here
pub fn color_print(color: &'static str, string: &String) {
    let string = format!("{}{}{}", color, string, RESET);

    println!("{}", string);
}

pub fn print_all_issues(all_files_issues: &VectorHashMap) {
    println!("");

    for (file, all_issues) in all_files_issues {
        color_print(YELLOW, file);

        for issue in all_issues {
            println!("{}{}", issue.to_str(), RESET);
        }

        println!("");
    }
    println!("{}", RESET);
}
