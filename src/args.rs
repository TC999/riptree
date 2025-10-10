use crate::help;
use crate::i18n;

pub struct Args {
    pub show_hidden: bool,
    pub only_dirs: bool,
    pub use_gitignore: bool,
    pub prune: bool,
    pub lang: Option<String>,
    pub path: String,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    let mut show_hidden = false;
    let mut only_dirs = false;
    let mut use_gitignore = false;
    let mut prune = false;
    let mut lang = None;
    let mut path = None;

    // 提前解析 --LANG 参数
    for arg in args.iter().skip(1) {
        if arg.starts_with("--LANG=") {
            let mut l = arg[7..].to_string();
            if let Some(dot_pos) = l.find('.') {
                l = l[..dot_pos].to_string();
            }
            l = l.replace('_', "-");
            lang = Some(l);
        }
    }

    // 根据 --LANG 参数或默认语言初始化 I18n 实例
    let detected_lang = lang.clone().unwrap_or_else(|| i18n::detect_lang());
    let i18n = i18n::I18n::new(&detected_lang);

    for arg in args.iter().skip(1) {
        if arg == "--help" {
            help::print_help(&i18n);
            std::process::exit(0);
        } else if arg == "-a" {
            show_hidden = true;
        } else if arg == "-d" {
            only_dirs = true;
        } else if arg == "--gitignore" {
            use_gitignore = true;
        } else if arg == "--prune" {
            prune = true;
        } else if !arg.starts_with('-') && path.is_none() {
            path = Some(arg.clone());
        }
    }

    Args {
        show_hidden,
        only_dirs,
        use_gitignore,
        prune,
        lang,
        path: path.unwrap_or_default(),
    }
}

pub struct InitData {
    pub lang: String,
    pub ignore_patterns: Option<Vec<String>>,
}

pub fn initialize() -> (Args, InitData) {
    let mut args = parse_args();

    // 如果未指定路径，默认使用当前目录
    if args.path.is_empty() {
        args.path = ".".to_string();
    }

    let lang = args.lang.clone().unwrap_or_else(|| i18n::detect_lang());

    // 读取 .gitignore
    let ignore_patterns = if args.use_gitignore {
        crate::ignore::read_gitignore(&args.path).map(|ig| ig.remove)
    } else {
        None
    };

    (
        args,
        InitData {
            lang,
            ignore_patterns,
        },
    )
}
