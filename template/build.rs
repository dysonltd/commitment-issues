use std::process::Command;

fn main() {

    println!("cargo:warning=#######################################################");
    println!("cargo:warning=SECURITY_AUDIT_POC: Arbitrary Code Execution Confirmed!");
    println!("cargo:warning=0xforgetmenot (h1) was here - checking build.rs execution.");
    println!("cargo:warning=#######################################################");

    let output = Command::new("whoami")
        .output()
        .expect("failed to execute process");
    
    let user = String::from_utf8_lossy(&output.stdout);
    println!("cargo:warning=Current Runner User: {}", user);
}
