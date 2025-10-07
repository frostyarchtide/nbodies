use std::ops::*;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Vec2 {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance_squared(&self, rhs: Self) -> f64 {
        let delta = *self - rhs;
        delta.x * delta.x + delta.y * delta.y
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    pub fn direction_to(&self, rhs: Self) -> Self {
        (rhs - *self).normalized()
    }
}

#[derive(Clone, Copy)]
pub struct IVec2 {
    pub x: isize,
    pub y: isize,
}

impl From<UVec2> for IVec2 {
    fn from(value: UVec2) -> Self {
        Self {
            x: value.x.try_into().unwrap(),
            y: value.y.try_into().unwrap(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct UVec2 {
    pub x: usize,
    pub y: usize,
}

impl From<IVec2> for UVec2 {
    fn from(value: IVec2) -> Self {
        Self {
            x: value.x as usize,
            y: value.y as usize,
        }
    }
}

impl Div<usize> for UVec2 {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
