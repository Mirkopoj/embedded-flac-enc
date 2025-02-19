use footer::FrameFooter;
use header::FrameHeader;
use sub_frame::SubFrame;

pub struct Frame<const CHANNELS: usize, const BLOCK_SIZE: usize> {
    header: FrameHeader,
    subframes: [SubFrame<BLOCK_SIZE>; CHANNELS],
    footer: FrameFooter,
}

mod footer;
mod header;
mod sub_frame;
