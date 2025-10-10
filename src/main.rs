// 是否显示隐藏文件
static mut SHOW_HIDDEN: bool = false;
static mut ONLY_DIRS: bool = false;

mod help;
mod print;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help") {
        println!("{}", help::HELP_TEXT);
        return;
    }
    // 解析参数，支持 tree -a 和 tree . -a
    let mut show_hidden = false;
    let mut only_dirs = false;
    let mut path = None;
    for arg in args.iter().skip(1) {
        if arg == "-a" {
            show_hidden = true;
        } else if arg == "-d" {
            only_dirs = true;
        } else if !arg.starts_with('-') && path.is_none() {
            path = Some(arg.clone());
        }
    }
    unsafe {
        SHOW_HIDDEN = show_hidden;
        ONLY_DIRS = only_dirs;
    }
    let path = path.unwrap_or_else(|| ".".to_string());
    let root = std::path::Path::new(&path);
    println!("{}", root.display());
    print::print_tree(root, String::new());
}