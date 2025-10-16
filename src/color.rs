use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::io::IsTerminal;

/// Minimal color support: parse LS_COLORS/TREE_COLORS entries like "di=01;34:ln=01;36:ex=01;32"
/// and provide helpers to wrap names with ANSI escapes. This is intentionally small and
/// safe for the Rust reimplementation.

#[derive(Default)]
pub struct ColorTable {
    map: HashMap<String, String>,
    left: String,
    right: String,
    reset: String,
    enabled: bool,
}

impl ColorTable {
    pub fn new() -> Self {
        let mut tbl = ColorTable {
            map: HashMap::new(),
            left: "\x1b[".to_string(),
            right: "m".to_string(),
            reset: "0".to_string(),
            enabled: false,
        };

        // Honor NO_COLOR
        if env::var_os("NO_COLOR").is_some() {
            tbl.enabled = false;
            return tbl;
        }

        // If stdout is not a tty, disable colors by default unless forced
        let force = env::var_os("CLICOLOR_FORCE").is_some();
        if !std::io::stdout().is_terminal() && !force {
            tbl.enabled = false;
            return tbl;
        }

        // Read TREE_COLORS first, fallback to LS_COLORS
        if let Some(s) = env::var_os("TREE_COLORS").or_else(|| env::var_os("LS_COLORS")) {
            if let Ok(s) = s.into_string() {
                for part in s.split(':') {
                    if part.is_empty() { continue; }
                    if let Some(eq) = part.find('=') {
                        let key = &part[..eq];
                        let val = &part[eq+1..];
                        // store simple mapping like "di" -> "01;34"
                        tbl.map.insert(key.to_string(), val.to_string());
                    }
                }
                tbl.enabled = true;
            }
        }

        tbl
    }

    pub fn enabled(&self) -> bool { self.enabled }

    fn wrap(&self, code: &str, name: &str) -> String {
        if !self.enabled { return name.to_string(); }
        format!("{}{}{}{}{}", self.left, code, self.right, name, format!("{}{}{}", self.left, self.reset, self.right))
    }

    pub fn color_for_path(&self, path: &Path) -> Option<String> {
        if path.is_dir() {
            return self.map.get("di").cloned();
        }
        // symlink
        if let Ok(md) = std::fs::symlink_metadata(path) {
            if md.file_type().is_symlink() {
                return self.map.get("ln").cloned();
            }
            // executable
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if (md.permissions().mode() & 0o111) != 0 {
                    return self.map.get("ex").cloned();
                }
            }
        }
        // extension based: look for *.ext entries like "*.rs=01;32"
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            if let Some(pos) = name.rfind('.') {
                let ext = &name[pos..]; // includes dot
                let key = format!("*{}", ext);
                if let Some(v) = self.map.get(&key) {
                    return Some(v.clone());
                }
            }
        }
        // fallback to normal file
        self.map.get("fi").cloned()
    }

    pub fn colorize_name(&self, path: &Path, name: &str) -> String {
        if !self.enabled { return name.to_string(); }
        if let Some(code) = self.color_for_path(path) {
            self.wrap(&code, name)
        } else {
            name.to_string()
        }
    }
}

// Provide a global lazy-initialized table to avoid parsing multiple times.
use std::sync::OnceLock;
static CTABLE: OnceLock<ColorTable> = OnceLock::new();

pub fn init() {
    let _ = CTABLE.get_or_init(|| ColorTable::new());
}

pub fn enabled() -> bool {
    CTABLE.get_or_init(|| ColorTable::new()).enabled()
}

pub fn colorize(path: &Path, name: &str) -> String {
    CTABLE.get_or_init(|| ColorTable::new()).colorize_name(path, name)
}
