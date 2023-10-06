use std::{
    fs::{create_dir, read_dir, remove_dir_all, write},
    path::{Path, PathBuf},
};

use ethers::prelude::Abigen;

const DEFAULT_ABI_DIR: &str = "./abi";
const DEFAULT_OUT_DIR: &str = "./src/contracts";

fn list_files_recursively(
    folder_path: &Path,
    file_paths: &mut Vec<PathBuf>,
) -> Result<(), std::io::Error> {
    // Read the contents of the current folder
    let entries = read_dir(folder_path)?;

    // Iterate over the entries in the folder
    for entry in entries {
        // Unwrap the entry
        let entry = entry?;

        // Get the full path of the entry
        let entry_path = entry.path();

        // Check if it's a file (not a directory)
        if entry.file_type()?.is_file() {
            // Add the file path to the list
            file_paths.push(entry_path);
        } else if entry.file_type()?.is_dir() {
            // If it's a directory, recursively call the function
            list_files_recursively(&entry_path, file_paths)?;
        }
    }

    Ok(())
}

fn rust_file_generation(abi_paths: &Vec<PathBuf>) {
    let out_dir = DEFAULT_OUT_DIR;
    let _ = remove_dir_all(out_dir);
    create_dir(out_dir).unwrap();

    let mut mod_content = String::from("");

    for path in abi_paths {
        let file_name = path.file_name().unwrap().to_string_lossy();
        let file_name = file_name.split(".").next().unwrap();
        let out_file_name = to_snake_case(file_name);

        println!("{}", path.to_string_lossy());

        Abigen::new(file_name, path.to_string_lossy())
            .unwrap()
            .generate()
            .unwrap()
            .write_to_file(format!("{}/{}.rs", out_dir, out_file_name))
            .unwrap();
        mod_content.push_str(&format!("pub mod {};\n", out_file_name));
    }

    write(format!("{}/mod.rs", out_dir), mod_content).unwrap();
}

fn to_snake_case(input: &str) -> String {
    let mut snake_case = String::new();
    let mut prev_char_was_upper = false;

    for c in input.chars() {
        if c.is_ascii_uppercase() {
            if !snake_case.is_empty() && !prev_char_was_upper {
                snake_case.push('_');
            }
            snake_case.push(c.to_ascii_lowercase());
            prev_char_was_upper = true;
        } else {
            snake_case.push(c);
            prev_char_was_upper = false;
        }
    }

    snake_case
}

#[tokio::main]
async fn main() {
    // Specify the folder path you want to start listing files from
    let folder_path = Path::new(DEFAULT_ABI_DIR);

    // Create a vector to store the file paths
    let mut file_paths: Vec<PathBuf> = Vec::new();

    // Call the recursive function
    list_files_recursively(folder_path, &mut file_paths).unwrap();

    rust_file_generation(&file_paths);
}
