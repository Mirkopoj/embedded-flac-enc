struct MetaDataBlockHeader {
    is_last: bool,
    block_type: MetaDataBlockType,
}

enum MetaDataBlockType {
    StreamInfo = 0,
    Padding = 1,
    Application = 2,
    SeekTable = 3,
    VorbisComent = 4,
    CueSheet = 5,
    Picture = 6,
}

trait MetaDataBlock {
    type Array;
    fn to_bytes(&self) -> Self::Array;
}

mod stream_info;
mod padding;
#[cfg(feature="application")]
mod application;
