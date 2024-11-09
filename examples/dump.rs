use std::fs::File;

use telefork::teledump;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No arguments provided",
        )));
    }
    let pid = &args[1];
    let pid: i32 = pid.parse().unwrap();
    let fname = "dump.telefork.bin";
    let mut output = File::create(fname).unwrap();
    println!("dumping pid {:?}", pid);
    teledump(pid, &mut output, true)?;
    Ok(())
}
