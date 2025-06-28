use std::io::Result;

fn main() -> Result<()> {
    tonic_build::compile_protos("proto/fileshare.proto")?;
    tauri_build::build();

    Ok(())
}
