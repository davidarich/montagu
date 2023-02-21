use montagu::Color;
use std::str::FromStr;

fn main() {
    // Example usage of Color struct

    // Instantiate a color obj with the string RGBA value for red
   let Ok(red) = Color::from_str("FF0000FF") else { return };
   let r = red.r;
   let g = red.g;
   let b = red.b;
   let a = red.a;
   print!("Red: {r}, Green: {g}, Blue: {b}, Alpha: {a}");
}
