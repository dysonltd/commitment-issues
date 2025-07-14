fn main() {
    println!("cargo:rerun-if-changed=./.git/");
    println!("cargo:rustc-link-arg=-Tmetadata.x");
}
