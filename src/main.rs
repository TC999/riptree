// 是否显示隐藏文件
static mut SHOW_HIDDEN: bool = false;
static mut ONLY_DIRS: bool = false;
static mut IGNORE_PATTERNS: Option<Vec<String>> = None;
static mut PRUNE: bool = false;

mod help;
mod print;
mod args;
mod prune;
mod i18n;
mod ignore;

fn main() {
    let args = args::parse_args();
    let lang = args.lang.clone().unwrap_or_else(|| i18n::detect_lang());
    let i18n = i18n::I18n::new(&lang);

    // 打印当前语言设置，便于调试
    println!("当前语言: {}", lang);

    // 如果未指定路径，默认使用当前目录
    let path = if args.path.is_empty() {
        ".".to_string()
    } else {
        args.path.clone()
    };

    // 读取 .gitignore
    let ignore_patterns = if args.use_gitignore {
        ignore::read_gitignore(&path).map(|ig| ig.remove)
    } else {
        None
    };

    unsafe {
        SHOW_HIDDEN = args.show_hidden;
        ONLY_DIRS = args.only_dirs;
        PRUNE = args.prune;
        IGNORE_PATTERNS = ignore_patterns;
    }
    let root = std::path::Path::new(&path);
    println!("{}", root.display());
    print::print_tree(root, String::new(), &i18n);
}