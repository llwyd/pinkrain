pub use crate::noise::Noise;

pub struct Pink{
    noise: [Noise; 15], // updated based on trailing zeros
    white: Noise, // Updated every iteration
    pink: f32, // Actual noise
    
    counter: u32,

    generators: u32,
    rollover: u32,
}

impl Pink{
    const GENERATORS: u32 = 15;
    
    pub fn new() -> Pink{
        Pink{
            noise:[Noise::new(); Self::GENERATORS as usize],
            white: Noise::new(),
            pink: 0.0,
            counter: 1,
            generators: Self::GENERATORS,
            rollover: 2u32.pow(Self::GENERATORS - 1),
        }
    }

    #[allow(dead_code)] 
    fn set_counter(&mut self, new_counter:u32){
        assert!(new_counter <= self.rollover);
        self.counter = new_counter;
    }
    

    fn get_noise_index(&self) -> u32{
        self.counter.trailing_zeros()
    }
    /* Generates a new sample using the Voss-McCartney algorithm
     * https://www.firstpr.com.au/dsp/pink-noise/
     */
    pub fn update(&mut self) -> f32{

        let index = self.get_noise_index();

        self.white.update();
        self.noise[index as usize].update();

        self.pink = self.pink - self.white.previous();
        self.pink = self.pink + self.white.value();
        
        self.pink = self.pink - self.noise[index as usize].previous();
        self.pink = self.pink + self.noise[index as usize].value();


        self.counter = self.counter & (self.rollover - 1); 
        self.counter = self.counter + 1;

        self.pink / (self.generators as f32 + 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;    
    
    #[test]
    fn trailing_zeros() {
        let mut p = Pink::new();
       
        assert!(p.generators == 15);
        assert!(p.counter == 1);
        assert!(p.get_noise_index() == 0);

        p.set_counter(0b1u32);
        assert!(p.get_noise_index() == 0);
        
        p.set_counter(0b10u32);
        assert!(p.get_noise_index() == 1);

        p.set_counter(0b100u32);
        assert!(p.get_noise_index() == 2);
        
        p.set_counter(0b1000u32);
        assert!(p.get_noise_index() == 3);

        p.set_counter(0b10000u32);
        assert!(p.get_noise_index() == 4);

        p.set_counter(0b100000u32);
        assert!(p.get_noise_index() == 5);

        p.set_counter(0b1000000u32);
        assert!(p.get_noise_index() == 6);

        p.set_counter(0b10000000u32);
        assert!(p.get_noise_index() == 7);

        p.set_counter(0b100000000u32);
        assert!(p.get_noise_index() == 8);

        p.set_counter(0b1000000000u32);
        assert!(p.get_noise_index() == 9);

        p.set_counter(0b10000000000u32);
        assert!(p.get_noise_index() == 10);

        p.set_counter(0b100000000000u32);
        assert!(p.get_noise_index() == 11);

        p.set_counter(0b1000000000000u32);
        assert!(p.get_noise_index() == 12);

        p.set_counter(0b10000000000000u32);
        assert!(p.get_noise_index() == 13);

        p.set_counter(0b100000000000000u32);
        assert!(p.get_noise_index() == 14);
    }
}

