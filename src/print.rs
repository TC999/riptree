use std::fs;
use std::path::Path;
use crate::{SHOW_HIDDEN, ONLY_DIRS};

pub fn print_tree(path: &Path, prefix: String) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        // 过滤隐藏文件
        unsafe {
            if !SHOW_HIDDEN {
                entries = entries
                    .into_iter()
                    .filter(|e| {
                        if let Some(name) = e.file_name().to_str() {
                            !name.starts_with('.')
                        } else {
                            true
                        }
                    })
                    .collect();
            }
            if ONLY_DIRS {
                entries = entries
                    .into_iter()
                    .filter(|e| e.path().is_dir())
                    .collect();
            }
        }
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