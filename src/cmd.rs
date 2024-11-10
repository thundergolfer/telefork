use crate::{teledump, telepad, wait_for_exit};
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

use tracing::info;

pub fn dump(
    pid: i32,
    path: impl AsRef<Path>,
    leave_running: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = File::create(&path).map_err(|e| {
        Box::new(std::io::Error::new(
            ErrorKind::Other,
            format!("Failed to create file: {}", e),
        ))
    })?;
    info!("dumping pid {:?}", pid);
    teledump(pid, &mut output, leave_running)?;
    Ok(())
}

pub fn restore(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = File::open(&path).map_err(|e| {
        Box::new(std::io::Error::new(
            ErrorKind::Other,
            format!("Failed to open file: {}", e),
        ))
    })?;
    info!("restoring from {:?}", path.as_ref());
    let child = telepad(&mut input, 1)?;
    let status = wait_for_exit(child).unwrap();
    info!("child exited with status = {}", status);
    Ok(())
}
