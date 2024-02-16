use booter::*;

pub fn get_bootstrap() -> String {
    format!("{BOOTSTRAP_DIR}/target/{BOOTSTRAP_TARGET}/{BOOTSTRAP_PROFILE}/bootstrap")
}

const BOOTSTRAP_DIR: &str = "../bootloader/bootstrap";
const BOOTSTRAP_TARGET: &str = "bootstrap";
const BOOTSTRAP_PROFILE: &str = "bootstrap";

const STAGE_ONE_DIR: &str = "../bootloader/stage_one";
const STAGE_1_TARGET: &str = "stage_1";
const STAGE_1_PROFILE: &str = "stage1";
pub fn get_stage_1() -> String {
    format!("{STAGE_ONE_DIR}/target/{STAGE_1_TARGET}/{STAGE_1_PROFILE}/stage_one")
}

const COMPILED_BOOTLOADER_LOC: &str = "target/disk.bin";

// TODO: clean build up!
fn main() {
    crate::cmd!(
        panic = "Failed building bootstrapper",
        dir = BOOTSTRAP_DIR,
        "cargo build --profile {BOOTSTRAP_PROFILE} --target {BOOTSTRAP_TARGET}.json",
    );

    crate::cmd!(
        panic = "Failed building first stage",
        dir = STAGE_ONE_DIR,
        "cargo build --profile {STAGE_1_PROFILE} --target {STAGE_1_TARGET}.json",
    );

    println!("\t[?] Creating bootable image...");
    let raw_bootstrap = remove_elf_16(
        get_bootstrap(),
    );
    ensure_size_512(&raw_bootstrap);
    
    let raw_stage_1 = remove_elf_16(
        get_stage_1(),
    );
    print_size_in_bytes(&raw_stage_1);

    append_file(
        &raw_bootstrap,
        &raw_stage_1,
        COMPILED_BOOTLOADER_LOC,
    );

    cmd!("qemu-system-x86_64 -drive file={},format=raw", COMPILED_BOOTLOADER_LOC);
}
