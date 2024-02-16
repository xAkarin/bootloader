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
    let raw_bootstrap = remove_elf_16(get_bootstrap());
    ensure_size_512(&raw_bootstrap);

    let raw_stage_1 = remove_elf_16(get_stage_1());
    let size = get_size(&raw_stage_1);
    println!("Stage 1 is {size} bytes");

    append_file(&raw_bootstrap, &raw_stage_1, COMPILED_BOOTLOADER_LOC);
    
    setup_partition_size(COMPILED_BOOTLOADER_LOC.to_string(), size).expect("Failed opening compiled bootloader to set the partition size");

    let user_args = std::env::args()
        .skip(1)
        .map(|a| format!("{a} "))
        .collect::<String>();
    cmd!(
        "qemu-system-x86_64 -drive file={},format=raw {user_args}",
        COMPILED_BOOTLOADER_LOC
    );
}
