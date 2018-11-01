pub struct Context {
    pub channels: usize,
    pub sample_rate: usize,
    pub sample_number: u64,
}

impl Context {
    pub fn new(channels: usize, sample_rate: usize) -> Self {
        Context {
            channels,
            sample_rate,
            sample_number: 0,
        }
    }
}
