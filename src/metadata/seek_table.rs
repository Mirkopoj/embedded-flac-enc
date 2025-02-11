use crate::ByteSink;

pub struct SeekPoint {
    sample_num_of_first_in_target: u64,
    offset_to_target_frame: u64,
    sample_count_in_target_frame: u16,
}

impl SeekPoint {
    pub fn write<BS: ByteSink>(&self, sink: &mut BS) {
        self.sample_num_of_first_in_target
            .to_be_bytes()
            .iter()
            .chain(self.offset_to_target_frame.to_be_bytes().iter())
            .chain(self.sample_count_in_target_frame.to_be_bytes().iter())
            .for_each(|&byte| sink.write(byte));
    }
}
