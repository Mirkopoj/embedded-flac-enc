use header::{ChannelBits, FrameHeader};
use sub_frame::{SubFrame, SubFrameType};

use crate::{utils::crc16_remainder, BufferByteSink, ByteSink};

pub struct Frame<const CHANNELS: usize, const BLOCK_SIZE: usize> {
    header: FrameHeader,
    subframes: [SubFrame<BLOCK_SIZE>; CHANNELS],
}

impl<const CHANNELS: usize, const BLOCK_SIZE: usize> Frame<CHANNELS, BLOCK_SIZE> {
    pub fn new(
        sample_rate: u32,
        channel_bits: ChannelBits,
        bit_depth: u8,
        frame_number: u64,
        sub_frame_header: SubFrameType,
        wasted_bits: u8,
        samples: [i32; BLOCK_SIZE],
    ) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self {
            header: FrameHeader::new_fixed_size(
                BLOCK_SIZE as u16,
                sample_rate,
                channel_bits,
                bit_depth,
                frame_number,
            ),
            subframes: [SubFrame::new(sub_frame_header, wasted_bits, bit_depth, samples); CHANNELS],
        }
    }

    /// MEM = 16 + `BLOCK_SIZE` * 4 deberia ir
    pub fn write<BS: ByteSink, const MEM: usize>(&self, sink: &mut BS) {
        const CRC_POLYNOMIAL: u16 = 0b1000_0000_0000_0101;
        const CRC_INITIAL: u16 = 0b0000_0000_0000_0000;
        let mut buff: BufferByteSink<MEM> = BufferByteSink::new();
        self.header.write(&mut buff);
        assert_eq!(CHANNELS, 1, "Por ahora solo funca en mono");
        self.subframes
            .iter()
            .for_each(|&sub_frame| sub_frame.write(&mut buff));
        let crc = crc16_remainder(buff.as_slice(), CRC_POLYNOMIAL, CRC_INITIAL);
        buff.buff.iter().for_each(|&byte| sink.write(byte));
        crc.to_be_bytes().iter().for_each(|&byte| sink.write(byte));
    }
}

mod header;
mod sub_frame;
