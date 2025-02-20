use core::{cmp::min, ops::Sub};

use crate::{BitSink, BitSinkAdapter, ByteSink};

#[derive(Clone, Copy)]
pub struct SubFrame<const N: usize> {
    header: SubFrameType,
    wasted_bits: u8,
    bit_depth: u8,
    samples: [i32; N],
}

impl<const N: usize> SubFrame<N> {
    pub fn new(header: SubFrameType, wasted_bits: u8, bit_depth: u8, samples: [i32; N]) -> Self {
        Self {
            header,
            wasted_bits,
            bit_depth,
            samples,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn write<BS: ByteSink>(&self, sink: &mut BS) {
        let wasted_bits_flag = u8::from(self.wasted_bits != 0);
        let header = ((self.header as u8) << 1) | wasted_bits_flag;
        sink.write(header);
        let mut bit_sink = BitSinkAdapter::new(sink);
        if self.wasted_bits != 0 {
            unary_code(u32::from(self.wasted_bits) - 1, &mut bit_sink);
        };
        match self.header {
            SubFrameType::Constant => (self.samples[0] >> self.wasted_bits)
                .to_be_bytes()
                .iter()
                .for_each(|&byte| bit_sink.write(byte, 8)),
            SubFrameType::Verbatim => self.samples.iter().for_each(|&sample| {
                write_sample(sample, &mut bit_sink, self.bit_depth, self.wasted_bits);
            }),
            SubFrameType::FixedPredictorOrder0 => self.wirte_predictor::<0>(
                ResidualCodingMethod::Rice4Bits,
                &RiceParams::Param(5),
                &mut bit_sink,
                |_, _| 0,
            ),
            SubFrameType::FixedPredictorOrder1 => self.wirte_predictor::<1>(
                ResidualCodingMethod::Rice4Bits,
                &RiceParams::Param(5),
                &mut bit_sink,
                |x, _| x[0],
            ),
            SubFrameType::FixedPredictorOrder2 => self.wirte_predictor::<2>(
                ResidualCodingMethod::Rice4Bits,
                &RiceParams::Param(5),
                &mut bit_sink,
                |x, cero| 2 * x[cero] - x[(cero - 1).rem_euclid(2)],
            ),
            SubFrameType::FixedPredictorOrder3 => self.wirte_predictor::<3>(
                ResidualCodingMethod::Rice4Bits,
                &RiceParams::Param(5),
                &mut bit_sink,
                |x, cero| {
                    3 * x[cero] - 3 * x[(cero - 1).rem_euclid(3)] + x[(cero - 2).rem_euclid(3)]
                },
            ),
            SubFrameType::FixedPredictorOrder4 => self.wirte_predictor::<4>(
                ResidualCodingMethod::Rice4Bits,
                &RiceParams::Param(5),
                &mut bit_sink,
                |x, cero| {
                    4 * x[cero] - 6 * x[(cero - 1).rem_euclid(4)] + 4 * x[(cero - 2).rem_euclid(4)]
                        - x[(cero - 3).rem_euclid(4)]
                },
            ),
            SubFrameType::LinearPredictorOrder1 => todo!(),
            SubFrameType::LinearPredictorOrder2 => todo!(),
            SubFrameType::LinearPredictorOrder3 => todo!(),
            SubFrameType::LinearPredictorOrder4 => todo!(),
            SubFrameType::LinearPredictorOrder5 => todo!(),
            SubFrameType::LinearPredictorOrder6 => todo!(),
            SubFrameType::LinearPredictorOrder7 => todo!(),
            SubFrameType::LinearPredictorOrder8 => todo!(),
            SubFrameType::LinearPredictorOrder9 => todo!(),
            SubFrameType::LinearPredictorOrder10 => todo!(),
            SubFrameType::LinearPredictorOrder11 => todo!(),
            SubFrameType::LinearPredictorOrder12 => todo!(),
            SubFrameType::LinearPredictorOrder13 => todo!(),
            SubFrameType::LinearPredictorOrder14 => todo!(),
            SubFrameType::LinearPredictorOrder15 => todo!(),
            SubFrameType::LinearPredictorOrder16 => todo!(),
            SubFrameType::LinearPredictorOrder17 => todo!(),
            SubFrameType::LinearPredictorOrder18 => todo!(),
            SubFrameType::LinearPredictorOrder19 => todo!(),
            SubFrameType::LinearPredictorOrder20 => todo!(),
            SubFrameType::LinearPredictorOrder21 => todo!(),
            SubFrameType::LinearPredictorOrder22 => todo!(),
            SubFrameType::LinearPredictorOrder23 => todo!(),
            SubFrameType::LinearPredictorOrder24 => todo!(),
            SubFrameType::LinearPredictorOrder25 => todo!(),
            SubFrameType::LinearPredictorOrder26 => todo!(),
            SubFrameType::LinearPredictorOrder27 => todo!(),
            SubFrameType::LinearPredictorOrder28 => todo!(),
            SubFrameType::LinearPredictorOrder29 => todo!(),
            SubFrameType::LinearPredictorOrder30 => todo!(),
            SubFrameType::LinearPredictorOrder31 => todo!(),
            SubFrameType::LinearPredictorOrder32 => todo!(),
        }
    }

    /// TODO:
    ///     - partitions
    fn wirte_predictor<const ORDER: usize>(
        &self,
        method: ResidualCodingMethod,
        params: &RiceParams,
        bit_sink: &mut impl BitSink,
        predictor: impl Fn(&[i32], usize) -> i32,
    ) {
        bit_sink.write(method as u8, 2);
        let partition_order = 0;
        bit_sink.write(partition_order, 4);
        params.write(bit_sink, method);
        self.samples[..ORDER]
            .iter()
            .for_each(|&byte| write_sample(byte, bit_sink, self.bit_depth, self.wasted_bits));
        let mut bufer = [0; ORDER];
        for (slot, &value) in bufer.iter_mut().zip(self.samples[ORDER..].iter()) {
            *slot = value;
        }
        self.samples[ORDER..]
            .iter()
            .copied()
            .predict(bufer, predictor)
            .for_each(|error| residual_codeing(params, bit_sink, error));
    }
}

#[derive(Clone, Copy)]
pub enum SubFrameType {
    Constant = 0b000_000,
    Verbatim = 0b000_001,
    FixedPredictorOrder0 = 0b001_000,
    FixedPredictorOrder1 = 0b001_001,
    FixedPredictorOrder2 = 0b001_010,
    FixedPredictorOrder3 = 0b001_011,
    FixedPredictorOrder4 = 0b001_100,
    LinearPredictorOrder1 = 0b100_000,
    LinearPredictorOrder2 = 0b100_001,
    LinearPredictorOrder3 = 0b100_010,
    LinearPredictorOrder4 = 0b100_011,
    LinearPredictorOrder5 = 0b100_100,
    LinearPredictorOrder6 = 0b100_101,
    LinearPredictorOrder7 = 0b100_110,
    LinearPredictorOrder8 = 0b100_111,
    LinearPredictorOrder9 = 0b101_000,
    LinearPredictorOrder10 = 0b10_1001,
    LinearPredictorOrder11 = 0b101_010,
    LinearPredictorOrder12 = 0b101_011,
    LinearPredictorOrder13 = 0b101_100,
    LinearPredictorOrder14 = 0b101_101,
    LinearPredictorOrder15 = 0b101_110,
    LinearPredictorOrder16 = 0b101_111,
    LinearPredictorOrder17 = 0b110_000,
    LinearPredictorOrder18 = 0b110_001,
    LinearPredictorOrder19 = 0b110_010,
    LinearPredictorOrder20 = 0b110_011,
    LinearPredictorOrder21 = 0b110_100,
    LinearPredictorOrder22 = 0b110_101,
    LinearPredictorOrder23 = 0b110_110,
    LinearPredictorOrder24 = 0b110_111,
    LinearPredictorOrder25 = 0b111_000,
    LinearPredictorOrder26 = 0b111_001,
    LinearPredictorOrder27 = 0b111_010,
    LinearPredictorOrder28 = 0b111_011,
    LinearPredictorOrder29 = 0b111_100,
    LinearPredictorOrder30 = 0b111_101,
    LinearPredictorOrder31 = 0b111_110,
    LinearPredictorOrder32 = 0b111_111,
}

fn write_sample(sample: i32, bit_sink: &mut impl BitSink, bit_depth: u8, wasted_bits: u8) {
    let wasted_sample = sample >> wasted_bits;
    let used_bits = bit_depth - wasted_bits;
    let full_bytes = usize::from(used_bits) / 8;
    let partial = used_bits % 8;
    let partial_bytes = usize::from(partial != 0);
    let skip = 4 - partial_bytes - full_bytes;
    let bytes = wasted_sample.to_be_bytes();
    let mut iter = bytes.into_iter().skip(skip);
    if partial != 0 {
        if let Some(next) = iter.next() {
            bit_sink.write(next, partial);
        }
    }
    iter.for_each(|byte| {
        bit_sink.write(byte, 8);
    });
}

struct Predictor<
    const N: usize,
    I: Iterator<Item = S>,
    S: Sub<Output = S> + Copy,
    F: Fn(&[I::Item], usize) -> I::Item,
> {
    iter: I,
    prev: [I::Item; N],
    last: usize,
    predication_fn: F,
}

impl<
        const N: usize,
        I: Iterator<Item = S>,
        S: Sub<Output = S> + Copy,
        F: Fn(&[I::Item], usize) -> I::Item,
    > Predictor<N, I, I::Item, F>
{
    fn new(iter: I, prev: [I::Item; N], predictor: F) -> Self {
        Self {
            iter,
            prev,
            last: N - 1,
            predication_fn: predictor,
        }
    }
}

impl<
        const N: usize,
        I: Iterator<Item = S>,
        S: Sub<Output = S> + Copy,
        F: Fn(&[I::Item], usize) -> I::Item,
    > Iterator for Predictor<N, I, I::Item, F>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let actual = self.iter.next()?;
        let prediction = (self.predication_fn)(&self.prev, self.last);
        self.prev[self.last] = prediction;
        self.last += 1;
        self.last %= self.prev.len();
        Some(actual - prediction)
    }
}

trait PredictorIter<
    const N: usize,
    F: Fn(&[Self::Item], usize) -> Self::Item,
    S: Sub<Output = S> + Copy,
>: Iterator<Item = S>
{
    fn predict(self, warm_up: [Self::Item; N], predictor: F) -> Predictor<N, Self, Self::Item, F>
    where
        Self: Sized,
    {
        Predictor::new(self, warm_up, predictor)
    }
}

impl<
        I: Iterator<Item = S>,
        S: Sub<Output = S> + Copy,
        const N: usize,
        F: Fn(&[I::Item], usize) -> Self::Item,
    > PredictorIter<N, F, S> for I
{
}

#[derive(Clone, Copy)]
enum ResidualCodingMethod {
    Rice4Bits = 0b00,
    Rice5Bits = 0b01,
}

enum RiceParams {
    Param(u8),
    Escape(u8),
}

impl RiceParams {
    pub fn write(&self, bit_sink: &mut impl BitSink, method: ResidualCodingMethod) {
        let bits = match method {
            ResidualCodingMethod::Rice4Bits => 4,
            ResidualCodingMethod::Rice5Bits => 5,
        };
        match self {
            RiceParams::Param(par) => bit_sink.write(*par, bits),
            RiceParams::Escape(_) => bit_sink.write(0xFF, bits),
        }
    }
}

#[allow(clippy::cast_sign_loss)]
fn signed_fold(n: i32) -> u32 {
    (if n.is_negative() { n * (-2) - 1 } else { n * 2 }) as u32
}

#[allow(clippy::cast_possible_truncation)]
fn unary_code(code: u32, bit_sink: &mut impl BitSink) {
    let mut remaining = code;
    while remaining > 0 {
        let push = min(remaining, 8);
        remaining -= push;
        bit_sink.write(0, push as u8);
    }
    bit_sink.write(1, 1);
}

#[allow(clippy::cast_possible_wrap)]
fn rice_code(lsp_size: u8, sample: u32, bit_sink: &mut impl BitSink) {
    let shift = i32::BITS - u32::from(lsp_size);
    let lsp = (sample << shift) >> shift;
    let msp = sample >> lsp_size;
    unary_code(msp, bit_sink);
    write_sample(lsp as i32, bit_sink, lsp_size, 0);
}

fn residual_codeing(params: &RiceParams, bit_sink: &mut impl BitSink, sample: i32) {
    match params {
        RiceParams::Param(rice) => rice_code(*rice, signed_fold(sample), bit_sink),
        RiceParams::Escape(num_bits) => write_sample(sample, bit_sink, *num_bits, 0),
    }
}
