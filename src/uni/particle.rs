#[derive(Debug)]
pub struct Particle<T, S>{
    pub x: T,
    pub y: S
}
impl<T: std::fmt::Debug, S: std::fmt::Debug> Particle<T, S>{
    pub fn new(x: T, y: S) -> Particle<T, S>{
        Particle{x, y}
    }
    pub fn display(&self){
        println!("x: {:?}, y: {:?}", &self.x, &self.y)
    }
}
