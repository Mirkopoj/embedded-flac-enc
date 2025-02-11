use crate::ByteSink;

pub struct Padding<const N: usize>;

impl<const N: usize> Padding<N> {
    pub fn write<BS: ByteSink>(&self, sink: &mut BS) {
        for _ in 0..N {
            sink.write(0);
        }
    }
}
