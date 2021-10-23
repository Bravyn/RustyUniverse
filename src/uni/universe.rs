use crate::Particle;

#[derive(Debug)]
pub struct Universe<T, S>{
    pub particles: Vec<Particle<T, S>>
}
impl<T: std::fmt::Debug, S: std::fmt::Debug> Universe<T, S> {
    pub fn new(particles: Vec<Particle<T, S>>)-> Universe<T, S> {
        Universe{particles}
    }
    pub fn display(&self){
        println!("{:?} ", &self.particles)
    }
}