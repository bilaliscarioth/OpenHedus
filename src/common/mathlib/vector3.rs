use std::ops::*;
use sdl2::pixels::Color;

#[cfg(target_arch = "x86_64" )]
#[derive(Debug, Clone, Copy)]
pub struct Vector3f {
    pub x : f64,
    pub y : f64,
    pub z : f64
}

#[cfg(target_arch = "x86")]
#[derive(Debug, Clone, Copy)]
pub struct Vector3f {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

#[cfg(target_arch = "x86")]
#[derive(Debug, Clone, Copy)]
pub struct Vector3u{
    pub x: u32,
    pub y: u32,
    pub z: u32
}

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy)]
pub struct Vector3u{
    pub x: u64,
    pub y: u64,
    pub z: u64
}

#[cfg(target_arch = "x86")]
#[derive(Debug, Clone, Copy)]
pub struct Vector3i{
    pub x: i32,
    pub y: i32,
    pub z: i32
}

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy)]
pub struct Vector3i{
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl Vector3f{
    #[cfg(target_arch = "x86_64")]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3f {
        Vector3f{
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86")]
    pub fn new(w: f32, x: f32, y: f32) -> Vector3f {
        Vector3f{
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[cfg(target_arch = "x86")]
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Vector3i{
    #[cfg(target_arch = "x86_64")]
    pub fn new(x: i64, y: i64, z: i64) -> Vector3i{
        Vector3i{
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86")]
    pub fn new(x: i32, x: i32, y: i32) -> Vector3i {
        Vector3i{
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn length(&self) -> f64 {
        ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt()
    }

    #[cfg(target_arch = "x86")]
    pub fn length(&self) -> f32 {
        ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt()
    }
}

impl Vector3u{
    #[cfg(target_arch = "x86_64")]
    pub fn new(x: u64, y: u64, z: u64) -> Vector3u {
        Vector3u{
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86")]
    pub fn new(w: u32, x: u32, y: u32) -> Vector3u {
        Vector3u{
            x: x,
            y: y,
            z: z
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn length(&self) -> f64 {
        ((self.x * self.x + self.y * self.y + self.z * self.z) as f64 ).sqrt()
    }

    #[cfg(target_arch = "x86")]
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn to_color(&self) -> Color {
        Color::RGB(
            self.x as u8,
            self.y as u8,
            self.z as u8
        )
    }
}

impl Add for Vector3f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vector3f {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}


impl Mul for Vector3f {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div for Vector3f {
    type Output = Self;

    fn div(self, other: Self) -> Self{
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}


impl AddAssign for Vector3f {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vector3f {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Vector3f {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Vector3f {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}


//Signed Int

impl Add for Vector3i {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vector3i {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}


impl Mul for Vector3i {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div for Vector3i {
    type Output = Self;

    fn div(self, other: Self) -> Self{
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}


impl AddAssign for Vector3i {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vector3i {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Vector3i {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Vector3i {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

// Unsigned Int

impl Add for Vector3u {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vector3u {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z}

    }
}


impl Mul for Vector3u {
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div for Vector3u {
    type Output = Self;

    fn div(self, other: Self) -> Self{
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl AddAssign for Vector3u {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vector3u {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Vector3u {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Vector3u {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}
