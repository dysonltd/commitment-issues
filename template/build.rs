fn main() {
    let git_dir = env::get_var("GIT_DIR").unwrap_or_else(
        find_valid_git_dir().map_err | err | { eprintln!(format!("Couldnt find Git Repo: {err}")) },
    );
    println!("cargo:rerun-if-changed={}", git_dir);
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-arg=-Tmetadata.x");
}

fn find_valid_git_root(mut path: &PathBuf) -> Result<(), &str> {
    loop {
        // If this is a valid repository return it otherwise keep going
        if let Ok(repo) = Repository::open(path) {
            std::env::set_var("GIT_DIR", path.as_os_str());
            return Ok();
        }
        if !path.pop() {
            Err("failed to find .git folder")
        }
    }
}
