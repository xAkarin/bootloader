// use std::io::{Read, Write};

use booter::*;
fn main() {
    exec_cmd_wait(format!("cargo build --profile stage1 --target {}", TARGET)
                  .as_str(), 
                  format!("{}", BOOTSTRAP_DIR)
                  .as_str())
        .expect("Failed to compile the first stage");


    //exec_cmd_wait(format!("cargo build --profile stage2 --target ../{}", TARGET).as_str(), "second_stage").expect("Failed to compile the second stage");
    println!("[!] Creating bootable image...");

    // Requires you to install llvm tools
    // sudo apt install llvm 
    exec_cmd_wait(format!("llvm-objcopy -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 target/target/stage1/first_stage target/target/stage1/first_stage.bin")
                  .as_str(), 
                  format!("{}", BOOTSTRAP_DIR)
                  .as_str())
        .expect("Failed to objcopy the first stage");
    //exec_cmd_wait("llvm-objcopy -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 second_stage/target/x86_16bit/stage2/second_stage target/second_stage.bin", ".").expect("Failed to objcopy the second stage");
    //exec_cmd_wait("cp first_stage.bin disk_img.bin", "target").unwrap();
//   // Appends the two files
    //let mut data_file = std::fs::OpenOptions::new()
    //    .read(true)
    //    .open("target/second_stage.bin")
    //    .expect("cannot open file");
    //let mut disk_file = std::fs::OpenOptions::new()
    //    .append(true)
    //    .open("target/disk_img.bin")
    //    .expect("cannot open file");

    //let mut buffer = Vec::new();
    //data_file.read_to_end(&mut buffer).unwrap();
    //disk_file.write(&buffer)
    //    .expect("write failed");


    //println!("[!] Launching bootable image...");
    //exec_cmd_wait(
    //    "qemu-system-x86_64 -drive format=raw,file=target/disk_img.bin",
    //    ".",
    // )
    //.expect("Failed running qemu");
}