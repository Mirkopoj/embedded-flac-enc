use core::cmp::{max, min};

use crate::ByteSink;

#[derive(Clone, Copy)]
pub struct StreamInfo {
    min_block_size: u16,
    max_block_size: u16,
    min_frame_size: u32,
    max_frame_size: u32,
    sample_rate: u32,
    channels: u8,
    bits_per_sample: u8,
    interchannel_sample_count: u64,
    md5_checksum: u128,
}

impl StreamInfo {
    pub fn new(sample_rate: u32, channels: u8, bits_per_sample: u8) -> Self {
        Self {
            min_block_size: u16::MAX,
            max_block_size: u16::MIN,
            min_frame_size: u32::MAX,
            max_frame_size: u32::MIN,
            sample_rate,
            channels,
            bits_per_sample,
            interchannel_sample_count: 0,
            md5_checksum: 0,
        }
    }

    pub fn added_block_with(&mut self, size: u16) {
        self.min_block_size = min(self.min_block_size, size);
        self.max_block_size = max(self.max_block_size, size);
        self.interchannel_sample_count += u64::from(size);
    }

    pub fn added_frame_with(&mut self, size: u32) {
        self.min_frame_size = min(self.min_frame_size, size);
        self.max_frame_size = max(self.max_frame_size, size);
    }


    pub fn write<BS: ByteSink>(&self, sink: &mut BS) {
        sink.write(0);
        sink.write(0);
        sink.write(34);
        self.min_block_size
            .to_be_bytes()
            .iter()
            .chain(self.max_block_size.to_be_bytes().iter())
            .chain(self.min_frame_size.to_be_bytes()[1..].iter())
            .chain(self.max_frame_size.to_be_bytes()[1..].iter())
            .for_each(|&byte| sink.write(byte));
        sink.write(((self.sample_rate >> 12) & 0xFF) as u8);
        sink.write(((self.sample_rate >> 4) & 0xFF) as u8);
        sink.write(
            (((self.sample_rate & 0xF) as u8) << 4)
                | (((self.channels - 1) & 0x7) << 1)
                | (((self.bits_per_sample - 1) >> 4) & 1),
        );
        sink.write(
            ((self.bits_per_sample - 1) << 4)
                | (((self.interchannel_sample_count >> 32) & 0xF) as u8),
        );
        self.interchannel_sample_count.to_be_bytes()[4..]
            .iter()
            .chain(self.md5_checksum.to_be_bytes().iter())
            .for_each(|&byte| sink.write(byte));
    }
}
