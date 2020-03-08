use std::env;
use std::fs;
use std::path::Path;


fn main()
{
    let proj_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let proj_dir = Path::new(&proj_dir);

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let mem_script = fs::read(proj_dir.join("src/memory.ld")).unwrap();
    fs::write(out_dir.join("memory.ld"), mem_script).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=src/memory.ld");
}
