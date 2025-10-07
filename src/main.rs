mod math;
mod screen;
mod universe;

use crossterm::{event::{poll, Event, read, KeyCode}, style::Color};
use math::*;
use rand::Rng;
use screen::*;
use std::{io, thread::sleep, time::Duration};
use universe::*;

pub const TICK_TIME: f64 = 1. / 60.;

fn main() -> io::Result<()> {
    let mut screen = Screen::new()?;
    let mut universe = Universe::new();

    let center = screen.get_resolution() / 2;
    let center = Vec2 {
        x: center.x as f64,
        y: center.y as f64,
    };

    let mut rng = rand::rng();

    for _ in 0..20 {
        let pos_x = rng.random_range(-50.0..50.0);
        let pos_y = rng.random_range(-50.0..50.0);

        universe.bodies.push(Body {
            position: Vec2 {
                x: center.x - pos_x,
                y: center.y - pos_y,
            },
            velocity: Vec2 {
                x: 0.,
                y: 0.,
            },
        });
    }

    let mut should_exit = false;

    while !should_exit {
        if poll(Duration::ZERO)? {
            match read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') {
                        should_exit = true;
                    }
                },
                _ => (),
            }
        }

        screen.clear();
        universe.update();

        for body in universe.bodies.iter() {
            let position = IVec2 {
                x: body.position.x as isize,
                y: body.position.y as isize,
            };
            screen.set_pixel(position, Color::Red);
        }

        screen.draw()?;

        sleep(Duration::from_secs_f64(TICK_TIME));
    }

    Ok(())
}
