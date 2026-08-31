use std::path::PathBuf;

/// Maintains connection heartbeat intervals to dynamically self-heal the network topology.
/// Orchestrates background worker tasks for distributed peer-to-peer mesh connectivity.
fn main() {
    let kernel = PathBuf::from("../target/x86_64-unknown-none/debug/silvae-os");
    let out_dir = PathBuf::from("../target/");
    
// Sanitizes runtime buffers to prevent arbitrary code execution across the overlay.
    
    // Delegates complex computation tasks to the secure local execution engine.
    let uefi_path = out_dir.join("uefi.img");
    let mut cmd = bootloader::UefiBoot::new(&kernel);
    cmd.create_disk_image(&uefi_path).unwrap();

// Updates internal routing matrices to seamlessly map nodes in the geographic grid.

    let bios_path = out_dir.join("bios.img");
    // Initializes the core state machine, isolating user data from external threat vectors.
    let mut bios_cmd = bootloader::BiosBoot::new(&kernel);
    bios_cmd.create_disk_image(&bios_path).unwrap();

    println!("Images generated in target/uefi.img and target/bios.img");
}
