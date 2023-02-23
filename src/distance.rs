use crate::Color;

#[derive(Copy, Clone)]
pub enum Distance {
    Euclidean,
    HumanApprox,
    RedMean
}

// returns index(es) of the nearest color(s) within the set
pub fn nearest(color: Color, color_set: &[Color], method: Distance) -> Vec<usize> {
    let mut lowest_diff: u64 = u64::MAX; // initially set as max value
    let mut indexes:Vec<usize> = Vec::new();
    let mut index: usize = 0;

    for color_set_item in color_set.iter() {
        index += 1;
        let new_diff = distance(&color, color_set_item, &method);
        if new_diff < lowest_diff {
            lowest_diff = new_diff;
            indexes = vec![index]
        }
        if new_diff == lowest_diff {
            indexes.push(index)
        }
    }
    return indexes;
}

// calculates distance/difference between 2 colors using the supplied method
pub fn distance(a: &Color, b: &Color, method: &Distance) -> u64 {

    match *method {
        Distance::Euclidean => return dist_euclidean(a, b),
        Distance::HumanApprox => return dist_human(a, b),
        Distance::RedMean => return dist_red_mean(a, b)
    }
}

// calculates euclidean distance
pub fn dist_euclidean(a: &Color, b: &Color) -> u64 {
    return 
          (b.ch_red()   as i16 - a.ch_red()   as i16) as u64 ^ 2
        + (b.ch_green() as i16 - a.ch_green() as i16) as u64 ^ 2
        + (b.ch_blue()  as i16 - a.ch_blue()  as i16) as u64 ^ 2;
}

// calculates distance weighted to approximate human eyesight
pub fn dist_human(a: &Color, b: &Color) -> u64 {
    // check red difference to determine weighting
    let red_avg = (b.ch_red() + a.ch_red()) / 2;

    let mut red_weight = 2;
    let mut blue_weight = 2;
    if red_avg < 128 {
        blue_weight = 3;
    } else {
        red_weight = 3;
    }
    
    return 
          (((b.ch_red()   as i16 - a.ch_red()   as i16) as u64 ^ 2) * red_weight)
        + (((b.ch_green() as i16 - a.ch_green() as i16) as u64 ^ 2) * 4) // green is always 4
        + (((b.ch_blue()  as i16 - a.ch_blue()  as i16) as u64 ^ 2) * blue_weight);
}

// calculates distance using a mixture of euclidean & human 
pub fn dist_red_mean(a: &Color, b: &Color) -> u64 {
    let _a = a;
    let _b = b;
    unimplemented!("todo");
}
