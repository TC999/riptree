use std::fs;
use std::path::Path;
use crate::{SHOW_HIDDEN, ONLY_DIRS, IGNORE_PATTERNS};

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
            if IGNORE_PATTERNS.is_some() {
                let patterns = IGNORE_PATTERNS.as_ref().unwrap();
                entries = entries
                    .into_iter()
                    .filter(|e| {
                        let name = e.file_name();
                        let name = name.to_string_lossy();
                        // 只做简单的后缀/前缀/全名匹配
                        !patterns.iter().any(|pat| {
                            if pat.ends_with('/') {
                                // 文件夹名匹配
                                pat.trim_end_matches('/') == name
                            } else if pat.starts_with("*") {
                                // 后缀匹配
                                name.ends_with(&pat[1..])
                            } else if pat.ends_with("*") {
                                // 前缀匹配
                                name.starts_with(&pat[..pat.len()-1])
                            } else {
                                // 全名匹配
                                pat == &name
                            }
                        })
                    })
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