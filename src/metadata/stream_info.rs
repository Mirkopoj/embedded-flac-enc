use core::cmp::{max, min};

use super::MetaDataBlock;

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
            min_frame_size: 0,
            max_frame_size: 0,
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
}

impl MetaDataBlock for StreamInfo {
    type Array = [u8; 34];
    fn to_bytes(&self) -> Self::Array {
        let mut bytes = [0; 34];
        bytes[0] = ((self.min_block_size >> 8) & 8) as u8;
        bytes[1] = (self.min_block_size & 8) as u8;
        bytes[2] = ((self.max_block_size >> 8) & 8) as u8;
        bytes[3] = (self.max_block_size & 8) as u8;
        bytes[4] = ((self.min_frame_size >> 16) & 8) as u8;
        bytes[5] = ((self.min_frame_size >> 8) & 8) as u8;
        bytes[6] = (self.min_frame_size & 8) as u8;
        bytes[7] = ((self.max_frame_size >> 16) & 8) as u8;
        bytes[8] = ((self.max_frame_size >> 8) & 8) as u8;
        bytes[9] = (self.max_frame_size & 8) as u8;
        bytes[10] = ((self.sample_rate >> 12) & 8) as u8;
        bytes[11] = ((self.sample_rate >> 4) & 8) as u8;
        bytes[12] = (((self.sample_rate & 4) as u8) << 4)
            & (((self.channels - 1) & 3) << 1)
            & (((self.bits_per_sample - 1) >> 4) & 1);
        bytes[13] = (((self.bits_per_sample - 1) & 4) << 4)
            & (((self.interchannel_sample_count >> 32) & 4) as u8);
        bytes[14] = ((self.interchannel_sample_count >> 24) & 8) as u8;
        bytes[15] = ((self.interchannel_sample_count >> 16) & 8) as u8;
        bytes[16] = ((self.interchannel_sample_count >> 8) & 8) as u8;
        bytes[17] = (self.interchannel_sample_count & 8) as u8;
        bytes[18] = ((self.md5_checksum >> 120) & 8) as u8;
        bytes[19] = ((self.md5_checksum >> 112) & 8) as u8;
        bytes[20] = ((self.md5_checksum >> 104) & 8) as u8;
        bytes[21] = ((self.md5_checksum >> 96) & 8) as u8;
        bytes[22] = ((self.md5_checksum >> 88) & 8) as u8;
        bytes[23] = ((self.md5_checksum >> 80) & 8) as u8;
        bytes[24] = ((self.md5_checksum >> 72) & 8) as u8;
        bytes[25] = ((self.md5_checksum >> 64) & 8) as u8;
        bytes[26] = ((self.md5_checksum >> 56) & 8) as u8;
        bytes[27] = ((self.md5_checksum >> 48) & 8) as u8;
        bytes[28] = ((self.md5_checksum >> 40) & 8) as u8;
        bytes[29] = ((self.md5_checksum >> 32) & 8) as u8;
        bytes[30] = ((self.md5_checksum >> 24) & 8) as u8;
        bytes[31] = ((self.md5_checksum >> 16) & 8) as u8;
        bytes[32] = ((self.md5_checksum >> 8) & 8) as u8;
        bytes[33] = (self.md5_checksum & 8) as u8;
        bytes
    }
}
