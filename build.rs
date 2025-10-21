use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("user.bin"))
        .compile_protos(&["proto/user.proto"], &["proto"])
        .unwrap();
    tonic_prost_build::compile_protos("proto/user.proto")?;
    Ok(())
}
