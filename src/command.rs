use std::process::Command;

pub fn start_bor() {
    println!("start_bor");
    Command::new("sudo")
        .arg("service")
        .arg("heimdalld")
        .arg("start")
        .output()
        .expect("failed to execute process");
}

pub fn stop_bor() {
    println!("stop_bor");
    Command::new("sudo")
        .arg("service")
        .arg("heimdalld")
        .arg("stop")
        .output()
        .expect("failed to execute process");
}
