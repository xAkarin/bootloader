use booter::*;

const TARGET: &str = "target.json"; 
const BOOTSTRAP_DIR: &str = "../bootloader/bootstrap";
const COMPILED_BOOTSTRAP_LOC: &str = "target/target/bootstrap/bootstrap.bin"; 
const STAGE_ONE_DIR: &str = "../bootloader/stage_one";
const COMPILED_STAGE_ONE_LOC: &str = "target/target/stage1/stage_one.bin"; 


// TODO: clean build up!
fn main() {
    exec_cmd_wait(format!("cargo build --profile bootstrap --target {}", TARGET)
                  .as_str(), 
                  format!("{}", BOOTSTRAP_DIR)
                  .as_str())
        .expect("Failed to compile the first stage");


    exec_cmd_wait(format!("cargo build --profile stage1 --target {}", TARGET)
                  .as_str(), 
                  format!("{}", STAGE_ONE_DIR)
                  .as_str())
        .expect("[!] Failed to compile the first stage");

    println!("[?] Creating bootable image...");

    remove_elf_16("target/target/bootstrap/bootstrap" , "target/target/bootstrap/bootstrap.bin", BOOTSTRAP_DIR);

    ensure_size_512(format!("{}/{}", BOOTSTRAP_DIR, COMPILED_BOOTSTRAP_LOC).as_str());

    remove_elf_16("target/target/stage1/stage_one", "target/target/stage1/stage_one.bin", STAGE_ONE_DIR);

    print_size_in_bytes(format!("{}/{}", STAGE_ONE_DIR, COMPILED_STAGE_ONE_LOC).as_str()); 
    
    // TODO: clean this function up and make it concat the files to a specific directory 
    append_file(format!("{}/{}", STAGE_ONE_DIR, COMPILED_STAGE_ONE_LOC).as_str(), 
                format!("{}/{}", BOOTSTRAP_DIR, COMPILED_BOOTSTRAP_LOC).as_str()); 
    
}
