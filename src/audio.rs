use prelude::*;

pub type Sample = f64;

pub fn init() -> (cpal::EventLoop, Context) {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device
        .default_output_format()
        .expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    (
        event_loop,
        Context::new(format.channels as usize, format.sample_rate.0 as usize),
    )
}

pub fn run(event_loop: &cpal::EventLoop, graph: AudioGraph) {
    event_loop.run({
        let mut graph = graph;
        move |_, data| match data {
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(graph.ctx.channels) {
                    let output = graph.sample();
                    for i in 0..sample.len() {
                        sample[i] = ((output[i] * 0.5 + 0.5) * f64::from(std::u16::MAX)) as u16;
                    }
                }
            }
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(graph.ctx.channels) {
                    let output = graph.sample();
                    for i in 0..sample.len() {
                        sample[i] = (output[i] * f64::from(std::i16::MAX)) as i16;
                    }
                }
            }
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for sample in buffer.chunks_mut(graph.ctx.channels) {
                    let output = graph.sample();
                    for i in 0..sample.len() {
                        sample[i] = output[i] as f32;
                    }
                }
            }
            _ => (),
        }
    });
}
