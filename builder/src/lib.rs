use std::path::Path; 

pub const TARGET: &str = "target.json";
pub const BOOTSTRAP_DIR: &str = "../bootloader/bootstrap";
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

