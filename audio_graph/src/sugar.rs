//! # AudioGraph building sugar
//!
//! This module implements shorthand methods which allow to instantiate specific Module
//! implementation, add it to the AudioGraph instance and connect it to the sources on one go.
use prelude::*;

impl AudioGraph {
    pub fn constant(&mut self, x: Sample) -> NodeIndex {
        let node = Constant::new(&self.ctx, x);
        self.add_node(node)
    }

    pub fn phasor(&mut self, frequency: NodeIndex) -> NodeIndex {
        let node = Phasor::new(&self.ctx);
        let node_idx = self.add_node(node);
        self.connect(frequency, node_idx);
        node_idx
    }

    pub fn add(&mut self, a: NodeIndex, b: NodeIndex) -> NodeIndex {
        let c = self.fn2(add);
        self.set_sources(c, &[a, b]);
        c
    }

    pub fn mul(&mut self, a: NodeIndex, b: NodeIndex) -> NodeIndex {
        let c = self.fn2(mul);
        self.set_sources(c, &[a, b]);
        c
    }

    pub fn sub(&mut self, a: NodeIndex, b: NodeIndex) -> NodeIndex {
        let c = self.fn2(sub);
        self.set_sources(c, &[a, b]);
        c
    }

    pub fn div(&mut self, a: NodeIndex, b: NodeIndex) -> NodeIndex {
        let c = self.fn2(div);
        self.set_sources(c, &[a, b]);
        c
    }

    pub fn sine(&mut self, frequency: NodeIndex) -> NodeIndex {
        let node = Sine::new(&self.ctx);
        let node_idx = self.add_node(node);
        self.connect(frequency, node_idx);
        node_idx
    }

    pub fn cosine(&mut self, frequency: NodeIndex) -> NodeIndex {
        self.osc(cosine, frequency)
    }

    pub fn tri(&mut self, frequency: NodeIndex) -> NodeIndex {
        self.osc(triangle, frequency)
    }

    pub fn pulse(&mut self, frequency: NodeIndex, width: NodeIndex) -> NodeIndex {
        let p = self.fn2(rectangle);
        self.set_sources(p, &[frequency, width]);
        p
    }

    pub fn range(&mut self, x: NodeIndex, a: NodeIndex, b: NodeIndex) -> NodeIndex {
        let out = self.fn3(range);
        self.set_sources(out, &[x, a, b]);
        out
    }

    pub fn unit(&mut self, x: NodeIndex) -> NodeIndex {
        let y = self.fn1(unit);
        self.connect(x, y);
        y
    }

    fn osc(&mut self, f: fn(Sample) -> Sample, frequency: NodeIndex) -> NodeIndex {
        let phasor = self.phasor(frequency);
        let osc = self.fn1(f);
        self.connect(phasor, osc);
        osc
    }

    fn fn1(&mut self, f: fn(Sample) -> Sample) -> NodeIndex {
        let node = Fn1::new(&self.ctx, f);
        self.add_node(node)
    }

    fn fn2(&mut self, f: fn(Sample, Sample) -> Sample) -> NodeIndex {
        let node = Fn2::new(&self.ctx, f);
        self.add_node(node)
    }

    fn fn3(&mut self, f: fn(Sample, Sample, Sample) -> Sample) -> NodeIndex {
        let node = Fn3::new(&self.ctx, f);
        self.add_node(node)
    }
}
