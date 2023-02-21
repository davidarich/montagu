use std::str::FromStr;

pub enum FromStrError {
    InvalidLength,
    InvalidHexidecimal,
}

#[derive(Copy, Clone, Debug)]
// todo: implement functions for accessing each channel value instead of public struct props
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl FromStr for Color {
    type Err = FromStrError;
    fn from_str(s: &str) ->Result<Color, Self::Err> {
        // shadow the value so it can't be changed outside of this scope
        let s = s;
        // todo: check for # prefix
        if s.len() != 8 {
            return Err(FromStrError::InvalidLength)
        }
        let Ok(x) = hex::decode(s) else { return Err(FromStrError::InvalidHexidecimal)};
        if x.len() != 4 {
            return Err(FromStrError::InvalidLength)
        }
        return Ok(Color {r: x[0], g: x[1], b:x[2], a:x[3] })
    }
}
// defaults to returning black, this could be improved
pub fn nearest(needle: Color, haystack: &[Color]) -> &Color {
    let mut lowest_diff: u32 = 255*4; // initially set as max value (255 x 4 channels)
    let mut return_color: &Color = &Color { r: 0, g: 0, b: 0, a:0 };

    for hay in haystack.iter() {
        // Red
        let new_diff = channel_diff(needle.r, hay.r)
        // Green
        + channel_diff(needle.g, hay.g)
        // Blue
        + channel_diff(needle.b, hay.b)
        // Alpha
        + channel_diff(needle.a, hay.a);

        if (new_diff as u32) < lowest_diff {
            lowest_diff = new_diff as u32;
            return_color = hay;
        }
    }
    return return_color;
}

// returns the difference between 2 color channel values
fn channel_diff(a: u8, b: u8) -> u8 {
    if a == b {
        return 0;
    }
    if a > b {
        return a - b;
    }
    // b > a is implied
    return b - a;
}
