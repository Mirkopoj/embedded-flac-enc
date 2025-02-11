#![no_std]

pub trait ByteSink {
    fn write(&mut self, next_byte: u8);
}

pub struct BufferByteSink<const N: usize> {
    length: usize,
    buff: [u8; N],
}

impl<const N: usize> BufferByteSink<N> {
    pub fn new() -> Self {
        Self {
            length: 0,
            buff: [0; N],
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buff[0..self.length]
    }
}

impl<const N: usize> ByteSink for BufferByteSink<N> {
    fn write(&mut self, next_byte: u8) {
        self.buff[self.length] = next_byte;
        self.length += 1;
    }
}

mod frames;
mod metadata;
mod utils;
