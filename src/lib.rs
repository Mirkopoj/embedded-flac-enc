#![no_std]

pub trait ByteSink {
    fn write(&mut self, next_byte: u8);
}

/// Appends the next `num_bits` from `next_bits` to the sink.
///
/// calling `bit_sink.write(0b0001_0011, 3);` will append, `0b011`
/// calling `bit_sink.write(0b0001_0011, 4);` will append, `0b0011`
/// calling `bit_sink.write(0b0001_0011, 5);` will append, `0b10011`
/// calling `bit_sink.write(0b0001_0011, 6);` will append, `0b010011`
pub trait BitSink {
    fn write(&mut self, next_bits: u8, num_bits: u8);
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

    pub fn del_last(&mut self) {
        self.length -= 1;
    }
}

impl<const N: usize> ByteSink for BufferByteSink<N> {
    fn write(&mut self, next_byte: u8) {
        self.buff[self.length] = next_byte;
        self.length += 1;
    }
}

pub struct BitSinkAdapter<'a, BS: ByteSink> {
    bits: u8,
    buff: u8,
    sink: &'a mut BS,
}

impl<'a, BS: ByteSink> BitSinkAdapter<'a, BS> {
    pub fn new(sink: &'a mut BS) -> Self {
        Self {
            bits: 0,
            buff: 0,
            sink,
        }
    }
}

impl<BS: ByteSink> Drop for BitSinkAdapter<'_, BS> {
    fn drop(&mut self) {
        if self.bits != 0 {
            self.sink.write(self.buff);
        }
    }
}

impl<BS: ByteSink> BitSink for BitSinkAdapter<'_, BS> {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn write(&mut self, next_bits: u8, num_bits: u8) {
        assert!(num_bits <= 8);
        let shift = 8 - i32::from(num_bits) - i32::from(self.bits);
        if shift.is_negative() {
            let shift = -shift;
            let push = (next_bits >> shift) | self.buff;
            self.sink.write(push);
            self.bits = shift as u8;
            let shift = 8 - shift;
            self.buff = next_bits << shift;
        } else {
            self.buff |= next_bits << shift;
            self.bits = 8 - shift as u8;
            if self.bits == 8 {
                self.sink.write(self.buff);
                self.bits = 0;
                self.buff = 0;
            }
        }
    }
}

pub mod frames;
pub mod metadata;
mod utils;
