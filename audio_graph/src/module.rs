//! # Module
use prelude::*;

/// Defines behavior of sound-producing node.
pub trait Module {
    /// How many sources must be connected to this module.
    fn inputs(&self) -> u8;

    /// Get Module's current frame.
    ///
    /// Must return the same value if no `sample` was called for Modules in-between.
    fn output(&self) -> &Frame;

    /// Compute the next frame.
    ///
    /// `input` is a flattened slice of source Modules' outputs.
    ///
    /// This is example of how multi-channel multi-source outputs are collected into the
    /// input buffer (stereo audio and 3 incoming connections case):
    /// ```
    /// first source output: [ 0 1 ]
    ///                        | |
    ///                        | +--------+
    ///                        +--------+ |
    ///                                 | |
    /// second source output: [ 0 1 ]   | |   [ 0 1 ]: third source output
    ///                         | |     | |     | |
    ///                         | +-----------+ | |
    ///                         +-----------+ | | |
    ///                                 | | | | | |
    ///                                 V V V V V V
    /// sink input:                   [ 0 1 2 3 4 5 ]
    /// ```
    fn sample(&mut self, ctx: &mut Context, input: &[Sample]);
}
