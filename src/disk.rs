use std::error::Error;
use std::process::Command;
use sysinfo::{DiskExt, System, SystemExt};

pub fn available_disk() -> Result<u64, Box<dyn Error>> {
    let sys = System::new_all();
    // ref. https://docs.rs/sysinfo/latest/sysinfo/struct.Disk.html
    for disk in sys.disks() {
        let mount_point = disk.mount_point();
        if mount_point.to_str() == Some("/") {
            return Ok(disk.available_space());
        }
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "root disk not found",
    )))
}

#[test]
fn test_available_disk() {
    assert_eq!(available_disk().unwrap() > 0, true);
}

pub fn delete_logs() {
    Command::new("sudo")
        .arg("journalctl")
        .arg("--vacuum-time")
        .arg("3h")
        .output()
        .expect("failed to execute process");
}
