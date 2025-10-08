use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // ----------------------
    // memory.x handling
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");

    // ----------------------
    // Assemble image_def.s
    let src = "image_def.s";
    let obj = out.join("image_def.o");

    println!("cargo:rerun-if-changed={}", src);

    let status = Command::new("arm-none-eabi-as")
        .args([
            "-mcpu=cortex-m33",
            "-mthumb",
            "-o",
            obj.to_str().unwrap(),
            src,
        ])
        .status()
        .expect("failed to run assembler");

    assert!(status.success(), "assembler failed");

    println!("cargo:rustc-link-arg={}", obj.display());

    // ----------------------
    // Linker arguments
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
}