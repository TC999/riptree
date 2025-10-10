// 是否显示隐藏文件
static mut SHOW_HIDDEN: bool = false;
static mut ONLY_DIRS: bool = false;
static mut IGNORE_PATTERNS: Option<Vec<String>> = None;

mod help;
mod print;
mod args;

fn main() {
    let args = args::parse_args();
    if args.path.is_empty() {
        println!("{}", help::HELP_TEXT);
        return;
    }
    // 读取 .gitignore
    let mut ignore_patterns = Vec::new();
    if args.use_gitignore {
        let gitignore_path = std::path::Path::new(&args.path).join(".gitignore");
        if let Ok(content) = std::fs::read_to_string(&gitignore_path) {
            for line in content.lines() {
                let line = line.trim();
                if !line.is_empty() && !line.starts_with('#') {
                    ignore_patterns.push(line.to_string());
                }
            }
        }
    }
    unsafe {
        SHOW_HIDDEN = args.show_hidden;
        ONLY_DIRS = args.only_dirs;
        if !ignore_patterns.is_empty() {
            IGNORE_PATTERNS = Some(ignore_patterns);
        } else {
            IGNORE_PATTERNS = None;
        }
    }
    let root = std::path::Path::new(&args.path);
    println!("{}", root.display());
    print::print_tree(root, String::new());
}