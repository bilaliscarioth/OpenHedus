use std::ops::*;
use sdl2::pixels::Color;


#[cfg(target_arch = "x86_64" )]
#[derive(Debug, Clone, Copy)]
pub struct Vector4f {
    pub w : f64,
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

#[cfg(target_arch = "x86")]
#[derive(Debug, Clone, Copy)]
pub struct Vector4f {
    pub w : f32,
    pub x : f32,
    pub y : f32,
    pub z : f32
}


#[cfg(target_arch = "x86")]
#[derive(Debug, Clone, Copy)]
pub struct Vector4u {
    pub w : u32,
    pub x : u32,
    pub y : u32,
    pub z : u32
}

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy)]
pub struct Vector4u {
    pub w : u64,
    pub x : u64,
    pub y : u64,
    pub z : u64
}

#[cfg(target_arch = "x86")]
#[derive(Debug, Clone, Copy)]
pub struct Vector4i {
    pub w : i32,
    pub x : i32,
    pub y : i32,
    pub z : i32
}

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy)]
pub struct Vector4i {
    pub w : i64,
    pub x : i64,
    pub y : i64,
    pub z : i64
}

impl Vector4f {
    #[cfg(target_arch = "x86_64" )]
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Vector4f {
        Vector4f{
            w: w,
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86" )]
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Vector4f {
        Vector4f{
            w: w,
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn length(&self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[cfg(target_arch = "x86")]
    pub fn length(&self) -> f32 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Vector4u {
    #[cfg(target_arch = "x86_64" )]
    pub fn new(w: u64, x: u64, y: u64, z: u64) -> Vector4u {
        Vector4u{
            w: w,
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86" )]
    pub fn new(w: u32, x: u64, y: u32, z: u32) -> Vector4u {
        Vector4u{
            w: w,
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn length(&self) -> f64 {
        ((self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt()
    }

    #[cfg(target_arch = "x86")]
    pub fn length(&self) -> f32 {
        ((self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z) as f32).sqrt()
    }

    pub fn to_color(&self) -> Color {
        Color::RGBA(
            self.w as u8,
            self.x as u8,
            self.y as u8,
            self.z as u8
        )
    }
}

impl Vector4i {
    #[cfg(target_arch = "x86_64" )]
    pub fn new(w: i64, x: i64, y: i64, z: i64) -> Vector4i {
        Vector4i{
            w: w,
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86" )]
    pub fn new(w: i32, x: i32, y: i32, z: i32) -> Vector4i {
        Vector4i{
            w: w,
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn length(&self) -> f64 {
        ((self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt()
    }

    #[cfg(target_arch = "x86")]
    pub fn length(&self) -> f32 {
        ((self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z) as f32).sqrt()
    }
}


impl Add for Vector4f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vector4f {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}


impl Mul for Vector4f {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Self {
            w: self.w * other.w,
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div for Vector4f {
    type Output = Self;

    fn div(self, other: Self) -> Self{
        Self {
            w: self.w / other.w,
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}


impl AddAssign for Vector4f {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vector4f {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Vector4f {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Vector4f {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}


//Signed Int

impl Add for Vector4i {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vector4i {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}


impl Mul for Vector4i {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Self {
            w: self.w * other.w,
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div for Vector4i {
    type Output = Self;

    fn div(self, other: Self) -> Self{
        Self {
            w: self.w / other.w,
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}


impl AddAssign for Vector4i {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vector4i {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Vector4i {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Vector4i {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

// Unsigned Int

impl Add for Vector4u {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vector4u {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}


impl Mul for Vector4u {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Self {
            w: self.w * other.w,
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div for Vector4u {
    type Output = Self;

    fn div(self, other: Self) -> Self{
        Self {
            w: self.w / other.w,
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl AddAssign for Vector4u {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vector4u {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Vector4u {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Vector4u {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}
