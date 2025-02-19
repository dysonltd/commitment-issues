use commitment_issues::include_metadata;

include_metadata!();

fn main() {
    println!("Hello, world!");

    println!("\n Here is the binary's metadata:\n");
    println!("Schema version:  {}", metadata::schema());
    println!("Compile time:    {}", metadata::compile_time());
    println!("Commit hash:     {}", metadata::short_hash());
    println!("Is dirty build:  {}", metadata::is_dirty());
    println!("Tag description: {}", metadata::tag_describe());
    println!("Last author:     {}", metadata::last_author());
}
