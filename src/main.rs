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
    if args.path.is_empty() {
        println!("{}", help::HELP_TEXT);
        return;
    }
    // 读取 .gitignore
    let ignore_patterns = if args.use_gitignore {
        ignore::read_gitignore(&args.path).map(|ig| ig.remove)
    } else {
        None
    };
    unsafe {
        SHOW_HIDDEN = args.show_hidden;
        ONLY_DIRS = args.only_dirs;
        PRUNE = args.prune;
        IGNORE_PATTERNS = ignore_patterns;
    }
    let root = std::path::Path::new(&args.path);
    println!("{}", root.display());
    print::print_tree(root, String::new(), &i18n);
}