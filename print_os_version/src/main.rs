mod vars;

use std::process::Command;
fn main() {
    println!("Hello, world!");
    let proc = Command::new("wmic")
        .args(&["os", "get", "caption"])
        .output()
        .expect("Failed to start 'wmic'");
    println!("{}", String::from_utf8_lossy(&proc.stdout));
    vars::run();
}
