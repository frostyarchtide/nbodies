use crate::math::*;
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{self, Color},
    terminal,
};
use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

pub struct Screen {
    resolution: UVec2,
    pixels: Vec<Color>,
}

impl Drop for Screen {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        stdout.execute(cursor::Show).unwrap();
        stdout.execute(terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

impl Screen {
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        stdout.execute(terminal::EnterAlternateScreen)?;
        stdout.execute(cursor::Hide)?;
        stdout.execute(terminal::Clear(terminal::ClearType::Purge))?;

        let size = terminal::size().unwrap();
        let mut pixels = Vec::with_capacity(size.0 as usize * size.1 as usize * 2);

        for _ in 0..(size.0 * size.1 * 2) {
            pixels.push(Color::Black);
        }

        Ok(Self {
            resolution: UVec2 {
                x: size.0 as usize,
                y: size.1 as usize * 2,
            },
            pixels,
        })
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = Color::Black;
        }
    }

    pub fn draw(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        for y in 0..(self.resolution.y / 2) {
            for x in 0..self.resolution.x {
                stdout
                    .queue(cursor::MoveTo(x as u16, y as u16))?
                    .queue(style::SetForegroundColor(
                        self.get_pixel(IVec2::from(UVec2 { x, y: y * 2 })).unwrap(),
                    ))?
                    .queue(style::SetBackgroundColor(
                        self.get_pixel(IVec2::from(UVec2 { x, y: y * 2 + 1 }))
                            .unwrap(),
                    ))?
                    .queue(style::Print('\u{2580}'))?;
            }
        }

        stdout.flush()?;
        sleep(Duration::from_secs_f32(1. / 60.));

        Ok(())
    }

    pub fn get_resolution(&self) -> UVec2 {
        self.resolution
    }

    pub fn set_pixel(&mut self, position: IVec2, color: Color) {
        if position.x < 0
            || position.y < 0
            || position.x >= self.resolution.x as isize
            || position.y >= self.resolution.y as isize
        {
            return;
        }

        let position = UVec2::from(position);

        self.pixels[position.y * self.resolution.x + position.x] = color;
    }

    pub fn get_pixel(&self, position: IVec2) -> Option<Color> {
        if position.x < 0
            || position.y < 0
            || position.x >= self.resolution.x as isize
            || position.y >= self.resolution.y as isize
        {
            return None;
        }

        let position = UVec2::from(position);

        Some(self.pixels[position.y * self.resolution.x + position.x])
    }
}
