pub mod default_sample_handler;
use crate::audio::Sample;

pub trait SampleHandler {
    fn recv_sample(&mut self, sample: Sample);
}
