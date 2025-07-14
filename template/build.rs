use std::path::PathBuf;

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let git_dir = find_valid_git_root!(current_dir).unwrap();
    println!("cargo:rerun-if-changed={}", git_dir);
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-arg=-Tmetadata.x");
}
