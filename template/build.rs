fn main() {
    println!("cargo:rerun-if-changed=./.git/");
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-arg=-Tmetadata.x");
}
