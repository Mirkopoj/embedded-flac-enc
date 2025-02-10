use super::MetaDataBlock;

pub struct Padding<const N: usize>;

impl<const N: usize> MetaDataBlock for Padding<N> {
    type Array = [u8; N];
    fn to_bytes(&self) -> Self::Array {
        [0; N]
    }
}
