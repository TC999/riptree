use std::fs;
use std::path::Path;
use fluent::FluentArgs;
use crate::{SHOW_HIDDEN, ONLY_DIRS, IGNORE_PATTERNS, PRUNE, REPORT};
use crate::prune::is_dir_pruned;
use crate::i18n::I18n; // 新增：引入 i18n 模块

pub fn print_tree(path: &Path, prefix: String, i18n: &I18n, level: Option<usize>) {
    let mut total_dirs = 0;
    let mut total_files = 0;
    print_tree_count(path, prefix, &mut total_dirs, &mut total_files, i18n, level, 0);
    // 输出统计信息，使用 Fluent 国际化
    let mut args = FluentArgs::new();
    args.set("total_dirs", total_dirs);
    args.set("total_files", total_files);
    unsafe {
        if REPORT {
            if ONLY_DIRS {
                println!("{}", i18n.text("stats-only-dirs", Some(&args)));
            } else {
                println!("{}", i18n.text("stats-all", Some(&args)));
            }
        }
    }
}

fn print_tree_count(
    path: &Path,
    prefix: String,
    total_dirs: &mut usize,
    total_files: &mut usize,
    i18n: &I18n,
    level: Option<usize>,
    current_level: usize,
) {
    if let Some(max_level) = level {
        if current_level >= max_level {
            return;
        }
    }

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
            if IGNORE_PATTERNS.is_some() { // 注：此处警告暂时忽略
                let patterns = IGNORE_PATTERNS.as_ref().unwrap(); // 注：此处警告暂时忽略
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
            if PRUNE {
                entries = entries
                    .into_iter()
                    .filter(|e| {
                        if e.path().is_dir() {
                            !is_dir_pruned(&e.path())
                        } else {
                            true
                        }
                    })
                    .collect();
            }
        }
        entries.sort_by_key(|e| e.path());

        let len = entries.len();
        for (i, entry) in entries.into_iter().enumerate() {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            let is_last = i == len - 1;
            // 连接符直接用原始字符串，不做国际化
            let connector = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", prefix, connector, file_name);

            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            if entry.path().is_dir() {
                *total_dirs += 1;
                print_tree_count(&entry.path(), new_prefix, total_dirs, total_files, i18n, level, current_level + 1);
            } else {
                *total_files += 1;
            }
        }
    }
}