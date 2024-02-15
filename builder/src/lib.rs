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
pub fn remove_elf_16(){
    todo!()
}

#[allow(dead_code)]
pub fn ensure_size_512(file_name: &str) -> bool{
    let file = std::fs::File::open(&file_name).expect(format!("Failed to open file: {}", &file_name).as_str()); 

    if file.metadata().unwrap().size() > 512 {
        return false; 
    } 

    return true;  
}

