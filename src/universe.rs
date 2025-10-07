use crate::{TICK_TIME, math::*};

const G: f64 = 1_000.;
const SMOOTHING: f64 = 1.;

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
        for i in 0..self.bodies.len() {
            for j in i..self.bodies.len() {
                let distance_squared = self.bodies[i]
                    .position
                    .distance_squared(self.bodies[j].position);
                if distance_squared > 0.0 {
                    let direction = self.bodies[i]
                        .position
                        .direction_to(self.bodies[j].position);
                    let force = direction * G / (distance_squared + SMOOTHING) * TICK_TIME;

                    self.bodies[i].velocity += force;
                    self.bodies[j].velocity -= force;
                }
            }
        }

        for body in self.bodies.iter_mut() {
            body.position += body.velocity * TICK_TIME;
        }
    }
}
