use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["proto/fileshare.proto"], &["proto/"])?;
    tauri_build::build();

    Ok(())
}
