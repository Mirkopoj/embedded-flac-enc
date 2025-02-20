use crate::ByteSink;

pub struct MetaDataBlockHeader<const N: usize> {
    is_last: bool,
    block_type: MetaDataBlockType<N>,
}

impl<const N: usize> MetaDataBlockHeader<N> {
    pub fn new(is_last: bool, block_type: MetaDataBlockType<N>) -> Self {
        Self {
            is_last,
            block_type,
        }
    }

    pub fn write<BS: ByteSink>(&self, sink: &mut BS) {
        let last_flag = if self.is_last {
            0b1000_0000
        } else {
            0b0000_0000
        };
        sink.write(last_flag | self.block_type.as_byte());
        self.block_type.write(sink);
    }
}

#[repr(u8)]
pub enum MetaDataBlockType<const N: usize> {
    StreamInfo(stream_info::StreamInfo) = 0,
    Padding(padding::Padding<N>) = 1,
    Application(application::Application<N>) = 2,
    SeekTable = 3,
    VorbisComent = 4,
    CueSheet = 5,
    Picture = 6,
}

impl<const N: usize> MetaDataBlockType<N> {
    fn as_byte(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
    fn write<BS: ByteSink>(&self, sink: &mut BS) {
        match self {
            MetaDataBlockType::StreamInfo(stream_info) => stream_info.write(sink),
            MetaDataBlockType::Padding(padding) => padding.write(sink),
            MetaDataBlockType::Application(application) => application.write(sink),
            MetaDataBlockType::SeekTable => todo!(),
            MetaDataBlockType::VorbisComent => todo!(),
            MetaDataBlockType::CueSheet => todo!(),
            MetaDataBlockType::Picture => todo!(),
        }
    }
}

mod application;
mod padding;
mod seek_table;
pub mod stream_info;
