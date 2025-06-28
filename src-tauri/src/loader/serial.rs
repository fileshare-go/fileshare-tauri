use std::{
    fs,
    io::{self, Read, Seek, SeekFrom},
    path::Path,
};

use crate::loader::ChunkLoader;

pub struct SerialLoader {
    file: fs::File,
    buf_size: i64,
    seek_at: i32,
}

impl SerialLoader {
    pub fn new(path: impl AsRef<Path>, buf_size: i64) -> Self {
        let file = fs::File::open(path).unwrap();
        Self {
            file,
            buf_size,
            seek_at: -1,
        }
    }
}

impl ChunkLoader for SerialLoader {
    fn load(&mut self, idx: i32) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; self.buf_size as usize];
        let gap = idx - self.seek_at;
        if gap != 1 {
            self.file
                .seek(SeekFrom::Current(gap as i64 * self.buf_size))?;
        }
        self.seek_at += gap;
        let len = self.file.read(&mut buf)?;
        buf.truncate(len);

        return Ok(buf);
    }
}
