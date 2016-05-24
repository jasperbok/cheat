use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

// TODO: Exit if no subject was given.
// TODO: Check if a file exists before trying to print it.
// TODO: If a file can't be read, exist with error instead of panicking.
// TODO: Support all extensions (or no extension).

fn main() {
    let subject = get_subject_from_args();
    let file_path = build_file_path(&subject);

    println!("{}", file_path);

    print_sheet(&file_path);
}

// Build a filepath to a file.
fn build_file_path(subject: &String) -> String {
    let mut dir = String::new();

    match env::var("CHEATSHEET_DIR") {
        Ok(val) => dir.push_str(&val),
        Err(e) => println!("{}", e),
    };

    // If the CHEATSHEET_DIR variable is not set, try to use the user's home
    // directory instead.
    if dir.len() == 0 {
        match env::home_dir() {
            Some(path) => dir.push_str(path.to_str().unwrap()),
            None => panic!("Can't find any usable directory"),
        }
    }

    // Always make sure the path ends with a slash.
    if !dir.ends_with("/") {
        dir.push_str("/");
    }

    dir.push_str(&subject);
    dir.push_str(".txt");

    dir
}

/// Returns the subject the user wishes to see a reference for.
/// This subject is always provided as the first argument to the
/// application.
fn get_subject_from_args() -> String {
    let mut args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        args.remove(1)
    } else {
        "none".to_string()
    }
}

/// Print the cheatsheet in path.
fn print_sheet(path: &str) {
    let mut contents = String::new();
    let mut f: fs::File;

    match fs::File::open(path) {
        Ok(val) => f = val,
        Err(e) => panic!(e),
    };

    match f.read_to_string(&mut contents) {
        Ok(_) => print!("{}", contents),
        Err(e) => println!("{}", e),
    };
}
