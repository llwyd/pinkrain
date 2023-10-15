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

#[cfg(test)]
mod tests {
    use super::*;    
    
    #[test]
    fn init() {
        let n = Noise::new();

        assert_eq!(n.previous(), 0.0);
        assert_eq!(n.value(), 0.0);
    }
    
    #[test]
    fn update() {
        let mut n = Noise::new();

        assert_eq!(n.previous(), 0.0);
        assert_eq!(n.value(), 0.0);

        n.update();
        
        assert_eq!(n.previous(), 0.0);
        assert_ne!(n.value(), 0.0);
    }
    
    #[test]
    fn update_twice() {
        let mut n = Noise::new();

        assert_eq!(n.previous(), 0.0);
        assert_eq!(n.value(), 0.0);

        n.update();
        
        assert_eq!(n.previous(), 0.0);
        assert_ne!(n.value(), 0.0);

        let prev = n.value();
        n.update();
        assert_eq!(n.previous(), prev);
        assert_ne!(n.value(), 0.0);
    }
}
