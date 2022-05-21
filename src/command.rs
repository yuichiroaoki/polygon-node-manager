use std::process::{Command, Stdio};

pub fn hello_world() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo Hello, world!")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

    let hello = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", hello);
}
