use board::stones::stone::Stone;

struct GoSample {
    x: Vec<Stone>,
    y: Vec<f32>,
}

impl GoSample {
    pub fn new(x: Vec<Stone>, y: Vec<f32>) -> Self {
        GoSample {
            x,
            y
        }
    }
}


impl GoSample {}