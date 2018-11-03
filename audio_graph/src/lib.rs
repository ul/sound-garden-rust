//! # audio_graph
//!
//! audio_graph is a library which allows creating and sampling a network of interconnected
//! audio signal modules. BYO audio driver or audio file encoder to play or record generated sound.
extern crate fixedbitset;
extern crate petgraph;

pub mod context;
pub mod graph;
pub mod module;
pub mod modules;
pub mod prelude;
pub mod pure;
pub mod sample;
pub mod sugar;
