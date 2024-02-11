fn main() {
    exec_cmd_wait(format!("cargo build --release --target ../{}", TARGET).as_str(), "../first_stage").expect("Failed compiling first stage");
    exec_cmd_wait(format!("cargo build --release --target ../{}", TARGET).as_str(), "../second_stage").expect("Failed compiling second stage");
}

//TODO Get from main.rs
const TARGET: &str = "x86_16bit.json";
fn exec_cmd_wait(cmd: &str, current_dir: &str) -> std::io::Result<std::process::ExitStatus> {
    println!("cargo:warning={}", cmd);
    let mut args = cmd.split(" ");
    let mut cmd = std::process::Command::new(args.next().unwrap());
    for arg in args {
        cmd.arg(arg);
    }
    cmd.current_dir(current_dir);
    cmd.spawn()?.wait()
}
