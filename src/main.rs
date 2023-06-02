/*
 The highlight_match function takes a file name and a search pattern as input, and returns an optional string with the matched pattern highlighted in green using ANSI escape codes. If the file name doesn't contain the pattern, the function returns None.

The find_files function has been modified to use highlight_match to highlight the matched patterns in the file names. The modified function constructs the full path of each matching file by concatenating the directory path, the file name with the matched pattern highlighted, and a newline character.

Finally, the display_files function now takes the search pattern as an input, and calls highlight_match on each file path before printing it. If the highlight_match function returns a Some value, indicating that the file name contains the search pattern, the function prints the highlighted file name. Otherwise, it prints the original file name as-is.
 *
 */

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // get the starting directory and search pattern from command line arguments
    let args: Vec<String> = env::args().collect();
    let start_dir = Path::new(&args[1]);
    let pattern = &args[2];

    // recursively search the directory for files that match the pattern
    let files = find_files(start_dir, pattern);

    // display the paths of the files found
    display_files(&files, pattern);
}

fn find_files(dir: &Path, pattern: &str) -> Vec<String> {
    let mut files = Vec::new();

    // check if the current directory matches the search pattern
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // recursively search subdirectories
                    let mut sub_files = find_files(&path, pattern);
                    files.append(&mut sub_files);
                } else if let Some(file_name) = path.file_name() {
                    // check if the file name matches the search pattern
                    if let Some(colored_file_name) = highlight_match(file_name, pattern) {
                        let full_path = path.to_string_lossy().into_owned();
                        files.push(format!(
                            "{}{}",
                            &full_path[..full_path.len() - file_name.to_string_lossy().len()],
                            colored_file_name
                        ));
                    }
                }
            }
        }
    }

    files
}

fn display_files(files: &Vec<String>, pattern: &str) {
    for file in files {
        println!(
            "{}",
            highlight_match(file, pattern).unwrap_or_else(|| file.to_string())
        );
    }
}

fn highlight_match(file: &str, pattern: &str) -> Option<String> {
    if let Some(match_start) = file.find(pattern) {
        let match_end = match_start + pattern.len();
        let colored_file = format!(
            "{}{}{}",
            &file[..match_start],
            "\x1b[32m",
            &file[match_start..match_end],
            "\x1b[0m"
        );
        Some(colored_file)
    } else {
        None
    }
}
