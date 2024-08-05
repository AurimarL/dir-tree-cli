use clap::{Arg, Command};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a folder in the file system.
#[derive(Debug)]
struct Folder {
    name: String,
    subfolders: Vec<Folder>,
    files: Vec<String>,
}

/// List of folder names to be ignored during traversal.
const IGNORED_FOLDERS: [&str; 6] = [
    "node_modules",
    "target",
    ".next",
    ".ssh",
    "coverage",
    ".git",
];

/// Checks if a folder should be ignored based on its name.
///
/// # Arguments
///
/// * `folder_name` - The name of the folder to check.
///
/// # Returns
///
/// Returns `true` if the folder should be ignored, otherwise `false`.
fn should_ignore(folder_name: &str) -> bool {
    IGNORED_FOLDERS.contains(&folder_name)
}

/// Recursively retrieves the folder structure starting from the given path.
///
/// # Arguments
///
/// * `path` - The path to the folder to be traversed.
///
/// # Returns
///
/// Returns a `Folder` struct representing the folder and its contents.
fn get_folders(path: &Path) -> Folder {
    let name = path.file_name()
        .map_or_else(|| String::from(""), |n| n.to_string_lossy().to_string());

    let mut subfolders = Vec::new();
    let mut files = Vec::new();

    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let entry_path = entry.path();
                let folder_name = entry_path.file_name()
                    .map_or_else(|| String::from(""), |n| n.to_string_lossy().to_string());

                if entry_path.is_dir() {
                    if !should_ignore(&folder_name) {
                        subfolders.push(get_folders(&entry_path));
                    }
                } else if entry_path.is_file() {
                    let file_name = entry_path.file_name()
                        .map_or_else(|| String::from(""), |n| n.to_string_lossy().to_string());
                    files.push(file_name);
                }
            }
        } else {
            eprintln!("Error reading directory: {}", path.display());
        }
    }

    Folder {
        name,
        subfolders,
        files,
    }
}

/// Prints the folder structure in a tree-like format.
///
/// # Arguments
///
/// * `folder` - The folder to print.
/// * `indent` - The current indentation level.
/// * `is_last` - Indicates if this is the last item at the current level.
fn print_folder_structure(folder: &Folder, indent: usize, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    println!("{:indent$}{}{}", "", connector, folder.name, indent = indent);

    for (i, file) in folder.files.iter().enumerate() {
        let is_last_file = i == folder.files.len() - 1;
        let file_connector = if is_last_file { "└── " } else { "├── " };
        println!("{:indent$}{}{}", "", file_connector, file, indent = indent + 2);
    }

    for (i, subfolder) in folder.subfolders.iter().enumerate() {
        print_folder_structure(subfolder, indent + 2, i == folder.subfolders.len() - 1);
    }
}

/// The main function that sets up the command line interface and starts the folder structure viewer.
fn main() {
    let matches = Command::new("Folder Structure Viewer")
        .version("1.0")
        .author("Aurimar Lopes <aurimardev@gmail.com>")
        .about("Displays the folder structure")
        .arg(
            Arg::new("path")
                .help("Sets the path to the folder")
                .required(true)
                .index(1)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .get_matches();

    let path: PathBuf = matches.get_one::<PathBuf>("path").unwrap().clone();

    let folder_structure = get_folders(&path);
    print_folder_structure(&folder_structure, 0, true);
}