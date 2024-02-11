use std::process::ExitStatus;

fn main() {
    println!("Creating bootable image...");
    exec_cmd_wait("objcopy -I elf32-i386 -O binary target/x86_16bit/release/bootloader_rust target/disk_image.bin", ".").expect("Failed objcopy");
    exec_cmd_wait("qemu-system-x86_64 -drive format=raw,file=target/disk_image.bin", ".").expect("Failed running qemu");
    
}

fn exec_cmd_wait(cmd: &str, current_dir: &str) -> std::io::Result<ExitStatus> {
    let mut args = cmd.split(" ");
    let mut cmd = std::process::Command::new(args.next().unwrap());
    for arg in args {
        cmd.arg(arg);
    }
    cmd.current_dir(current_dir);
    cmd.spawn()?.wait()
}

// objcopy -I elf32-i386 -O binary target/x86_16bit/release/bootloader_rust target/disk_image.bin && qemu-system-x86_64 -drive format=raw,file=target/disk_image.bin