if variable::get("is_init") {
	if variable::prompt("Does your project contain a build.rs script?", false) {
		file::delete("build.rs");

		print();
		print("*** Add the following statements to your build.rs:");
		print();
    	print("println!(\"cargo:rerun-if-changed={}\", commitment_issues::find_valid_git_root!());");
		print("#[cfg(not(target_os = \"macos\"))]");
		print("println!(\"cargo:rustc-link-arg=-Tmetadata.x\");");
		print();
	}

	if variable::prompt("Does your project contain a .cargo/config.toml file?", false) {
		file::delete(".cargo/config.toml");

		print();
		print("*** Add the following to your .cargo/config.toml:");
		print();
		print("[net]");
		print("git-fetch-with-cli = true");
		print();
	}

	print("*** To add the commitment issues crate to your project, run:");
	print();
	print("cargo add --git https://github.com/dysonltd/commitment-issues");
	print();

	variable::set("project-name", "existing project");
}
