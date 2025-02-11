use footer::FrameFooter;
use header::FrameHeader;
use sub_frame::SubFrame;

pub struct Frame<const CHANNELS: usize> {
    header: FrameHeader,
    subframes: [SubFrame; CHANNELS],
    footer: FrameFooter,
}

mod header;
mod sub_frame;
mod footer;
