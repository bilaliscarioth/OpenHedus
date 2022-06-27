use std::ops::*;


#[cfg(target_arch = "x86_64" )]
#[derive(Debug, Clone, Copy)]
pub struct Vector2f{
    pub x : f64,
    pub y : f64,
}

#[cfg(target_arch = "x86")]
#[derive(Debug, Clone, Copy)]
pub struct Vector2f{
    pub x : f32,
    pub y : f32,
}

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy)]
pub struct Vector2u {
    pub x: u64,
    pub y: u64
}

#[cfg(target_arch = "x86")]
#[derive(Debug, Clone, Copy)]
pub struct Vector2u {
    pub x: u32,
    pub y: u32
}

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy)]
pub struct Vector2i {
    pub x: i64,
    pub y: i64
}

#[cfg(target_arch = "x86")]
#[derive(Debug, Clone, Copy)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32
}

impl Vector2f{
    #[cfg(target_arch = "x86_64")]
    pub fn new(x: f64, y: f64) -> Vector2f {
        Vector2f{
            x: x,
            y: y
        }
    }
    #[cfg(target_arch = "x86")]
    pub fn new(x: f32, y: f32) -> Vector2f {
        Vector2f{
            x: x,
            y: y
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[cfg(target_arch = "x86")]
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Add for Vector2f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Vector2f {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}


impl Mul for Vector2f {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Vector2f {
    type Output = Self;

    fn div(self, other: Self) -> Self{
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}


impl AddAssign for Vector2f {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vector2f {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Vector2f {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Vector2f {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

//Signed Int

impl Add for Vector2i {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Vector2i {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}


impl Mul for Vector2i {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Vector2i {
    type Output = Self;

    fn div(self, other: Self) -> Self{
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl AddAssign for Vector2i {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vector2i {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Vector2i {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Vector2i {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}



// Unsigned Int

impl Add for Vector2u {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Vector2u {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}


impl Mul for Vector2u {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Vector2u {
    type Output = Self;

    fn div(self, other: Self) -> Self{
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl AddAssign for Vector2u {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vector2u {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Vector2u {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Vector2u {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}
