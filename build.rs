use std::{env::var, io, fs, path::Path};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

const DIR: &str = "ss_src";
pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", DIR);
    
    let src_dir = Path::new(&var("CARGO_MANIFEST_DIR").unwrap()).join(DIR);
    let dest_dir = Path::new(&var("CARGO_MANIFEST_DIR").unwrap()).join("target").join(var("PROFILE").unwrap()).join(DIR);

    if let Err(_) = copy_dir_all(src_dir, dest_dir) {
        println!("cargo:warning=failed to copy {}", DIR);
    }
}