use std::{collections::HashSet, env, fs};

const FILE_EXT: &str = ".rs";

fn walk_dirs(path: &String, folders_to_ignore: &HashSet<&str>) {
    let files = fs::read_dir(path).unwrap();

    for file in files {
        let current_path = file.unwrap().path();

        let current_path_str = current_path.to_str().unwrap();

        if current_path.is_file() && current_path_str.ends_with(FILE_EXT) {
            println!("{}", fs::read_to_string(current_path).unwrap());
        } else if current_path.is_dir() {
            // ignore symlinks
            let splits: Vec<&str> = current_path_str.split("/").collect();
            let dir_name = *splits.last().unwrap();

            if folders_to_ignore.contains(dir_name) {
                println!("{} in ignore list. Igonoring", dir_name);
                continue;
            }

            walk_dirs(&String::from(current_path_str), folders_to_ignore);
        }
    }
}

fn main() {
    let folders_to_ignore: HashSet<&str> =
        HashSet::from([".git", "node_modules", "target", "dist", "env"]);

    let args: Vec<String> = env::args().collect();

    let mut cwd = &String::from(env::current_dir().unwrap().to_str().unwrap());

    if args.len() > 1 {
        cwd = &args[1];
    }

    walk_dirs(cwd, &folders_to_ignore);
}
