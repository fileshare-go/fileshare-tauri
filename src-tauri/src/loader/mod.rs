use std::io;

mod jumper;
mod serial;

pub trait ChunkLoader {
    fn load(&mut self, index: i32) -> io::Result<Vec<u8>>;
}

pub use serial::SerialLoader;
