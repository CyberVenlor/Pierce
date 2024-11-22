use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("failed to open stdin");
        stdin.write_all(b"Hello, Rust!\n").expect("failed to write to stdin");
    }

    let output = child.wait_with_output().expect("failed to read stdout");
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
}

