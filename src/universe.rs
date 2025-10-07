use crate::math::*;

pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
}

pub struct Universe {
    pub bodies: Vec<Body>,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            bodies: Default::default(),
        }
    }
    
    pub fn update(&mut self) {
        for body in self.bodies.iter_mut() {
            body.position += body.velocity;
        }
    }
}
