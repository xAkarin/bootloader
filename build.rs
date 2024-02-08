use std::env;

/// Stolen from Brightshards operating system
/// https://github.com/Bright-Shard/bs/blob/main/boot/bootstrapper/build.rs
fn main() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = std::path::Path::new(&root);

    println!(
        "cargo:rustc-link-arg-bins=--script={}",
        root.join("linker.ld").display()
    );
}
