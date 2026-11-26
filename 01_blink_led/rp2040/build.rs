//! build.rs - makes `memory.x` visible to the linker (see docs/02_memory_x_linker_startup.md)
use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    // OUT_DIR is a per-crate scratch directory that cargo creates for build artefacts.
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // Copy memory.x there so `link.x` (from cortex-m-rt) can `INCLUDE memory.x`.
    File::create(out.join("memory.x")).unwrap()
        .write_all(include_bytes!("memory.x")).unwrap();
    // Tell rustc's linker to search OUT_DIR for linker scripts.
    println!("cargo:rustc-link-search={}", out.display());
    // Re-run only when these files change.
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
