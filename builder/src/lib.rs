use std::{os::unix::fs::MetadataExt, path::Path}; 

pub const STAGE1_DIR: &str = "";

pub fn exec_cmd_wait(command: &str, current_dir: &str) -> std::io::Result<std::process::ExitStatus> { 
    let mut args = command.split(" ");
    
    let mut cmd = std::process::Command::new(args.next().unwrap());
    
    for arg in args {
        cmd.arg(arg);
    }
    
    cmd.current_dir(current_dir);
    
    assert_eq!(cmd.get_current_dir(), Some(Path::new(current_dir)));

    println!("Running command {command} in {}", cmd.get_current_dir().unwrap().to_str().expect("Failed to convert path to string")); 

    cmd.stdout(std::process::Stdio::piped());
    
    return cmd.spawn()?.wait();
}

#[allow(dead_code)]
pub fn remove_elf_16(input_location: &str, output_location: &str, directory: &str){
    let elf_remover = std::process::Command::new("llvm-objcopy")
        .arg("-I")
        .arg("elf64-x86-64")
        .arg("-O")
        .arg("binary")
        .arg("--binary-architecture=i386:x86-64")
        .arg(input_location)
        .arg(output_location)
        .current_dir(directory)
        .spawn()
        .unwrap()
        .wait();

    if elf_remover.is_err() {
        panic!("[!] Failed to remove elf headers from bin!");
    }
}

#[allow(dead_code)]
pub fn ensure_size_512(file_name: &str) { 
    let file = std::fs::File::open(file_name).expect(format!("Failed to open file: {}", file_name).as_str()); 
    
    let size = file.metadata().unwrap().size(); 

    if size > 512 {
        panic!("[!] the size of {} is greater than 512 bytes!", file_name);
    }

    println!("The size of {} is {}", file_name, size); 
}

