use crate::{
    constants::{RESET, YELLOW},
    types::VectorHashMap,
};

// TODO: Use Generics here
pub fn color_print(color: &'static str, string: &String) {
    let string = format!("{}{}{}", color, string, RESET);

    println!("{}", string);
}

pub fn print_all_issues(all_files_issues: &mut VectorHashMap) {
    println!("");

    for (file, all_issues) in all_files_issues {
        color_print(YELLOW, file);

        all_issues.sort_by(|item1, item2| item2.priority.cmp(&item1.priority));

        for issue in all_issues {
            println!("{}{}", issue.to_str(), RESET);
        }

        println!("");
    }
    println!("{}", RESET);
}
