use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Define the URL of the file to download
    let url = "https://github.com/NVIDIA/cuda-checkpoint/blob/main/bin/x86_64_Linux/cuda-checkpoint?raw=true";
    let filename = "cuda-checkpoint";

    // Determine the output directory for the binary
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable is not set");
    let dest_path = Path::new(&out_dir).join(filename);

    // Download the binary using curl
    let status = Command::new("curl")
        .arg("-L") // Follow redirects
        .arg("-o")
        .arg(&dest_path)
        .arg(url)
        .status()
        .expect("Failed to execute curl");

    if !status.success() {
        panic!("Failed to download cuda-checkpoint");
    }

    // Make the binary executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest_path)
            .expect("Failed to retrieve metadata")
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest_path, perms).expect("Failed to set permissions");
    }

    // Print cargo metadata to add the binary to the build process
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=OUT_DIR");
}
