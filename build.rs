use std::{env, fs, path::Path};

fn main() {
    let src_dir = Path::new("locales");

    // 标准 Cargo 环境变量
    let target = env::var("TARGET").ok();
    let host = env::var("HOST").ok();
    let cargo_target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());

    // 仅当 TARGET 存在且与 HOST 不同时，认为是显式指定了 --target（交叉编译）
    let use_triple = match (&target, &host) {
        (Some(t), Some(h)) => t != h,
        _ => false,
    };

    let release_dir = if use_triple {
        Path::new(&format!("{}/{}/release/locales", cargo_target_dir, target.clone().unwrap())).to_path_buf()
    } else {
        Path::new(&format!("{}/release/locales", cargo_target_dir)).to_path_buf()
    };

    let debug_dir = if use_triple {
        Path::new(&format!("{}/{}/debug/locales", cargo_target_dir, target.clone().unwrap())).to_path_buf()
    } else {
        Path::new(&format!("{}/debug/locales", cargo_target_dir)).to_path_buf()
    };

    // 输出更明确的调试信息，便于确认逻辑
    println!("cargo:warning=TARGET = {:?}", target);
    println!("cargo:warning=HOST   = {:?}", host);
    println!("cargo:warning=Interpreting as {} a cross-target.", if use_triple { "" } else { "not" });

    println!("cargo:warning=Copying locales to:");
    println!("cargo:warning=  release -> {}", release_dir.display());
    println!("cargo:warning=  debug   -> {}", debug_dir.display());

    if !src_dir.exists() {
        println!("cargo:warning=Source locales directory '{}' does not exist, skipping copy.", src_dir.display());
        return;
    }

    if let Err(e) = copy_ftl_files(src_dir, &release_dir) {
        panic!("failed to copy locales to release dir {}: {}", release_dir.display(), e);
    }
    if let Err(e) = copy_ftl_files(src_dir, &debug_dir) {
        panic!("failed to copy locales to debug dir {}: {}", debug_dir.display(), e);
    }
}

fn copy_ftl_files(src: &Path, dest_base: &Path) -> std::io::Result<()> {
    if !dest_base.exists() {
        fs::create_dir_all(dest_base)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("ftl") {
            let file_name = path.file_name().expect("file name should exist");
            let dest_path = dest_base.join(file_name);
            fs::copy(&path, &dest_path)?;
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    Ok(())
}