use prelude::*;

impl Context {
    pub fn constant(&self, x: Sample) -> Node {
        Constant::new(self, x)
    }

    pub fn phasor(&self) -> Node {
        Phasor::new(self)
    }

    pub fn fn1(&self, f: fn(Sample) -> Sample) -> Node {
        Fn1::new(self, f)
    }

    pub fn fn2(&self, f: fn(Sample, Sample) -> Sample) -> Node {
        Fn2::new(self, f)
    }

    pub fn fn3(&self, f: fn(Sample, Sample, Sample) -> Sample) -> Node {
        Fn3::new(self, f)
    }

    pub fn add(&self) -> Node {
        self.fn2(add)
    }

    pub fn mul(&self) -> Node {
        self.fn2(mul)
    }

    pub fn sub(&self) -> Node {
        self.fn2(sub)
    }

    pub fn div(&self) -> Node {
        self.fn2(div)
    }

    pub fn sine(&self) -> Node {
        self.fn1(sine)
    }

    pub fn cosine(&self) -> Node {
        self.fn1(cosine)
    }

    pub fn tri(&self) -> Node {
        self.fn1(triangle)
    }

    pub fn pulse(&self) -> Node {
        self.fn2(rectangle)
    }

    pub fn range(&self) -> Node {
        self.fn3(range)
    }

    pub fn unit(&self) -> Node {
        self.fn1(unit)
    }
}
