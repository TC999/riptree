use std::fs;
use std::path::Path;
use fluent::FluentArgs;
use crate::{SHOW_HIDDEN, ONLY_DIRS, IGNORE_PATTERNS, PRUNE, REPORT, SHOW_BYTES, SHOW_HUMAN};
use crate::prune::is_dir_pruned;
use crate::i18n::I18n;
use crate::color::colorize;

pub fn print_tree(path: &Path, prefix: String, i18n: &I18n, level: Option<usize>) {
    let mut total_dirs = 0;
    let mut total_files = 0;
    print_tree_count(path, prefix, &mut total_dirs, &mut total_files, i18n, level, 0);
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
            unsafe {
                if SHOW_HUMAN && !entry.path().is_dir() {
                    let size: u64 = match entry.metadata() {
                        Ok(m) => m.len(),
                        Err(_) => 0,
                    };
                    let human = human_readable(size);
                    // 固定 5 宽度显示（如 " 805", "1.5K"）
                    println!("{}{}[{:>5}]  {}", prefix, connector, human, file_name);
                } else if SHOW_BYTES && !entry.path().is_dir() {
                    // 获取字节大小，读取元数据失败时使用 0
                    let size: u64 = match entry.metadata() {
                        Ok(m) => m.len(),
                        Err(_) => 0,
                    };
                    // 固定宽度为 11 位（保持与示例一致），两位空格分隔
                    println!("{}{}[{:>11}]  {}", prefix, connector, size, file_name);
                } else {
                    if let Ok(metadata) = fs::symlink_metadata(&entry.path()) {
                        if metadata.file_type().is_symlink() {
                            if let Ok(target) = fs::read_link(&entry.path()) {
                                println!("{}{}{} -> {}", prefix, connector, file_name, target.display());
                                continue;
                            }
                        }
                    }
                    // Apply colorization to file and directory names.
                    let colorized_name = colorize(&entry.path(), &file_name);
                    println!("{}{}{}", prefix, connector, colorized_name);
                }
            }

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

// human_readable: 将字节数转换为简短字符串，保留一位小数。返回值长度可能超过 4（如 "1024.0K"），
// 但在打印处使用格式化限制宽度为 5 个字符（右对齐）。这里生成常见的 K/M/G/T 单位。
fn human_readable(size: u64) -> String {
    const K: f64 = 1024.0;
    let s = size as f64;
    if s < K {
        // 直接显示整数
        format!("{}", size)
    } else if s < K * K {
        format!("{:.1}K", s / K)
    } else if s < K * K * K {
        format!("{:.1}M", s / (K * K))
    } else if s < K * K * K * K {
        format!("{:.1}G", s / (K * K * K))
    } else {
        format!("{:.1}T", s / (K * K * K * K))
    }
}