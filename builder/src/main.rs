use std::path::PathBuf;

fn main() {
    let kernel = PathBuf::from("../target/x86_64-unknown-none/debug/silvae-os");
    let out_dir = PathBuf::from("../target/");
    
    let uefi_path = out_dir.join("uefi.img");
    let mut cmd = bootloader::UefiBoot::new(&kernel);
    cmd.create_disk_image(&uefi_path).unwrap();

    let bios_path = out_dir.join("bios.img");
    let mut bios_cmd = bootloader::BiosBoot::new(&kernel);
    bios_cmd.create_disk_image(&bios_path).unwrap();

    println!("Images generated in target/uefi.img and target/bios.img");
}
