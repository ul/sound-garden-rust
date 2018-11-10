//! # UI module
//!

use audio_graph::prelude::*;
use parking_lot::Mutex;
use sciter::{Element, EventHandler};
use std::sync::Arc;

#[macro_use]
mod macros;

struct Handler {
    audio: Arc<Mutex<AudioGraph>>,
    format: cpal::Format,
}

fn report_error(root: &Element, msg: &str) {
    if root
        .call_function("Error.report", &make_args!(msg))
        .is_err()
    {
        println!("Failed to call Error.report");
    };
}

impl Handler {
    fn graph_text_change(&mut self, root: &Element, text: String) {
        let channels = self.format.channels as usize;
        let sample_rate = self.format.sample_rate.0 as usize;
        let mut nodes = Vec::new();
        let mut g = AudioGraph::new(channels);
        for (i, token) in text.split_whitespace().enumerate() {
            let node: Node = match token {
                "s" | "sine" => Box::new(Osc::new(channels, sample_rate, sine)),
                "t" | "tri" => Box::new(Osc::new(channels, sample_rate, triangle)),
                "w" | "saw" => Box::new(Phasor::new(channels, sample_rate)),
                "+" => Box::new(Fn2::new(channels, add)),
                "-" => Box::new(Fn2::new(channels, sub)),
                "*" => Box::new(Fn2::new(channels, mul)),
                "/" => Box::new(Fn2::new(channels, div)),
                _ => match token.parse::<Sample>() {
                    Ok(x) => Box::new(Constant::new(channels, x)),
                    Err(_) => {
                        report_error(
                            root,
                            &format!("Node #{} `{}` is unknown module.", i + 1, token),
                        );
                        return;
                    }
                },
            };
            nodes.push((g.add_node(node), token));
        }
        let mut stack = Vec::new();
        for (i, (idx, token)) in nodes.into_iter().enumerate() {
            let inputs = g.node(idx).inputs();
            if stack.len() < (inputs as usize) {
                report_error(
                    root,
                    &format!(
                        "Node #{} `{}` has not enough inputs on the stack.",
                        i + 1,
                        token
                    ),
                );
                return;
            }
            let mut sources = Vec::new();
            for _ in 0..inputs {
                sources.push(stack.pop().unwrap());
            }
            g.set_sources_rev(idx, &sources);
            stack.push(idx)
        }
        report_error(root, "");
        {
            *self.audio.lock() = g
        }
    }
}

impl EventHandler for Handler {
    dispatch_script_call! {
        fn graph_text_change(String);
    }
}

pub fn main(format: cpal::Format, audio: Arc<Mutex<AudioGraph>>) {
    let mut frame = sciter::Window::new();
    let handler = Handler { audio, format };
    frame.event_handler(handler);

    let resources = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("resources")
        .unwrap();
    frame.load_file(resources.join("main.htm").to_str().unwrap());

    // let root = Element::from_window(frame);
    frame.run_app();
}
