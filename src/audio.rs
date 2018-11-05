//! # Audio module
//!
//! This module is responsible for turning AudioGraph into actual sound. It uses cpal to open audio
//! device and then to run event loop with a callback to sample AudioGraph.
use parking_lot::Mutex;
use prelude::*;
use std::sync::Arc;
use std::thread;

/// Initalize audio subsystem
///
/// It creates empty AudioGraph and starts to sample it in the background thread.
/// AudioGraph is returned wrapped into Arc/Mutex to allow it's modification from the UI thread.
/// Background audio thread handle is not preserved because cpal doesn't expose a way to stop its
/// EventLoop (https://github.com/tomaka/cpal/issues/245), thus no point in joining such thread.
/// Right now `init` opens default output device with default parameters, but it should be made
/// configurable.
/// Also, though it's no point in running SG if audio initialization fails this function should
/// still return Result instead of panic.
pub fn init() -> Arc<Mutex<AudioGraph>> {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device
        .default_output_format()
        .expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());
    let channels = format.channels as usize;
    let ctx = Context::new(channels, format.sample_rate.0 as usize);
    let graph = Arc::new(Mutex::new(AudioGraph::new(ctx)));

    thread::spawn({
        let graph = graph.clone();
        move || {
            event_loop.run({
                move |_, data| {
                    let mut graph = graph.lock();
                    match data {
                        cpal::StreamData::Output {
                            buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
                        } => {
                            for sample in buffer.chunks_mut(channels) {
                                let output = graph.sample();
                                for i in 0..sample.len() {
                                    sample[i] =
                                        ((output[i] * 0.5 + 0.5) * f64::from(std::u16::MAX)) as u16;
                                }
                            }
                        }
                        cpal::StreamData::Output {
                            buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
                        } => {
                            for sample in buffer.chunks_mut(channels) {
                                let output = graph.sample();
                                for i in 0..sample.len() {
                                    sample[i] = (output[i] * f64::from(std::i16::MAX)) as i16;
                                }
                            }
                        }
                        cpal::StreamData::Output {
                            buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                        } => {
                            for sample in buffer.chunks_mut(channels) {
                                let output = graph.sample();
                                for i in 0..sample.len() {
                                    sample[i] = output[i] as f32;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            })
        }
    });

    graph
}
