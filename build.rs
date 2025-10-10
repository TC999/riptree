use std::fs;
use std::path::Path;

fn main() {
    let src_dir = Path::new("locales");
    let target_release = Path::new("target/release/locales");
    let target_debug = Path::new("target/debug/locales");

    // 只在本地有 locales 文件夹时执行
    if src_dir.exists() {
        // 复制到 release 目录
        if !target_release.exists() {
            fs::create_dir_all(&target_release).unwrap();
        }
        for entry in fs::read_dir(src_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("ftl") {
                let file_name = path.file_name().unwrap();
                let dest_path = target_release.join(file_name);
                fs::copy(&path, &dest_path).unwrap();
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
        // 复制到 debug 目录
        if !target_debug.exists() {
            fs::create_dir_all(&target_debug).unwrap();
        }
        for entry in fs::read_dir(src_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("ftl") {
                let file_name = path.file_name().unwrap();
                let dest_path = target_debug.join(file_name);
                fs::copy(&path, &dest_path).unwrap();
            }
        }
    }
}