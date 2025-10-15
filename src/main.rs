mod help;
mod print;
mod args;
mod prune;
mod i18n;
mod ignore;

// 是否显示隐藏文件
static mut SHOW_HIDDEN: bool = false;
static mut ONLY_DIRS: bool = false;
static mut IGNORE_PATTERNS: Option<Vec<String>> = None;
static mut PRUNE: bool = false;
static mut REPORT: bool = true;
static mut SHOW_BYTES: bool = false; // 新增静态变量以控制是否显示文件大小

fn main() {
    let (args, init_data) = args::initialize();
    let i18n = i18n::I18n::new(&init_data.lang);

    unsafe {
        SHOW_HIDDEN = args.show_hidden;
        ONLY_DIRS = args.only_dirs;
        PRUNE = args.prune;
        IGNORE_PATTERNS = init_data.ignore_patterns;
        REPORT = args.report;
        SHOW_BYTES = args.show_bytes; // 赋值静态变量
    }

    let root = std::path::Path::new(&args.path);
    println!("{}", root.display());
    print::print_tree(root, String::new(), &i18n, args.level); // 传递 level 参数
}