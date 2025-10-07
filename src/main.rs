mod math;
mod screen;
mod universe;

use crossterm::style::Color;
use screen::*;
use math::*;
use universe::*;
use std::{io, thread::sleep, time::Duration};

fn main() -> io::Result<()> {
    let mut screen = Screen::new()?;
    let mut universe = Universe::new();

    universe.bodies.push(Body { position: Vec2 { x: 5., y: 5. }, velocity: Vec2 { x: 3., y: 0. } });

    let should_exit = false;
    while !should_exit {
        screen.clear();
        universe.update();

        for body in universe.bodies.iter() {
            let position = UVec2 { x: body.position.x as usize, y: body.position.y as usize };
            screen.set_pixel(position, Color::Red);
        }

        screen.draw()?;

        sleep(Duration::from_secs_f32(1. / 30.));
    }

    Ok(())
}
