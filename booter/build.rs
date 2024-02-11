include!("src/lib.rs");

fn main() {
    exec_cmd_wait(format!("cargo build --release --target ../{}", TARGET).as_str(), "../first_stage").expect("Failed compiling first stage");
    exec_cmd_wait(format!("cargo build --release --target ../{}", TARGET).as_str(), "../second_stage").expect("Failed compiling second stage");
}