use crate::{utils::crc8_remainder, BufferByteSink, ByteSink};

pub struct FrameHeader {
    boundary: Boundary,
    block_size_bits: BlockSizeBits,
    sample_rate_bits: SampleRateBits,
    channel_bits: ChannelBits,
    bit_depth_bits: BitDepthBits,
    coded_num: CodedNum,
}

impl FrameHeader {
    pub fn new_fixed_size(
        block_size: u16,
        sample_rate: u32,
        channel_bits: ChannelBits,
        bit_depth: u8,
        frame_number: u64,
    ) -> Self {
        Self {
            boundary: Boundary::FixedBlockSize,
            block_size_bits: BlockSizeBits::from_u16(block_size),
            sample_rate_bits: SampleRateBits::from_u32(sample_rate),
            channel_bits,
            bit_depth_bits: BitDepthBits::from_u8(bit_depth),
            coded_num: CodedNum::new(frame_number),
        }
    }

    pub fn new_variable_size(
        block_size: u16,
        sample_rate: u32,
        channel_bits: ChannelBits,
        bit_depth: u8,
        sample_number: u64,
    ) -> Self {
        Self {
            boundary: Boundary::VariableBlockSize,
            block_size_bits: BlockSizeBits::from_u16(block_size),
            sample_rate_bits: SampleRateBits::from_u32(sample_rate),
            channel_bits,
            bit_depth_bits: BitDepthBits::from_u8(bit_depth),
            coded_num: CodedNum::new(sample_number),
        }
    }

    pub fn write<BS: ByteSink>(&self, sink: &mut BS) {
        const CRC_POLYNOMIAL: u8 = 0b0000_0111;
        const CRC_INITIAL: u8 = 0b0000_0000;
        let mut buff: BufferByteSink<16> = BufferByteSink::new();
        (self.boundary as u16)
            .to_be_bytes()
            .iter()
            .for_each(|&byte| buff.write(byte));
        buff.write(self.block_size_bits.as_u8() << 4 | self.sample_rate_bits.as_u8());
        buff.write(self.channel_bits.as_u8() << 4 | self.bit_depth_bits.as_u8() << 1);
        self.coded_num.write(&mut buff);
        match self.block_size_bits {
            BlockSizeBits::Uncommon8Bit(byte) => buff.write(byte),
            BlockSizeBits::Uncommon16Bit(bytes) => bytes
                .to_be_bytes()
                .iter()
                .for_each(|&byte| buff.write(byte)),
            _ => (),
        }
        match self.sample_rate_bits {
            SampleRateBits::Uncommon8Bit(byte) => buff.write(byte),
            SampleRateBits::Uncommon16Bit(bytes) | SampleRateBits::Uncommon16BitDiv10(bytes) => {
                bytes
                    .to_be_bytes()
                    .iter()
                    .for_each(|&byte| buff.write(byte));
            }
            _ => (),
        }
        let crc = crc8_remainder(buff.as_slice(), CRC_POLYNOMIAL, CRC_INITIAL);
        buff.write(crc);
        buff.as_slice().iter().for_each(|&byte| sink.write(byte));
    }
}

#[derive(Clone, Copy)]
enum Boundary {
    FixedBlockSize = 0xFFF8,
    VariableBlockSize = 0xFFF9,
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum BlockSizeBits {
    B192 = 0b0001,
    B576 = 0b0010,
    B1152 = 0b0011,
    B2304 = 0b0100,
    B4608 = 0b0101,
    Uncommon8Bit(u8) = 0b0110,
    Uncommon16Bit(u16) = 0b0111,
    B256 = 0b1000,
    B512 = 0b1001,
    B1024 = 0b1010,
    B2048 = 0b1011,
    B4096 = 0b1100,
    B8192 = 0b1101,
    B16384 = 0b1110,
    B32768 = 0b1111,
}

impl BlockSizeBits {
    pub fn from_u16(block_size: u16) -> Self {
        const U8_MAX: u16 = u8::MAX as u16;
        #[allow(clippy::cast_possible_truncation)]
        match block_size {
            192 => Self::B192,
            576 => Self::B576,
            1152 => Self::B1152,
            2304 => Self::B2304,
            4608 => Self::B4608,
            256 => Self::B256,
            512 => Self::B512,
            1024 => Self::B1024,
            2048 => Self::B2048,
            4096 => Self::B4096,
            8192 => Self::B8192,
            16384 => Self::B16384,
            32768 => Self::B32768,
            0..=U8_MAX => Self::Uncommon8Bit(block_size as u8),
            _ => Self::Uncommon16Bit(block_size),
        }
    }

    pub fn as_u8(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum SampleRateBits {
    SampleRateOnlyStoredInTheStreaminfoMetadataBlock = 0b0000,
    KHz88_2 = 0b0001,
    KHz176_4 = 0b0010,
    KHz192 = 0b0011,
    KHz8 = 0b0100,
    KHz16 = 0b0101,
    KHz22_05 = 0b0110,
    KHz24 = 0b0111,
    KHz32 = 0b1000,
    KHz44_1 = 0b1001,
    KHz48 = 0b1010,
    KHz96 = 0b1011,
    Uncommon8Bit(u8) = 0b1100,
    Uncommon16Bit(u16) = 0b1101,
    Uncommon16BitDiv10(u16) = 0b1110,
}

impl SampleRateBits {
    pub fn from_u32(sample_rate: u32) -> Self {
        const U8_LIM: u32 = u8::MAX as u32 + 1;
        const U16_LIM: u32 = u16::MAX as u32 + 1;
        #[allow(clippy::cast_possible_truncation)]
        match sample_rate {
            88_200 => Self::KHz88_2,
            176_400 => Self::KHz176_4,
            192_000 => Self::KHz192,
            8_000 => Self::KHz8,
            16_000 => Self::KHz16,
            22_050 => Self::KHz22_05,
            24_000 => Self::KHz24,
            32_000 => Self::KHz32,
            44_100 => Self::KHz44_1,
            48_000 => Self::KHz48,
            96_000 => Self::KHz96,
            0..U8_LIM => Self::Uncommon8Bit(sample_rate as u8),
            U8_LIM..U16_LIM => Self::Uncommon16Bit(sample_rate as u16),
            _ if sample_rate % 10 == 0 => Self::Uncommon16Bit((sample_rate / 10) as u16),
            _ => Self::SampleRateOnlyStoredInTheStreaminfoMetadataBlock,
        }
    }

    pub fn as_u8(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

#[derive(Clone, Copy)]
enum ChannelBits {
    Mono = 0b0000,
    LeftRight = 0b0001,
    LeftRightCenter = 0b0010,
    FrontleftFrontrightBackleftBackright = 0b0011,
    FrontleftFrontrightFrontcenterBackleftBackright = 0b0100,
    FrontleftFrontrightFrontcenterLfeBackleftBackright = 0b0101,
    FrontleftFrontrightFrontcenterLfeBackcenterSideleftSideright = 0b0110,
    FrontleftFrontrightFrontcenterLfeBackleftBackrightSideleftSideright = 0b0111,
    LeftRightStoredAsLeftMinusSideAndStereo = 0b1000,
    LeftRightStoredAsSideMinusightAndStereo = 0b1001,
    LeftRightStoredAsMidMinusSideAndStereo = 0b1010,
}

impl ChannelBits {
    pub fn as_u8(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

#[derive(Clone, Copy)]
enum BitDepthBits {
    BitDepthOnlyStoredInTheStreaminfoMetadataBlock = 0b000,
    BitsPerSample8 = 0b001,
    BitsPerSample12 = 0b010,
    BitsPerSample16 = 0b100,
    BitsPerSample20 = 0b101,
    BitsPerSample24 = 0b110,
    BitsPerSample32 = 0b111,
}

impl BitDepthBits {
    pub fn from_u8(bit_depth: u8) -> Self {
        match bit_depth {
            8 => Self::BitsPerSample8,
            12 => Self::BitsPerSample12,
            16 => Self::BitsPerSample16,
            20 => Self::BitsPerSample20,
            24 => Self::BitsPerSample24,
            32 => Self::BitsPerSample32,
            _ => Self::BitDepthOnlyStoredInTheStreaminfoMetadataBlock,
        }
    }

    pub fn as_u8(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

struct CodedNum {
    length: u8,
    code: [u8; 7],
}

impl CodedNum {
    fn new(num: u64) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let (length, code) = match num {
            0x0000_0000_0000..=0x0000_0000_007F => (1, [num as u8, 0, 0, 0, 0, 0, 0]),
            0x0000_0000_0080..=0x0000_0000_07FF => (
                2,
                [
                    0b1100_0000 | (0b0001_1111 & (num >> 6) as u8),
                    0b1000_0000 | (0b0011_1111 & num as u8),
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ),
            0x0000_0000_0800..=0x0000_0000_FFFF => (
                3,
                [
                    0b1110_0000 | (0b0000_1111 & (num >> 12) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 6) as u8),
                    0b1000_0000 | (0b0011_1111 & num as u8),
                    0,
                    0,
                    0,
                    0,
                ],
            ),
            0x0000_0001_0000..=0x0000_001F_FFFF => (
                4,
                [
                    0b1111_0000 | (0b0000_0111 & (num >> 18) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 12) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 6) as u8),
                    0b1000_0000 | (0b0011_1111 & num as u8),
                    0,
                    0,
                    0,
                ],
            ),
            0x0000_0020_0000..=0x0000_03FF_FFFF => (
                5,
                [
                    0b1111_1000 | (0b0000_0011 & (num >> 24) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 18) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 12) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 6) as u8),
                    0b1000_0000 | (0b0011_1111 & num as u8),
                    0,
                    0,
                ],
            ),
            0x0000_0400_0000..=0x0000_7FFF_FFFF => (
                6,
                [
                    0b1111_1100 | (0b0000_0001 & (num >> 30) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 24) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 18) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 12) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 6) as u8),
                    0b1000_0000 | (0b0011_1111 & num as u8),
                    0,
                ],
            ),
            0x0000_8000_0000..=0x000F_FFFF_FFFF => (
                7,
                [
                    0b1111_1110,
                    0b1000_0000 | (0b0011_1111 & (num >> 30) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 24) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 18) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 12) as u8),
                    0b1000_0000 | (0b0011_1111 & (num >> 6) as u8),
                    0b1000_0000 | (0b0011_1111 & num as u8),
                ],
            ),
            _ => (0, [0; 7]),
        };
        Self { length, code }
    }

    pub fn write<BS: ByteSink>(&self, sink: &mut BS) {
        let length = self.length as usize;
        self.code[0..length]
            .iter()
            .for_each(|&byte| sink.write(byte));
    }
}
