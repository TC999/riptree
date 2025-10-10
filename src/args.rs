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
    if args.iter().any(|a| a == "--help") {
        // 这里不直接打印帮助，交给 main 处理
        return Args {
            show_hidden: false,
            only_dirs: false,
            use_gitignore: false,
            prune: false,
            lang: None,
            path: String::new(),
        };
    }
    let mut show_hidden = false;
    let mut only_dirs = false;
    let mut use_gitignore = false;
    let mut prune = false;
    let mut lang = None;
    let mut path = None;
    for arg in args.iter().skip(1) {
        if arg == "-a" {
            show_hidden = true;
        } else if arg == "-d" {
            only_dirs = true;
        } else if arg == "--gitignore" {
            use_gitignore = true;
        } else if arg == "--prune" {
            prune = true;
        } else if arg.starts_with("--LANG=") {
            lang = Some(arg[7..].to_string());
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
        path: path.unwrap_or_else(|| ".".to_string()),
    }
}
