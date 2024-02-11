use booter::*;
fn main() {
    println!("Creating bootable image...");
    exec_cmd_wait("objcopy -I elf32-i386 -O binary target/x86_16bit/release/bootloader_rust target/disk_image.bin", ".").expect("Failed objcopy");
    println!("Launching bootable image...");
    exec_cmd_wait("qemu-system-x86_64 -drive format=raw,file=target/disk_image.bin", ".").expect("Failed running qemu");
}
