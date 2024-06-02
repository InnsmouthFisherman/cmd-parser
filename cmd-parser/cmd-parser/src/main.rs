use std::fs;
use std::io;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("{}, version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let mut msg = String::new();
    let mut path = PathBuf::new();

    match env::current_dir() {
        Ok(current_dir) => {
            println!("Current directory: {:?}", current_dir);
            path = current_dir;
        },
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
        }
    }

    while msg.trim() != "exit" {
        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read line");
        msg = msg.trim().to_string();
        match msg.as_str() {
            "-h" | "--help" => println!("Help message"),
            "ls" | "list_directories" => locate_directories(&path),
            "rd" | "read" => read_file(),
            "cd" | "child_dir" => {
                path = child_dir(&mut path);
            },
            _ => println!("Unknown argument: {}", msg),
        }
        msg.clear();
    }
}

fn child_dir(path: &mut PathBuf) -> PathBuf {
    let mut req = String::new();
    io::stdin()
        .read_line(&mut req)
        .expect("Failed to read line");

    path.push(&req.trim());
    println!("{}", path.display());
    path.clone()
}

fn locate_directories(path: &PathBuf) {
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        println!("{}", entry.path().display());
    }
}

fn read_file() {
    println!("Enter file name: ");
    let mut req = String::new();
    io::stdin()
        .read_line(&mut req)
        .expect("Failed to read line");
    let filename = req.trim(); // Trim to remove any trailing newline characters

    // Check if the filename is valid
    if filename.contains(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..]) {
        println!("Filename contains invalid characters. Please enter a valid filename.");
        return;
    }

    match fs::read_to_string(filename) {
        Ok(contents) => println!("{}", contents),
        Err(error) => {
            println!("Error reading file: {}", error);
            if let Some(os_error) = error.raw_os_error() {
                match os_error {
                    2 => println!("File not found."),
                    123 => println!("Syntax error in filename, folder name, or volume label."),
                    _ => println!("Unknown OS error code: {}", os_error),
                }
            } else {
                println!("Unknown error: {}", error);
            }
        }
    }
}