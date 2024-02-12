pub fn exec_cmd_wait(cmd: &str, current_dir: &str) -> std::io::Result<std::process::ExitStatus> {
    println!("{cmd}");
    let mut args = cmd.split(" ");
    let mut cmd = std::process::Command::new(args.next().unwrap());
    for arg in args {
        cmd.arg(arg);
    }
    cmd.current_dir(current_dir);
    cmd.stdout(std::process::Stdio::piped());
    cmd.spawn()?.wait()
}
pub const TARGET: &str = "x86_16bit.json";
