use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use crate::loader::ChunkLoader;

pub struct JumperLoader {
    file: fs::File,
    step_size: i32,
    buf_size: usize,
}

impl JumperLoader {
    pub fn new(path: impl AsRef<Path>, buf_size: usize, step_size: i32) -> Self {
        let file = fs::File::open(path).unwrap();
        Self {
            file,
            step_size,
            buf_size,
        }
    }
}

// impl ChunkLoader for JumperLoader {
//     fn load(&mut self) -> io::Result<()> {
//         let mut buf = Vec::with_capacity(self.buf_size);
//         self.file.read_exact(&mut buf)?;

//         return Ok(());
//     }
// }
