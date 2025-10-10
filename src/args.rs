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
    let mut show_help = false;

    for arg in args.iter().skip(1) {
        if arg == "--help" {
            show_help = true;
        } else if arg == "-a" {
            show_hidden = true;
        } else if arg == "-d" {
            only_dirs = true;
        } else if arg == "--gitignore" {
            use_gitignore = true;
        } else if arg == "--prune" {
            prune = true;
        } else if arg.starts_with("--LANG=") {
            let mut l = arg[7..].to_string();
            if let Some(dot_pos) = l.find('.') {
                l = l[..dot_pos].to_string();
            }
            l = l.replace('_', "-");
            lang = Some(l);
        } else if !arg.starts_with('-') && path.is_none() {
            path = Some(arg.clone());
        }
    }

    if show_help {
        // 如果检测到 --help，仍然返回解析后的参数
        return Args {
            show_hidden,
            only_dirs,
            use_gitignore,
            prune,
            lang,
            path: String::new(),
        };
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
