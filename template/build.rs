use std::env;

fn main() {
    let git_dir = env::var("PROJECT_GIT_DIR").unwrap_or_else(|_| "./.git/".to_string());
    println!("cargo:rerun-if-changed={}", git_dir);
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-arg=-Tmetadata.x");
}
