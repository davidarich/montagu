use std::str::FromStr;

pub use self::distance::*;
//pub use self::transform::*;

mod distance;
//mod transform;

// represents 24 bit RGB color with 8 bit alpha channel for transparency
#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    // channel values in decimal form
    pub fn ch_red(&self) -> u8 {
        self.r
    }
    pub fn ch_green(&self) -> u8 {
        self.g
    }
    pub fn ch_blue(&self) -> u8 {
        self.b
    }
    // channel values in hexidecimal string
    pub fn ch_red_hex(&self) -> String {
        hex::encode([self.ch_red()])
    }
    pub fn ch_green_hex(&self) -> String {
        hex::encode([self.ch_green()])
    }
    pub fn ch_blue_hex(&self) -> String {
        hex::encode([self.ch_blue()])
    }
}

impl From<u32> for Color {
    fn from(item: u32) -> Self {
        let bytes: [u8; 4] = item.to_le_bytes();
        Color { r: bytes[0], g: bytes[1], b: bytes[2] }
    }
}

pub enum FromVecError {
    InvalidLength,
}

impl From<Vec<u8>> for Color {
    fn from(item: Vec<u8>) -> Self {
        Color { r: item[0], g: item[1], b: item[2] }
    }
}

pub enum FromStrError {
    InvalidLength,
    InvalidHexidecimal,
}

impl FromStr for Color {
    type Err = FromStrError;
    fn from_str(s: &str) -> Result<Color, Self::Err> {
        // shadow the value so it can't be changed outside of this scope
        let s = s;

        if s.len() < 6 || s.len() > 7 {
            return Err(FromStrError::InvalidLength)
        }
        // only pass the last 6 chars to skip a # prefix
        let Ok(x) = hex::decode(&s[s.len()-6..]) else { return Err(FromStrError::InvalidHexidecimal)};
        if x.len() != 3 {
            return Err(FromStrError::InvalidLength)
        }
        return Ok(Color::from(x))
    }
}
