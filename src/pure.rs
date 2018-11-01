use prelude::*;

pub fn sin(x: Sample) -> Sample {
    x.sin()
}

pub fn sine(x: Sample) -> Sample {
    sin(2.0 * std::f64::consts::PI * x)
}

pub fn add(x: Sample, y: Sample) -> Sample {
    x + y
}

pub fn mul(x: Sample, y: Sample) -> Sample {
    x * y
}
