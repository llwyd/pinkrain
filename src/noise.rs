use rand::random;

#[derive(Copy, Clone)]
pub struct Noise{
    previous:f32,
    value:f32,
}

impl Noise{
    pub fn new() -> Noise{
        Noise{
            previous: 0.0,
            value: 0.0,
        }
    }
    pub fn update(&mut self){
        self.previous = self.value;
        self.value = (random::<f32>() * 2.0) - 1.0;
    }

    pub fn previous(&self) -> f32{
        self.previous
    }
    
    pub fn value(&self) -> f32{
        self.value
    }
}
