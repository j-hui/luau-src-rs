use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let artifacts = luau0_src::Build::new().no_std(true).build();
    artifacts.print_cargo_metadata();

    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    // Specify linker arguments.

    // `--nmagic` is required if memory section addresses are not aligned to 0x10000,
    // for example the FLASH and RAM sections in your `memory.x`.
    // See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    // println!("cargo:rustc-link-arg=--nmagic");

    // Set the linker script to the one provided by cortex-m-rt.
    // println!("cargo:rustc-link-arg=-Tlink.x");

    // println!(
    //     "cargo:rustc-link-search={}/lib",
    //     "/Applications/ArmGNUToolchain/12.2.mpacbti-rel1/arm-none-eabi/bin/../arm-none-eabi"
    // );
    // println!("cargo:rustc-link-lib=static=c");
    // println!("cargo:rustc-link-lib=static=m");
    // println!("cargo:rustc-link-lib=static=nosys");
    // // println!("cargo:rustc-link-lib=static=gloss");
    // // println!("cargo:rustc-link-lib=static=g");
    // println!("cargo:rustc-link-lib=static=stdc++");
    // println!("cargo:rustc-link-lib=static=supc++");
}
