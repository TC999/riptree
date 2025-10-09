use std::fs;
use std::path::Path;

fn print_tree(path: &Path, prefix: String) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.path());

        let len = entries.len();
        for (i, entry) in entries.into_iter().enumerate() {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            let is_last = i == len - 1;
            let connector = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", prefix, connector, file_name);

            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            if entry.path().is_dir() {
                print_tree(&entry.path(), new_prefix);
            }
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let root = Path::new(&path);
    println!("{}", root.display());
    print_tree(root, String::new());
}