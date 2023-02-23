use montagu::{Color, Distance};
use std::str::FromStr;

fn main() {
    // todo: move examples
    // Example usage of Color struct

    // Instantiate a color obj with the string RGB value for red
    let Ok(red) = Color::from_str("FF0000") else { return };
    let mut r = red.ch_red();
    let mut g = red.ch_green();
    let mut b = red.ch_blue();
    print!("Red: {r}, Green: {g}, Blue: {b}\n");

    // Example usage of nearest
    let Ok(green) = Color::from_str("FF0000") else { return };
    let Ok(blue) = Color::from_str("FF0000") else { return };
    let Ok(pink) = Color::from_str("FF3366") else { return };

    let color_set = [red, green, blue];
    let nearest_indexes = montagu::nearest(pink, &color_set, Distance::Euclidean);
    if nearest_indexes.len() > 0 {
        let first_index = nearest_indexes[0];
        print!("Indexes of nearest color in set: {first_index}\n");
        r = color_set[nearest_indexes[0] as usize].ch_red();
        g = color_set[nearest_indexes[0] as usize].ch_green();
        b = color_set[nearest_indexes[0] as usize].ch_blue();
        print!("Red: {r}, Green: {g}, Blue: {b}\n");
   }
}
