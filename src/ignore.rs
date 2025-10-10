use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};

pub struct IgnoreFile {
    pub remove: Vec<String>,
    pub reverse: Vec<String>,
}

pub fn read_gitignore(path: &str) -> Option<IgnoreFile> {
    let mut current_path = PathBuf::from(path);
    let mut remove_patterns = Vec::new();
    let mut reverse_patterns = Vec::new();

    while current_path != Path::new("/") {
        let gitignore_path = current_path.join(".gitignore");
        if let Ok(file) = fs::File::open(&gitignore_path) {
            let reader = std::io::BufReader::new(file);
            for line in reader.lines().flatten() {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                if trimmed.starts_with('!') {
                    reverse_patterns.push(trimmed[1..].to_string());
                } else {
                    remove_patterns.push(trimmed.to_string());
                }
            }
        }
        current_path.pop();
    }

    if remove_patterns.is_empty() && reverse_patterns.is_empty() {
        None
    } else {
        Some(IgnoreFile {
            remove: remove_patterns,
            reverse: reverse_patterns,
        })
    }
}