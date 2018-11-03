//! # Audio context

/// Represents audio context and is passed as mutable to Module's `sample`.
///
/// The first purpose of this structure is to conveys basic information about static parameters of
/// audio environment, such as number of channels and sample rate; as well as dynamic such as logic
/// time expressed as a frame number.
///
/// The second purpose is to provide efficient non-local communication between Modules, that's why
/// it is passed to `sample` as mutable. This functionality is not used in audio_graph yet.
pub struct Context {
    channels: usize,
    sample_rate: usize,
    frame_number: usize,
}

impl Context {
    pub fn new(channels: usize, sample_rate: usize) -> Self {
        Context {
            channels,
            sample_rate,
            frame_number: 0,
        }
    }

    /// Get device output channels count.
    pub fn channels(&self) -> usize {
        self.channels
    }

    /// Get device sample rate.
    pub fn sample_rate(&self) -> usize {
        self.sample_rate
    }

    /// Get logical time expressed as a number of frames from the start.
    pub fn frame_number(&self) -> usize {
        self.frame_number
    }

    /// Advance logical time by one frame.
    pub fn tick(&mut self) {
        self.frame_number += 1;
    }
}
