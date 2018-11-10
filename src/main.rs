//! # Sound Garden
//!
//! Sound Garden is a virtual modular audio synthesis environment.

extern crate audio_graph;
extern crate cpal;
extern crate find_folder;
extern crate hashbrown;
extern crate parking_lot;
#[macro_use]
extern crate sciter;

mod audio;
mod prelude;
mod ui;

/// Right now code in the `main` is just a playground for testing API and UI design.
/// With time it should:
/// * Process command-line arguments to configure SG instance
///    (list/select audio backend/device, on/off input/OSC, OSC ports, path to record session etc.)
/// * Initialize OSC and run background OSC server thread
/// * Initialize audio and run background audio event loop thread
/// * Initialize UI and run its event loop
fn main() {
    let (format, graph) = audio::init();

    ui::main(format, graph);
}
