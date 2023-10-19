pub use crate::noise::Noise;
    
const GENERATORS: usize = 15;

pub struct Pink{
    noise: [Noise; GENERATORS], // updated based on trailing zeros
    white: Noise, // Updated every iteration
    pink: f32, // Actual noise
    
    counter: u32,

    generators: u32,
    rollover: u32,
}

impl Pink{
    const GENERATORS: u32 = GENERATORS as u32;
    
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

    fn get_noise_index(&self) -> u32{
        assert!(self.counter > 0);
        assert!(self.counter <= self.rollover);
        
        self.counter.trailing_zeros()
    }

    fn increment_counter(&mut self){
        assert!(self.counter > 0);
        assert!(self.counter <= self.rollover);
        
        self.counter = self.counter & (self.rollover - 1); 
        self.counter = self.counter + 1;
    }

    /* Generates a new sample using the Voss-McCartney algorithm
     * https://www.firstpr.com.au/dsp/pink-noise/
     */
    pub fn update(&mut self) -> f32{

        let index = self.get_noise_index() as usize;
        assert!( index < self.generators as usize );

        self.white.update();
        self.noise[index].update();

        self.pink = self.pink - self.white.previous();
        self.pink = self.pink + self.white.value();
        
        self.pink = self.pink - self.noise[index].previous();
        self.pink = self.pink + self.noise[index].value();

        self.increment_counter();

        self.pink / (self.generators as f32 + 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;    
    
    #[test]
    fn initialisation() {
        let p = Pink::new();
        
        assert_eq!(p.counter, 1);
        assert_eq!(p.generators, Pink::GENERATORS);
        assert_eq!(p.pink, 0.0);
        assert_eq!(p.rollover,16384);
        assert_eq!(p.white.value(), 0.0);
        assert_eq!(p.white.previous(), 0.0);

        for i in 0..Pink::GENERATORS{
            assert_eq!(p.noise[i as usize].value(), 0.0);
            assert_eq!(p.noise[i as usize].previous(), 0.0);
        }

    }
    
    #[test]
    fn update() {
        let mut p = Pink::new();
        assert_eq!(p.counter, 1);
        assert_eq!(p.generators, Pink::GENERATORS);
        assert_eq!(p.pink, 0.0);
        assert_eq!(p.rollover,16384);
        assert_eq!(p.white.value(), 0.0);
        assert_eq!(p.white.previous(), 0.0);
       
        p.update();
        assert_ne!(p.pink, 0.0);
        assert_ne!(p.white.value(), 0.0);
        assert_eq!(p.white.previous(), 0.0);
        assert_ne!(p.noise[0].value(), 0.0);
        assert_eq!(p.noise[0].previous(), 0.0);
    }

    #[test]
    fn index_distribution() {
        let mut p = Pink::new();
        let mut count: [u32; Pink::GENERATORS as usize] = [0; Pink::GENERATORS as usize];

        for _i in 0..p.rollover{
            let index = p.get_noise_index();
            count[index as usize] = count[index as usize] + 1;
            p.update();
        }
        
        for i in 0..Pink::GENERATORS - 1{
            assert_eq!(count[i as usize], p.rollover >> (i + 1));
        }
        
        assert_eq!(count[(Pink::GENERATORS - 1) as usize], 1);
    }
    
    #[test]
    fn increment_counter() {
        let mut p = Pink::new();
        assert_eq!(p.counter, 1);

        p.increment_counter();
        assert_eq!(p.counter, 2);
        
        p.increment_counter();
        assert_eq!(p.counter, 3);
    }
    
    #[test]
    fn increment_counter_rollover() {
        let mut p = Pink::new();
        assert_eq!(p.counter, 1);

        p.counter = p.rollover - 1;
        p.increment_counter();
        assert_eq!(p.counter, p.rollover);
        
        p.increment_counter();
        assert_eq!(p.counter, 1);
    }

    #[test]
    fn trailing_zeros() {
        let mut p = Pink::new();
       
        assert!(p.generators == 15);
        assert!(p.counter == 1);
        assert!(p.get_noise_index() == 0);

        p.counter = 0b1u32;
        assert!(p.get_noise_index() == 0);
        
        p.counter =0b10u32;
        assert!(p.get_noise_index() == 1);

        p.counter =0b100u32;
        assert!(p.get_noise_index() == 2);
        
        p.counter =0b1000u32;
        assert!(p.get_noise_index() == 3);

        p.counter =0b10000u32;
        assert!(p.get_noise_index() == 4);

        p.counter =0b100000u32;
        assert!(p.get_noise_index() == 5);

        p.counter =0b1000000u32;
        assert!(p.get_noise_index() == 6);

        p.counter =0b10000000u32;
        assert!(p.get_noise_index() == 7);

        p.counter =0b100000000u32;
        assert!(p.get_noise_index() == 8);

        p.counter =0b1000000000u32;
        assert!(p.get_noise_index() == 9);

        p.counter =0b10000000000u32;
        assert!(p.get_noise_index() == 10);

        p.counter =0b100000000000u32;
        assert!(p.get_noise_index() == 11);

        p.counter = 0b1000000000000u32;
        assert!(p.get_noise_index() == 12);

        p.counter = 0b10000000000000u32;
        assert!(p.get_noise_index() == 13);

        p.counter = 0b100000000000000u32;
        assert!(p.get_noise_index() == 14);
    }
}

