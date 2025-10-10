use crate::help;

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

    Args {
        show_hidden,
        only_dirs,
        use_gitignore,
        prune,
        lang,
        path: path.unwrap_or_default(),
    }
}

//fn print_help() {
//    println!("Usage: riptree [OPTIONS] [PATH]\n\nOptions:\n  --help          Show this help message\n  -a              Show hidden files\n  -d              Only show directories\n  --gitignore     Respect .gitignore files\n  --prune         Prune empty directories\n  --LANG=<lang>   Set the language (e.g., en-US, zh-CN)");
//}
