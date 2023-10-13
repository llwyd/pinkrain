use rand::random;

#[derive(Copy, Clone)]
pub struct Noise{
    previous:f32,
    value:f32,
}

pub struct Pink{
    noise: [Noise; 15], // updated based on trailing zeros
    white: Noise, // Updated every iteration
    pink: f32, // Actual noise
    
    pink_norm: f32,
    counter: u32,

    generators: u32,
    rollover: u32,
}

impl Pink{
    fn update_noise(noise:&mut Noise){
        noise.previous = noise.value;
        noise.value = (random::<f32>() * 2.0) - 1.0;
    }
    pub fn new() -> Pink{
        let mut pink = Pink{
            noise:[Noise{previous:0.0, value:0.0}; 15],
            white: Noise{previous:0.0, value:0.0},
            pink: 0.0,
            pink_norm: 0.0,
            counter: 1,
            generators: 15,
            rollover: 0,
        };
        
        Self::update_noise(&mut pink.white);
        
        for i in 0..pink.generators{
            Self::update_noise(&mut pink.noise[i as usize]);
            pink.pink += pink.noise[i as usize].value;
        }
        pink.pink += pink.white.value;
        
        pink.rollover = 2u32.pow(pink.generators - 1);

        pink
    }

    /* Generates a new sample using the Voss-McCartney algorithm
     * https://www.firstpr.com.au/dsp/pink-noise/
     */
    pub fn update(&mut self) -> f32{

        let index = self.counter.trailing_zeros();

        Self::update_noise(&mut self.white);
        Self::update_noise(&mut self.noise[index as usize]);

        self.pink = self.pink - self.white.previous;
        self.pink = self.pink + self.white.value;
        
        self.pink = self.pink - self.noise[index as usize].previous;
        self.pink = self.pink + self.noise[index as usize].value;

        self.pink_norm = self.pink / (self.generators as f32 + 1.0);

        self.counter = self.counter & (self.rollover - 1); 
        self.counter = self.counter + 1;

        self.pink_norm
    }
}

