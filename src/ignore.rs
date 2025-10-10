use std::path::Path;

pub fn read_gitignore(path: &str) -> Option<Vec<String>> {
    let gitignore_path = Path::new(path).join(".gitignore");
    if let Ok(content) = std::fs::read_to_string(&gitignore_path) {
        let patterns: Vec<String> = content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(String::from)
            .collect();
        if !patterns.is_empty() {
            return Some(patterns);
        }
    }
    None
}