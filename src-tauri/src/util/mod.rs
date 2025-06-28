use std::{fs, path::Path};

use sha2::{Digest, Sha256};

pub fn calculate_sha256(path: impl AsRef<Path>) -> std::io::Result<String> {
    let mut file = fs::File::open(&path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}
