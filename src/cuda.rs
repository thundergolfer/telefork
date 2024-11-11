use memfd_exec::{MemFdExecutable, Stdio};

/// Returns the cuda-checkpoint executable as an array of bytes.
///
/// The cuda-checkpoint executable is used to checkpoint/restore CUDA state.
#[cfg(target_os = "linux")]
fn get_cuda_checkpoint_binary() -> &'static [u8] {
    include_bytes!(concat!(env!("OUT_DIR"), "/cuda-checkpoint"))
}

/// Run cuda-checkpoint.
/// Ref: https://github.com/NVIDIA/cuda-checkpoint
pub fn checkpoint(pid: i32) -> Result<(), Box<dyn std::error::Error>> {
    // The `MemFdExecutable` struct is at near feature-parity with `std::process::Command`,
    // so you can use it in the same way. The only difference is that you must provide the
    // executable contents as a `Vec<u8>` as well as telling it the argv[0] to use.
    let c = MemFdExecutable::new("cuda-checkpoint", get_cuda_checkpoint_binary())
        .arg("--toggle")
        .args(["--pid", &pid.to_string().as_str()])
        // We'll capture the stdout of the process, so we need to set up a pipe.
        .stdout(Stdio::piped())
        // Spawn the process as a forked child
        .spawn()?;

    // Get the output and status code of the process (this will block until the process
    // exits)
    let output = c.wait_with_output()?;
    assert!(output.status.into_raw() == 0);
    Ok(())
}
