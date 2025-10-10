pub fn is_dir_pruned(path: &std::path::Path) -> bool {
    match std::fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let file_type = entry.file_type();
                if let Ok(ft) = file_type {
                    if ft.is_file() {
                        return false;
                    }
                    if ft.is_dir() && !is_dir_pruned(&entry.path()) {
                        return false;
                    }
                }
            }
            true
        }
        Err(_) => false,
    }
}
