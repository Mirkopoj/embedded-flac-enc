pub struct BitIterator<I: Iterator<Item = u8>> {
    iter: I,
    first: Option<u8>,
    last: Option<u8>,
    bit: u8,
}

impl<I: Iterator<Item = u8>> BitIterator<I> {
    pub fn new(mut iter: I) -> Self {
        let first = iter.next();
        let last = iter.next();
        Self {
            iter,
            first,
            last,
            bit: 0,
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for BitIterator<I> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.first?;
        let ret = if self.bit == 0 {
            first
        } else {
            let last = self.last?;
            first << self.bit | last >> (8 - self.bit)
        };
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.first = self.last;
            self.last = self.iter.next();
        }
        Some(ret)
    }
}

pub trait BitIter: Iterator<Item = u8> {
    fn bit_iter(self) -> BitIterator<Self>
    where
        Self: Sized,
    {
        BitIterator::new(self)
    }
}

impl<I: Iterator<Item = u8>> BitIter for I {}

pub fn crc8_remainder(bit_stream: &[u8], crc_polynomial: u8, initial: u8) -> u8 {
    let rigth_pad = [initial];
    let mut it = bit_stream
        .iter()
        .chain(rigth_pad.iter())
        .copied()
        .bit_iter();
    let mut res = it.next().unwrap();
    for next in it {
        let msb = res & 0b1000_0000;
        res = (res << 1) | (next & 1);
        if msb != 0b1000_0000 {
            continue;
        }
        res ^= crc_polynomial;
    }
    res
}

pub struct BitIterator16<I: Iterator<Item = u8>> {
    iter: I,
    first: Option<u8>,
    second: Option<u8>,
    last: Option<u8>,
    bit: u8,
}

impl<I: Iterator<Item = u8>> BitIterator16<I> {
    pub fn new(mut iter: I) -> Self {
        let first = iter.next();
        let second = iter.next();
        let last = iter.next();
        Self {
            iter,
            first,
            second,
            last,
            bit: 0,
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for BitIterator16<I> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let first = u16::from(self.first?);
        let second = u16::from(self.second?);
        let ret = if self.bit == 0 {
            first << 8 | second
        } else {
            let last = u16::from(self.last?);
            first << (8 + self.bit) | second << self.bit | last >> (8 - self.bit)
        };
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.first = self.second;
            self.second = self.last;
            self.last = self.iter.next();
        }
        Some(ret)
    }
}

pub trait BitIter16: Iterator<Item = u8> {
    fn bit_iter16(self) -> BitIterator16<Self>
    where
        Self: Sized,
    {
        BitIterator16::new(self)
    }
}

impl<I: Iterator<Item = u8>> BitIter16 for I {}

pub fn crc16_remainder(bit_stream: &[u8], crc_polynomial: u16, initial: u16) -> u16 {
    let rigth_pad = initial.to_be_bytes();
    let mut it = bit_stream
        .iter()
        .chain(rigth_pad.iter())
        .copied()
        .bit_iter16();
    let mut res = it.next().unwrap();
    for next in it {
        let msb = res & 0b1000_0000_0000_0000;
        res = (res << 1) | (next & 1);
        if msb != 0b1000_0000_0000_0000 {
            continue;
        }
        res ^= crc_polynomial;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{crc16_remainder, crc8_remainder, BitIter, BitIter16};

    #[test]
    fn bit_iter() {
        let slice: [u8; 3] = [0b0011_0100, 0b1100_0001, 0b0110_1100];
        let mut it = slice.iter().copied().bit_iter();
        assert_eq!(Some(0b0011_0100), it.next());
        assert_eq!(Some(0b0110_1001), it.next());
        assert_eq!(Some(0b1101_0011), it.next());
        assert_eq!(Some(0b1010_0110), it.next());
        assert_eq!(Some(0b0100_1100), it.next());
        assert_eq!(Some(0b1001_1000), it.next());
        assert_eq!(Some(0b0011_0000), it.next());
        assert_eq!(Some(0b0110_0000), it.next());
        assert_eq!(Some(0b1100_0001), it.next());
        assert_eq!(Some(0b1000_0010), it.next());
        assert_eq!(Some(0b0000_0101), it.next());
        assert_eq!(Some(0b0000_1011), it.next());
        assert_eq!(Some(0b0001_0110), it.next());
        assert_eq!(Some(0b0010_1101), it.next());
        assert_eq!(Some(0b0101_1011), it.next());
        assert_eq!(Some(0b1011_0110), it.next());
        assert_eq!(Some(0b0110_1100), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn crc() {
        let slice: [u8; 3] = [0b0011_0100, 0b1100_0001, 0b0110_1100];
        let crc = crc8_remainder(&slice, 7, 0);
        assert_eq!(crc, 0b1011_0001);
    }

    #[test]
    fn bit_iter16() {
        let slice: [u8; 3] = [0b0011_0100, 0b1100_0001, 0b0110_1100];
        let mut it = slice.iter().copied().bit_iter16();
        assert_eq!(Some(0b0011_0100_1100_0001), it.next());
        assert_eq!(Some(0b0110_1001_1000_0010), it.next());
        assert_eq!(Some(0b1101_0011_0000_0101), it.next());
        assert_eq!(Some(0b1010_0110_0000_1011), it.next());
        assert_eq!(Some(0b0100_1100_0001_0110), it.next());
        assert_eq!(Some(0b1001_1000_0010_1101), it.next());
        assert_eq!(Some(0b0011_0000_0101_1011), it.next());
        assert_eq!(Some(0b0110_0000_1011_0110), it.next());
        assert_eq!(Some(0b1100_0001_0110_1100), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn crc16() {
        let slice: [u8; 8] = [0xAF, 0x72, 0x3C, 0x4C, 0x1E, 0x06, 0xC9, 0xA0];
        let crc = crc16_remainder(&slice, 32773, 0);
        assert_eq!(crc, 0x9753);
    }
}
